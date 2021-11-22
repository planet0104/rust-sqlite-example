use anyhow::Result;
use async_std::task;
use sea_orm::{ConnectOptions, Database, EntityTrait};
mod user;
use cli_table::{format::Justify, print_stdout, Cell, CellStruct, Style, Table};
use user::Entity as User;

fn main() -> Result<()> {
    task::block_on(query())?;
    Ok(())
}

async fn query() -> Result<()> {
    let opt = ConnectOptions::new("sqlite://databases/users".to_owned());

    let db = Database::connect(opt).await?;

    // 查询
    let users = User::find().all(&db).await?;

    // 删除
    // let u: user::ActiveModel = users[3].clone().into();
    // User::delete(u).exec(&db).await?;

    let table = users
        .into_iter()
        .map(|u| {
            vec![
                u.uid.cell(),
                u.first_name.clone().cell().justify(Justify::Right),
                u.created().cell(),
            ]
        })
        .collect::<Vec<Vec<CellStruct>>>()
        .table()
        .title(vec![
            "UID".cell().bold(true),
            "姓名".cell().bold(true),
            "创建时间".cell().bold(true),
        ])
        .bold(true);

    print_stdout(table)?;

    Ok(())
}
