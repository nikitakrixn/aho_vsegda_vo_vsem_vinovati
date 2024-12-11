use diesel_async::RunQueryDsl;
use backend::{
    db,
    models::{
        departments::NewDepartment,
        positions::NewPosition,
    },
    schema::{departments, positions},
};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let pool = db::create_pool(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"));
    let mut conn = pool.get().await.expect("Failed to get DB connection");

    // Создаем отделы
    let departments = vec![
        NewDepartment { name: "Управление".into() },
        NewDepartment { name: "Отдел правовой работы".into() },
        NewDepartment { name: "Отдел отчетности и финансирования".into() },
        NewDepartment { name: "Сметный отдел".into() },
        NewDepartment { name: "Производственный отдел".into() },
        NewDepartment { name: "Технический отдел".into() },
        NewDepartment { name: "Отдел комплектации и закупок".into() },
        NewDepartment { name: "Административно-хозяйственный отдел".into() },
        NewDepartment { name: "Отдел земельных отношений и государственной регистрации прав".into() },
        NewDepartment { name: "Планово-экономический отдел".into() },
        NewDepartment { name: "Отдел специальных работ".into() },
    ];

    for department in departments {
        diesel::insert_into(departments::table)
            .values(&department)
            .execute(&mut conn)
            .await
            .expect("Error seeding department");
    }

    // Создаем должности
    let positions = vec![
        NewPosition { name: "Директор".into() },
        NewPosition { name: "Заместитель директора (главный инженер)".into() },
        NewPosition { name: "Заместитель директора по финансово-экономическим вопросам".into() },
        NewPosition { name: "Заместитель директора по производству".into() },
        NewPosition { name: "Начальник отдела".into() },
        NewPosition { name: "Заместитель начальника отдела".into() },
        NewPosition { name: "Главный специалист".into() },
        NewPosition { name: "Ведущий инженер по охране труда".into() },
        NewPosition { name: "Ведущий специалист".into() },
        NewPosition { name: "Ведущий документовед".into() },
        NewPosition { name: "Водитель автомобиля".into() },
        NewPosition { name: "Уборщик служебных помещений".into() },
    ];

    for position in positions {
        diesel::insert_into(positions::table)
            .values(&position)
            .execute(&mut conn)
            .await
            .expect("Error seeding position");
    }

    println!("Seeding completed successfully!");
}