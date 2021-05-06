use std::fs::OpenOptions;
use std::fs::File;
use std::io::SeekFrom;

pub struct DiskManager {
    // ヒープファイルのファイルディスクリプタ
    heap_file: File,
    // 採番するページIDを決めるカウンタ
    next_page_id: u64, // 符号なし64bit整数型
}

// page_idはほぼ整数値だが、page_id同士の演算など無意味な処理を静的型チェックで検出するためあえて独自定義型を使う
// Rustではnew typeパターンと言う※以下はnewtypeイデオム
pub struct PageId(pub u64);

// 引数の&mut selfはレシーバ = this
// レシーバが存在するメソッドはインスタンスメソッド
// レシーバが存在しないメソッドはスタティックメソッド
impl DiskManager {
    const PAGE_SIZE:u64 = 4096;

    // コンストラクタ
    // io::ResultはI/O関連の操作の結果を表す型
    // io::Result<Self>は自分自身=DiskMagagerを返す
    pub fn new(heap_file:File) -> io::Result<Self> {
        // ファイルサイズ取得
        let heap_file_size = heap_file.metadata()?.len();
        let next_page_id = heap_file_size / PAGE_SIZE as u64;
        // Self{heap_file, next_page_id}はDiskManagerのインスタンス
        // 関数の最後に評価した式が戻り値 (return をあえて書かない)
        // この場合、最後に;を書くとエラーになる
        Ok(Self {
            heap_file,
            next_page_id,
        })
    }

    // ファイルパスを指定して開く
    pub fn open(heap_file_path:impl AsRef<Path>) -> io::Result<Self> {
        // ファイルを開く際のオプションを設定してファイルを開く
        // 一般的に、OpenOptionsを使用する場合は、最初にOpenOptions :: newを呼び出し、次にメソッドへの呼び出しをチェーンして各オプションを設定し、次にOpenOptions :: openを呼び出して、開こうとしているファイルのパスを渡します。これにより、さらに操作できるファイルを含むio :: Resultが得られます。
        // https://doc.rust-lang.org/std/fs/struct.OpenOptions.html
        let heap_file = OpenOptions::new()
            .read(true) // 読み込み可能
            .write(true) // 書き込み可能
            .create(true) // ファイルが無い場合は生成
            // openメソッドはio::Resutl<File>を返す
            .open(heap_file_path)?; // ?はエラーが返ってきたらそこで早期returnする、という意味
        self::new(heap_file)
    }

    // 新しいページIDを採番する
    pub fn allocate_page(&mut self) -> PageId {
        // 「self」はthis的な意味合い
        let page_id = self.next_page_id;
        self.next_page_id += 1;
        // PageId型を返却(returnを省略※「;」は書かない)
        PageId(page_id)
    }

    // ページのデータを読み出す
    pub fn read_page_data(&mut self, page_id:PageId, data:&mut [u8]) -> io::Result<()> { // 戻り値型はvoid
        let offset = PAGE_SIZE as u64 * page_id.to_u64();
        self.heap_file.seek(SeekFrom::Start(offset))?;
        // 読み出したデータをdata引数に書き込む
        self.heap_file.read_exact(data)
    }

    // データをページに書き出す
    pub fn write_page_data(&mut self, page_id:PageId, data:&[u8]) -> io::Result<()> {
        // オブセットを計算
        // 現在のpage_idにページサイズをかけることでファイル内のオフセットが分かる
        let offset = PAGE_SIZE as u64 * page_id.to_u64();
        // ページ先頭へシーク
        // SeekFrom::Start(offset)は「ファイルの先頭から数えてoffsetバイト目」と言う意味
        self.heap_file.seek(SeekFrom::Start(offset))?;
        // データを書き込む
        self.heap_file.write_all(data);
    }
}