use std::fmt::{Display, Formatter};

pub struct DataSet<'a> { 
    pub is_create: bool, // create or update
    pub fields: Vec<&'a str>, // fields
    pub values: Vec<String>, // values
}

impl<'a> DataSet<'a> { 

    /// 生成數據
    pub fn create() -> Self { 
        Self { 
            is_create: true,
            fields: vec![],
            values: vec![],
        }
    }
    
    /// 更新數據
    pub fn update() -> Self { 
        Self { 
            is_create: false,
            fields: vec![],
            values: vec![],
        }
    }

    /// 清空
    pub fn clear(&mut self) -> &mut Self { 
        self.is_create = true;
        self.fields = vec![];
        self.values = vec![];
        self
    }

    /// 設置字段值
    pub fn set<T: ToString>(&mut self, field: &'a str, value: &T) -> &mut Self { 
        if field == "id" { 
            self.is_create = true;
        }
        self.fields.push(field);
        let value_string = value.to_string();
        let real_value = value_string.replace("'", "\'");
        self.values.push(real_value);
        self
    }

    /// 生成SQL語句
    pub fn build(&self) -> String { 
        if self.is_create { 
            return self.build_create();
        }
       self.build_update()
    }

    /// Creating sql
    fn build_create(&self) -> String { 
        let mut sql = String::from("(");
        sql.push_str(self.fields.join(",").as_str());
        sql.push_str(") VALUES ('");
        sql.push_str(self.values.join("','").as_str());
        sql.push_str("')");
        sql
    }

    /// updating sql
    fn build_update(&self) -> String { 
        let length = self.fields.len();
        let mut updates: Vec<String> = vec![];
        for i in 0..length { 
            updates.push(format!("{} = '{}'", &self.fields[i], &self.values[i]));
        }
        updates.join(",")
    }
}

impl<'a> Display for DataSet<'a> { 

    /// 格式化輸出數據
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { 
        let mut result = String::new();
        let method = if self.is_create { "create" } else { "update" };
        result.push_str(&format!("(method: {}){}\n", method, "{"));
        for i in 0..self.fields.len() { 
            result.push_str(&format!("    {} => {},\n", self.fields[i], self.values[i]));
        }
        result.push_str(&format!("{}", "}"));
        write!(f, "{}", result)
    }
}


/// 生成添加数據
#[macro_export]
macro_rules! create_row { 
    [$($key: expr => $val: expr,)+] => (
        {
            let mut data = crate::db::data_set::DataSet::create();
            $(data.set($key, &$val);)+
            data
        }
    )
}

/// 生成修改数據
#[macro_export]
macro_rules! update_row { 
    [$($key: expr => $val: expr,)*] => (
        {
            let mut data = crate::db::data_set::DataSet::update();
            $(data.set($key, &$val);)*
            data
        }
    )
}