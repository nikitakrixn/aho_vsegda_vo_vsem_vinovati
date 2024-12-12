// use ldap3::{LdapConnAsync, SearchEntry, Scope, LdapError};
// use ldap3::result::Result;
// use crate::services::ldap_service::{LdapConfig, connect_to_ldap};
// use crate::models::employees::Employee;
// use crate::{db::DbPool, log_error};
// use diesel_async::RunQueryDsl;
// use diesel::ExpressionMethods;
// use std::collections::HashSet;

// #[derive(Debug, Clone)]
// pub struct NewAdUser {
//     pub username: String,
//     pub first_name: String,
//     pub last_name: String,
//     pub password: String,
//     pub groups: Option<Vec<String>>, // Добавляем опциональное поле для групп
//     pub email: Option<String>, // Добавляем опциональное поле для email
// }

// // Функция создания пользователя в AD
// pub async fn create_user_in_ad(
//     ldap: &mut LdapConnAsync,
//     config: &LdapConfig,
//     user_data: &NewAdUser,
// ) -> Result<(), LdapError> {
//     let user_dn = format!("CN={},{}", user_data.username, config.base_dn);

//     let mut add_op = ldap.add(&user_dn);
//     add_op.append("objectClass", vec!["top", "person", "organizationalPerson", "user"]);
//     add_op.append("cn", vec![&user_data.username]);
//     add_op.append("sAMAccountName", vec![&user_data.username]);
//     add_op.append("givenName", vec![&user_data.first_name]);
//     add_op.append("sn", vec![&user_data.last_name]);
//     add_op.append("userPassword", vec![&user_data.password]);
//     add_op.append("displayName", vec![&format!("{} {}", user_data.last_name, user_data.first_name)]);
//         // Проверка наличия email и добавление атрибута mail
//         if let Some(email) = &user_data.email {
//             add_op.append("mail", vec![email]);
//         }

//     add_op.execute().await?.success()?;

//     // Добавление в группы, если они указаны
//     if let Some(groups) = &user_data.groups {
//         for group_cn in groups {
//             add_user_to_group(ldap, config, &user_data.username, group_cn).await?;
//         }
//     }

//     Ok(())
// }

// // Функция добавления пользователя в группу AD
// pub async fn add_user_to_group(
//     ldap: &mut LdapConnAsync,
//     config: &LdapConfig,
//     username: &str,
//     group_cn: &str,
// ) -> Result<(), LdapError> {
//     let group_dn = format!("CN={},{}", group_cn, config.base_dn);
//     let user_dn = format!("CN={},{}", username, config.base_dn);

//     let mut modify_op = ldap.modify(&group_dn);
//     modify_op.add("member", &user_dn);

//     modify_op.execute().await?.success()?;

//     Ok(())
// }

// //Поиск пользователей
// pub async fn search_users_in_ad(
//     ldap: &mut LdapConnAsync,
//     config: &LdapConfig,
//     filter: &str,
// ) -> Result<Vec<SearchEntry>, LdapError> {
//     let (rs, _res) = ldap
//         .search(
//             &config.base_dn,
//             Scope::Subtree,
//             filter,
//             vec!["cn", "sAMAccountName", "givenName", "sn", "mail", "memberOf"],
//         )
//         .await?
//         .success()?;

//         Ok(rs.into_iter().map(SearchEntry::construct).collect())
// }
// //Получение групп пользователя
// pub async fn get_user_groups(
//     ldap: &mut LdapConnAsync,
//     config: &LdapConfig,
//     username: &str,
// ) -> Result<HashSet<String>, LdapError> {
//     let user_dn = format!("CN={},{}", username, config.base_dn);
//     let filter = format!("(memberOf=*)");
//     let (rs, _res) = ldap
//         .search(&user_dn, Scope::Base, &filter, vec!["memberOf"])
//         .await?
//         .success()?;

//         let mut group_set = HashSet::new();

