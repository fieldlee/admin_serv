use std::fmt::Display;

/// 生成查詢條件
///
/// 調用方法: 
///
/// ```rust
/// let _cond = CondBuilder::new()
///     .eq("id", 1)
///     .gt("age", 30)
///     .ne("name", "james")
///     .lt("score", 90)
///     .build();
/// // 或者如下:
/// let _cond = cond![
///     "id" => &1
/// ];
/// ```
#[derive(Debug)]
pub struct CondBuilder { 
    conditions: String,
    count: usize,
}

impl<'a> CondBuilder { 
    
    /// Create new CondBuilder
    pub fn new() -> Self { 
        CondBuilder{ 
            conditions: "1 = 1".into(),
            count: 0,
        }
    }
    
    /// append new condition
    pub fn append(&mut self, where_str: &str) -> &mut Self { 
        self.conditions.push_str(&format!(" AND {}", where_str));
        self.count += 1;
        self
    }

    /// 等於
    pub fn eq<T: Display>(&mut self, key: &str, value: &T) -> &mut Self { 
        self.conditions.push_str(&format!(" AND {} = '{}'", key, value));
        self.count += 1;
        self
    }

    /// 不等於
    pub fn ne<T: Display>(&mut self, key: &str, value: &T) -> &mut Self { 
        self.conditions.push_str(&format!(" AND {} <> '{}'", key, value));
        self.count += 1;
        self
    }

    /// 大於
    pub fn gt<T: Display>(&mut self, key: &str, value: &T) -> &mut Self { 
        self.conditions.push_str(&format!(" AND {} > '{}'", key, value));
        self.count += 1;
        self
    }

    /// 大於等於
    pub fn gte<T: Display>(&mut self, key: &str, value: &T) -> &mut Self { 
        self.conditions.push_str(&format!(" AND {} >= '{}'", key, value));
        self.count += 1;
        self
    }

    /// 小於
    pub fn lt<T: Display>(&mut self, key: &str, value: &T) -> &mut Self { 
        self.conditions.push_str(&format!(" AND {} < '{}'", key, value));
        self.count += 1;
        self
    }

    /// 小於等於
    pub fn lte<T: Display>(&mut self, key: &str, value: &T) -> &mut Self { 
        self.conditions.push_str(&format!(" AND {} <= '{}'", key, value));
        self.count += 1;
        self
    }

    /// LIKE
    pub fn like<T: Display>(&mut self, key: &str, value: &T) -> &mut Self { 
        self.conditions.push_str(&format!(" AND {} LIKE '%{}%'", key, value));
        self.count += 1;
        self
    }

    /// BETWEEN
    pub fn between<T: Display>(&mut self, key: &str, value1: &T, value2: &T) -> &mut Self { 
        self.conditions.push_str(&format!(" AND {} BETWEEN '{}' AND '{}'", key, value1, value2));
        self.count += 1;
        self
    }

    /// OR
    pub fn or(&mut self, cond: &CondBuilder) -> &mut Self { 
        self.conditions.push_str(&format!(" OR ({})", cond.build()));
        self.count += 1;
        self
    }

    /// IN
    pub fn in_range<T: Display>(&mut self, key: &str, value: &T) -> &mut Self { 
        self.conditions.push_str(&format!(" AND {} IN ({})", key, value));
        self.count += 1;
        self
    }
    
    /// CLEAR
    pub fn clear(&mut self) -> &mut Self { 
        self.conditions = "1 = 1".to_owned();
        self
    }

    /// build
    pub fn build(&'a self) -> &'a String { 
        &self.conditions
    }
    
    /// 条件数量
    pub fn len(&self) -> usize { 
        self.count
    }
}


/// 全部以等號標識出來的查詢條件
#[macro_export]
macro_rules! cond { 
    [$($method: ident => [ $($field: expr => &$val: expr,)+],)* ] => ({
        let mut cond = crate::data::cond_builder::CondBuilder::new();
        $($(cond.$method($field, &$val);)*)*
        cond
    });
    [$($key: expr => $val: expr,)*] => ({
        let mut cond = crate::data::cond_builder::CondBuilder::new();
        $(cond.eq($key, &$val);)*
        cond
    });
}
