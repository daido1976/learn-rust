use crate::disk::{DiskManager, PageId};
use std::{
    cell::RefCell,
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
    pub is_dirty: bool,
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
    page_table: HashMap<String, Buffer>,
}
