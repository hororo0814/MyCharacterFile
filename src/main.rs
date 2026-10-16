mod domain;
mod repository;
mod ui;
use crate::repository::character_file::data_file_path;
use crate::repository::character_file::load_text;
use crate::repository::character_store::CharacterStore;
use ui::app::CharacterApp;

fn main() -> eframe::Result {
    let file_path = match data_file_path() {
        Ok(path) => path,
        Err(error) => {
            eprintln!("保存先の取得に失敗しました: {error}");
            return Err(eframe::Error::AppCreation(Box::new(error)));
        }
    };
    //ファイルが存在するかを確認
    let new_store = if file_path.exists() {
        println!("ファイルの存在を確認");
        match load_text() {
            //ファイルが読み込めれば既存のStoreを使う
            Ok(store) => store,
            //読み込めなければエラーを出力し，空のStoreを用意
            Err(error) => {
                eprintln!("読み込みエラー: {error}");
                return Err(eframe::Error::AppCreation(Box::new(error)));
            }
        }
    } else {
        CharacterStore::new()
    };

    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "キャラクター図鑑",
        options,
        Box::new(|cc| Ok(Box::new(CharacterApp::new(cc, new_store)))),
    )
}
