use std::collections::HashMap;
// 型エイリアス
// 配列の型：[要素の型; 配列の長さ]
pub type Page = [u8; PAGE_SIZE];
pub struct BufferId(pub u64);

pub struct Buffer {
    pub page_id: PageId,
    // RefCell<T>：複雑なデータ構造のデータ競合をコンパイル時ではなく実行時に検査する
    pub page: RefCell<Page>,
    // Cell<T>：読み取り専用の値の中に書き込み可能な値を作る
    pub is_dirty: Cell<bool>,
} 

pub struct Frame {
    usage_count: u64,
    // Rc<T>：対象データへの参照の数をカウントする
    // Rc<T>は複数の所有権を可能にする(reference counting)
    // bufferを様々な場所に貸し出している(様々な場所から参照されている可能性がある)ため誤って破棄しない様に参照カウントを保持しておく
    // 
    // ヒープにプログラムの複数箇所で読む何らかのデータを確保したいけれど、
    // コンパイル時にはどの部分が最後にデータを使用し終わるか決定できない時にRc<T>型を使用する
    // Rc<T>はシングルスレッドでのみ利用される
    buffer: Rc<Buffer>,
}

pub struct BufferPool {
    // Vec<T>：連続した拡張可能(サイズ変更可能)な配列型(Frameの配列)
    // Vec<T>はusize型のlenプロパティ(配列の長さ)を持つ
    // usize型はハードウェアにとって都合の良いサイズの整数型であり配列のインデックスに使われる
    // Vecのインデックスは整数
    buffers: Vec<Frame>,
    next_victim_id: BufferId,
}

pub struct BufferPoolManager {
    disk:DiskManager,
    pool:BufferPool,
    // HashMapはKey/Valueで、KeyはEq又はHashトレイトを保持する値ならなんでもOK
    page_table: HashMap<PageId, BufferId>,
}

impl BufferPool {
    // &mut selfが引数にあるのでレシーバ
    // 破棄するバッファを決めてBufferIdを返す
    fn evict(&mut self) -> Option<BufferId> {
        let pool_size = self.size();
        let mut consecutive_pinned = 0;
        let victim_id = loop {
            let next_victim_id = self.next_victim_id;
            // BufferPoolのnext_victim_id番目の要素をframeに代入
            let frame = &mut self[next_victim_id];
            // バッファの利用回数が0の場合
            if frame.usage_count == 0 {
                break self.next_victim_id;
            }

            if Rc::get_mut(&mut frame.buffer).is_some() {
                frame.usage_count -= 1;
                consecutive_pinned = 0;
            } else {
                consecutive_pinned += 1;
                if consecutive_pinned >= pool_size {
                    return None;
                }
            }
        }
    }
}