mod domain;
mod repository;
mod ui;
use crate::domain::ability::{Ability, AbilityType};
use crate::repository::character_file::load_text;
use crate::repository::character_file::save_store;
use crate::repository::character_store::{CharacterStore, CharacterStoreErr};
use domain::character::{Character, CharacterId, Gender, Race};
use std::path::Path;
use ui::app::CharacterApp;

fn create_sample_store() -> Result<CharacterStore, CharacterStoreErr> {
    //空のストアを作る
    let mut store = CharacterStore::new();
    //サンプルキャラクターを作る
    let ability1 = Ability::new("フレア".to_string(), 50, AbilityType::Attack, 20);
    let character = Character::new(
        CharacterId(1),
        "亜紀".to_string(),
        Some(17),
        Race::Human,
        Gender::Male,
        vec![ability1],
        "由紀とともに旅をする".to_string(),
    );
    //store.add(character) で登録する
    store.add(character)?;
    //save_store(&store) で保存する
    save_store(&store)?;
    Ok(store)
}

fn main() -> eframe::Result {
    println!("作業ディレクトリ: {:?}", std::env::current_dir());
    println!("読み込み対象: {:?}", std::fs::canonicalize("file.txt"));
    //ファイルが存在するかを確認
    let new_store = if Path::new("file.txt").exists() {
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
        //ファイルが存在しない場合だけサンプルデータを作る
        match create_sample_store() {
            Ok(store) => store,
            Err(error) => {
                eprintln!("サンプルデータの作成エラー: {error}");
                return Err(eframe::Error::AppCreation(Box::new(error)));
            }
        }
    };

    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "ほろろのキャラクター大辞典",
        options,
        Box::new(|cc| Ok(Box::new(CharacterApp::new(cc, new_store)))),
    )
}
