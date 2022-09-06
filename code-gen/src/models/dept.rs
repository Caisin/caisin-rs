use caisin_macros::CreateTable;
use rbatis::rbdc::datetime::DateTime;
#[derive(CreateTable)]
#[caisin(comment = "部门表", haha, tbName = "sys_dept")]
pub struct Dept {
    #[caisin(comment = "部门ID", pk)]
    pub dept_id: String,
    #[caisin(comment = "分区", pk)]
    pub parent_id: String,
    #[caisin(comment = "部门名称", index, size = 255)]
    pub dept_name: String,
    pub order_num: Option<i32>,
    pub leader: Option<String>,
    pub phone: Option<String>,
    #[caisin(comment = "邮箱", index, size = 255)]
    pub email: Option<String>,
    pub status: String,
    pub created_by: String,
    pub updated_by: Option<String>,
    pub created_at: DateTime,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}
