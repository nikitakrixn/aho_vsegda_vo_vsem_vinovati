use chrono::Local;
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

    let employees_data = vec![
        ("Валерий", "Зубарев", Some("Александрович"), 5, "zubarevva@oosz.ru"),
        ("Никита", "Найдёнов", Some("Андреевич"), 9, "naydenovna@oosz.ru"),
    ];

    for (first_name, last_name, middle_name, position_id, email) in employees_data {
        let employee = backend::models::employees::NewEmployee {
            first_name: first_name.into(),
            last_name: last_name.into(),
            middle_name: middle_name.map(|s| s.into()),
            department_id: Some(8),
            position_id: Some(position_id),
            hire_date: Local::now().naive_local().date(),
            ad_login: None,
            email: Some(email.into()),
            status: "работает".into(),
            phone: None,
        };

        diesel::insert_into(backend::schema::employees::table)
            .values(&employee)
            .execute(&mut conn)
            .await
            .expect("Error seeding employee");
    }

    println!("Seeding completed successfully!");
}