//         for entry in rs {
//             if let Some(member_of) = entry.attrs.get("memberOf") {
//                 for group_dn in member_of {
//                     let group_cn = extract_cn_from_dn(group_dn);
//                     group_set.insert(group_cn);
//                 }
//             }
//         }
//         Ok(group_set)
// }
// //Добавление в группу
// pub async fn add_employee_to_group(
//     State(pool): State<DbPool>,
//     State(ldap_config): State<LdapConfig>,
//     State(mut ldap): State<LdapConnectionState>,
//     employee_id: i32,
//     group_cn: &str,
// ) -> Result<(), String> {
//     // Получение данных сотрудника из БД
//     let employee = get_employee_details(&pool, employee_id).await.map_err(|e| {
//         log_error!("Error getting employee details: {}", e);
//         format!("Failed to get employee details: {}", e)
//     })?;

//     // Проверка наличия логина AD у сотрудника
//     let ad_login = employee.ad_login.ok_or_else(|| {
//         log_error!("Employee with ID {} does not have an AD login", employee_id);
//         format!("Employee with ID {} does not have an AD login", employee_id)
//     })?;

//     // Добавление пользователя в группу
//     add_user_to_group(&mut ldap, &ldap_config, &ad_login, group_cn)
//         .await
//         .map_err(|e| {
//             log_error!("Error adding user to group: {}", e);
//             format!("Failed to add user to group: {}", e)
//         })?;

//     Ok(())
// }
// //Удаление из группы
// pub async fn remove_employee_from_group(
//     State(pool): State<DbPool>,
//     State(ldap_config): State<LdapConfig>,
//     State(mut ldap): State<LdapConnectionState>,
//     employee_id: i32,
//     group_cn: &str,
// ) -> Result<(), String> {
//     // Получение данных сотрудника из БД
//     let employee = get_employee_details(&pool, employee_id).await.map_err(|e| {
//         log_error!("Error getting employee details: {}", e);
//         format!("Failed to get employee details: {}", e)
//     })?;

//     // Проверка наличия логина AD у сотрудника
//     let ad_login = employee.ad_login.ok_or_else(|| {
//         log_error!("Employee with ID {} does not have an AD login", employee_id);
//         format!("Employee with ID {} does not have an AD login", employee_id)
//     })?;

//     // Удаление пользователя из группы
//     remove_user_from_group(&mut ldap, &ldap_config, &ad_login, group_cn)
//         .await
//         .map_err(|e| {
//             log_error!("Error removing user from group: {}", e);
//             format!("Failed to remove user from group: {}", e)
//         })?;

//     Ok(())
// }

// //Вспомогательная функция удаления из группы
// async fn remove_user_from_group(
//     ldap: &mut LdapConnAsync,
//     config: &LdapConfig,
//     username: &str,
//     group_cn: &str,
// ) -> Result<(), LdapError> {
//     let group_dn = format!("CN={},{}", group_cn, config.base_dn);
//     let user_dn = format!("CN={},{}", username, config.base_dn);

//     let mut modify_op = ldap.modify(&group_dn);
//     modify_op.delete("member", &user_dn);

//     modify_op.execute().await?.success()?;

//     Ok(())
// }
// //Вспомогательная функция для парсинга
// fn extract_cn_from_dn(dn: &str) -> String {
//     dn.split(',')
//         .next()
//         .and_then(|part| part.strip_prefix("CN="))
//         .unwrap_or("")
//         .to_string()
// }

// //Создание пользователя
// pub async fn create_ad_account_and_update_employee(
//     pool: &DbPool,
//     ldap_config: &LdapConfig,
//     mut ldap: LdapConnAsync,
//     employee_id: i32,
//     user_data: &NewAdUser,
// ) -> Result<(), String> {
// // 1. Создание учетной записи в AD
//     create_user_in_ad(&mut ldap, ldap_config, user_data)
//         .await
//         .map_err(|e| {
//             log_error!("Error creating user in AD: {}", e);
//             format!("Failed to create user in AD: {}", e)
//         })?;

// // 2. Обновление записи сотрудника в базе данных
//     let mut conn = pool.get().await.map_err(|e| {
//         log_error!("Database connection error: {}", e);
//         format!("Database connection error: {}", e)
//     })?;

//     diesel::update(crate::schema::employees::table.find(employee_id))
//         .set((crate::schema::employees::ad_login.eq(&user_data.username),
//               crate::schema::employees::email.eq(&user_data.email)))
//         .execute(&mut conn)
//         .await
//         .map_err(|e| {
//             log_error!("Failed to update employee in database: {}", e);
//             format!("Failed to update employee in database: {}", e)
//         })?;

