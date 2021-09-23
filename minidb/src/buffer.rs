#![allow(dead_code)]
use crate::disk::{DiskManager, PageId};
use anyhow::Result;
use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    ops::{Index, IndexMut},
    rc::Rc,
};

use crate::disk::PAGE_SIZE;

pub type Page = [u8; PAGE_SIZE];

#[derive(Clone, Copy)]
pub struct BufferId(usize);

pub struct Buffer {
    pub page_id: PageId,
    pub page: RefCell<Page>,
    pub is_dirty: Cell<bool>,
}

pub struct Frame {
    usage_count: u64,
    buffer: Rc<Buffer>,
}

pub struct BufferPool {
    buffers: Vec<Frame>,
    next_victim_id: BufferId,
}

impl BufferPool {
    fn size(&self) -> usize {
        self.buffers.len()
    }

    // バッファプール内のすべてのバッファを巡回しながら、捨てるバッファを探す（Clock-sweep アルゴリズム）
    fn evict(&mut self) -> Option<BufferId> {
        let pool_size = self.size();
        let mut consecutive_pinned = 0;

        // 1. バッファプール内のすべてのバッファを巡回するループ
        let victim_id = loop {
            let next_victim_id = self.next_victim_id;
            let frame = &mut self[next_victim_id];
            if frame.usage_count == 0 {
                // 2. バッファが利用されていない
                break next_victim_id;
            }
            if Rc::get_mut(&mut frame.buffer).is_some() {
                // 3. 貸し出し中ではないのでデクリメントする
                frame.usage_count -= 1;
                consecutive_pinned = 0;
            } else {
                // 4. 貸し出し中なのでインクリメントする
                consecutive_pinned += 1;
                if consecutive_pinned >= pool_size {
                    return None;
                }
            }
            self.next_victim_id = self.increment_id(self.next_victim_id);
        };
        Some(victim_id)
    }

    fn increment_id(&self, buffer_id: BufferId) -> BufferId {
        BufferId((buffer_id.0 + 1) % self.size())
    }
}

// BufferPool で `self[next_victim_id]` のようなアクセスができるのは以下のおかげ。
// See. https://keens.github.io/blog/2019/10/31/rustdekouzoutainihairetsunoyounaindekkusuakusesu/
impl Index<BufferId> for BufferPool {
    type Output = Frame;

    fn index(&self, index: BufferId) -> &Self::Output {
        &self.buffers[index.0]
    }
}

impl IndexMut<BufferId> for BufferPool {
    fn index_mut(&mut self, index: BufferId) -> &mut Self::Output {
        &mut self.buffers[index.0]
    }
}

pub struct BufferPoolManager {
    disk: DiskManager,
    pool: BufferPool,
    page_table: HashMap<PageId, BufferId>,
}

impl BufferPoolManager {
    fn fetch_page(&mut self, page_id: PageId) -> Result<Rc<Buffer>> {
        // ページがバッファプールにある場合
        if let Some(&buffer_id) = self.page_table.get(&page_id) {
            let frame = &mut self.pool[buffer_id];
            frame.usage_count += 1;
            return Ok(Rc::clone(&frame.buffer));
        }
        // ページがバッファプールにない場合
        // 1. これから読み込むページの格納するバッファ（捨てるバッファ）を探す
        let buffer_id = self
            .pool
            .evict()
            .ok_or_else(|| anyhow::anyhow!("BufferPool is full"))?;
        let frame = &mut self.pool[buffer_id];
        let evict_page_id = frame.buffer.page_id;
        {
            let buffer = Rc::get_mut(&mut frame.buffer).unwrap();
            // 2. is_dirty が true なら、バッファのページをディスクに書き込む
            if buffer.is_dirty.get() {
                self.disk
                    .write_page_data(evict_page_id, buffer.page.get_mut())?;
            }
            buffer.page_id = page_id;
            buffer.is_dirty.set(false);
            // 3. ページを読み出す
            self.disk.read_page_data(page_id, buffer.page.get_mut())?;
            frame.usage_count = 1;
        }
        let page = Rc::clone(&frame.buffer);
        // 4. バッファに入ってるページが入れ替わったので、ページテーブルを更新する
        self.page_table.remove(&evict_page_id);
        self.page_table.insert(page_id, buffer_id);
        Ok(page)
    }
}