//     Ok(())
// }
// //Функция генерации имени пользователя
// pub fn generate_username(first_name: &str, last_name: &str) -> String {
// // Транслитерация с русского на английский
//     let transliterated_first_name = match first_name.chars().next() {
//         Some('А') | Some('а') => "a",
//         Some('Б') | Some('б') => "b",
//         Some('В') | Some('в') => "v",
//         Some('Г') | Some('г') => "g",
//         Some('Д') | Some('д') => "d",
//         Some('Е') | Some('е') | Some('Ё') | Some('ё') => "e",
//         Some('Ж') | Some('ж') => "zh",
//         Some('З') | Some('з') => "z",
//         Some('И') | Some('и') => "i",
//         Some('Й') | Some('й') => "y",
//         Some('К') | Some('к') => "k",
//         Some('Л') | Some('л') => "l",
//         Some('М') | Some('м') => "m",
//         Some('Н') | Some('н') => "n",
//         Some('О') | Some('о') => "o",
//         Some('П') | Some('п') => "p",
//         Some('Р') | Some('р') => "r",
//         Some('С') | Some('с') => "s",
//         Some('Т') | Some('т') => "t",
//         Some('У') | Some('у') => "u",
//         Some('Ф') | Some('ф') => "f",
//         Some('Х') | Some('х') => "h",
//         Some('Ц') | Some('ц') => "c",
//         Some('Ч') | Some('ч') => "ch",
//         Some('Ш') | Some('ш') => "sh",
//         Some('Щ') | Some('щ') => "shch",
//         Some('Ъ') | Some('ъ') => "",
//         Some('Ы') | Some('ы') => "y",
//         Some('Ь') | Some('ь') => "",
//         Some('Э') | Some('э') => "e",
//         Some('Ю') | Some('ю') => "yu",
//         Some('Я') | Some('я') => "ya",
//         _ => "",
//     };

//     let transliterated_last_name = last_name
//         .chars()
//         .map(|c| match c {
//             'А' | 'а' => 'a',
//             'Б' | 'б' => 'b',
//             'В' | 'в' => 'v',
//             'Г' | 'г' => 'g',
//             'Д' | 'д' => 'd',
//             'Е' | 'е' | 'Ё' | 'ё' => 'e',
//             'Ж' | 'ж' => "zh",
//             'З' | 'з' => 'z',
//             'И' | 'и' => 'i',
//             'Й' | 'й' => 'y',
//             'К' | 'к' => 'k',
//             'Л' | 'л' => 'l',
//             'М' | 'м' => 'm',
//             'Н' | 'н' => 'n',
//             'О' | 'о' => 'o',
//             'П' | 'п' => 'p',
//             'Р' | 'р' => 'r',
//             'С' | 'с' => 's',
//             'Т' | 'т' => 't',
//             'У' | 'у' => 'u',
//             'Ф' | 'ф' => 'f',
//             'Х' | 'х' => 'h',
//             'Ц' | 'ц' => 'c',
//             'Ч' | 'ч' => "ch",
//             'Ш' | 'ш' => "sh",
//             'Щ' | 'щ' => "shch",
//             'Ъ' | 'ъ' => "",
//             'Ы' | 'ы' => 'y',
//             'Ь' | 'ь' => "",
//             'Э' | 'э' => 'e',
//             'Ю' | 'ю' => "yu",
//             'Я' | 'я' => "ya",
//             _ => c.to_ascii_lowercase(),
//         })
//         .collect::<String>();

//     format!("{}{}", transliterated_first_name, transliterated_last_name)
// }
// //Функция генерации пароля
// pub fn generate_password() -> String {
// // TODO: Реализовать более надежную генерацию пароля
//     "zaq1XSW2".to_string()
// }
// async fn get_employee_details(pool: &DbPool, employee_id: i32) -> Result<Employee, String> {
//     use crate::schema::employees::dsl::*;

//     let mut conn = pool.get().await.map_err(|e| {
//         log_error!("Database connection error: {}", e);
//         format!("Database connection error: {}", e)
//     })?;

//     employees
//         .filter(id.eq(employee_id))
//         .first::<Employee>(&mut conn)
//         .await
//         .map_err(|e| {
//             log_error!("Failed to get employee details: {}", e);
//             format!("Failed to get employee details: {}", e)
//         })
// }