use std::os::raw::c_void;
use std::os::raw::c_char;
use std::ffi::CStr;
use std::ffi::CString;
use std::ptr::null_mut;
use astrolabe::Date;
use astrolabe::DateUnit;
use astrolabe::errors::AstrolabeError;

#[repr(C)]
union astrolabe_date_t {
  data: *mut c_void,
  error: *mut c_char
}

#[repr(C)]
#[allow(non_camel_case_types)]
enum astrolabe_error {
  ASTROLABE_NONE,
  ASTROLABE_OUT_OF_RANGE,
  ASTROLABE_INVALID_FORMAT,
}

#[repr(C)]
#[allow(dead_code, non_camel_case_types)]
enum astrolabe_date_unit {
    DATE_UNIT_YEAR,
    DATE_UNIT_MONTH,
    DATE_UNIT_DAY,
}

#[no_mangle]
unsafe extern "C" fn astrolabe_date_now() -> astrolabe_date_t {
  astrolabe_date_t {
    data: Box::into_raw(Box::new(Date::now())) as *mut c_void
  }
}

#[no_mangle]
unsafe extern "C" fn astrolabe_date_from_ymd(year: i32, month: u32, day: u32, error_code: *mut astrolabe_error) -> astrolabe_date_t {
  let res = match Date::from_ymd(year, month, day) {
    Ok(v) => v,
    Err(e) => match e {
      AstrolabeError::OutOfRange(s) => {
        let err = match CString::new(s.to_string()) {
          Ok(v) => v.into_raw(),
          Err(_) => null_mut()
        };
        *error_code = astrolabe_error::ASTROLABE_OUT_OF_RANGE;
        return astrolabe_date_t {
          error: err
        }
      },
      AstrolabeError::InvalidFormat(s) => {
        let err = match CString::new(s.to_string()) {
          Ok(v) => v.into_raw(),
          Err(_) => null_mut()
        };
        *error_code = astrolabe_error::ASTROLABE_INVALID_FORMAT;
        return astrolabe_date_t {
          error: err
        }
      }
    }
  };
  *error_code = astrolabe_error::ASTROLABE_NONE;
  astrolabe_date_t {
    data: Box::into_raw(Box::new(res)) as *mut c_void
  }
}

#[no_mangle]
unsafe extern "C" fn astrolabe_date_from_timestamp(timestamp: i64, error_code: *mut astrolabe_error) -> astrolabe_date_t {
  let res = match Date::from_timestamp(timestamp) {
    Ok(v) => v,
    Err(e) => match e {
      AstrolabeError::OutOfRange(s) => {
        let err = match CString::new(s.to_string()) {
          Ok(v) => v.into_raw(),
          Err(_) => null_mut()
        };
        *error_code = astrolabe_error::ASTROLABE_OUT_OF_RANGE;
        return astrolabe_date_t {
          error: err
        }
      },
      AstrolabeError::InvalidFormat(s) => {
        let err = match CString::new(s.to_string()) {
          Ok(v) => v.into_raw(),
          Err(_) => null_mut()
        };
        *error_code = astrolabe_error::ASTROLABE_INVALID_FORMAT;
        return astrolabe_date_t {
          error: err
        }
      }
    }
  };
  *error_code = astrolabe_error::ASTROLABE_NONE;
  astrolabe_date_t {
    data: Box::into_raw(Box::new(res)) as *mut c_void
  }
}

#[no_mangle]
unsafe extern "C" fn astrolabe_date_from_days(days: i32) -> astrolabe_date_t {
  astrolabe_date_t {
    data: Box::into_raw(Box::new(Date::from_days(days))) as *mut c_void
  }
}

#[no_mangle]
unsafe extern "C" fn astrolabe_date_as_days(astrolabe_date: *mut astrolabe_date_t) -> i32 {
  let date = *((*astrolabe_date).data as *mut Date);
  date.as_days()
}

#[no_mangle]
unsafe extern "C" fn astrolabe_date_timestamp(astrolabe_date: *mut astrolabe_date_t) -> i64 {
  let date = *((*astrolabe_date).data as *mut Date);
  date.timestamp()
}

#[no_mangle]
unsafe extern "C" fn astrolabe_date_between(astrolabe_date: *mut astrolabe_date_t, compare: *mut astrolabe_date_t) -> u32 {
  let date = *((*astrolabe_date).data as *mut Date);
  let compare_rs = *((*compare).data as *mut Date);
  date.between(&compare_rs)
}

#[no_mangle]
unsafe extern "C" fn astrolabe_date_get(astrolabe_date: *mut astrolabe_date_t, unit: astrolabe_date_unit) -> i32 {
  let unit_rs = match unit {
    astrolabe_date_unit::DATE_UNIT_YEAR => DateUnit::Year,
    astrolabe_date_unit::DATE_UNIT_MONTH => DateUnit::Month,
    astrolabe_date_unit::DATE_UNIT_DAY => DateUnit::Day
  };
  let date = *((*astrolabe_date).data as *mut Date);
  date.get(unit_rs)
}

#[no_mangle]
unsafe extern "C" fn astrolabe_date_set(astrolabe_date: *mut astrolabe_date_t, value: i32, unit: astrolabe_date_unit, error_code: *mut astrolabe_error) -> astrolabe_date_t {
  let unit_rs = match unit {
    astrolabe_date_unit::DATE_UNIT_YEAR => DateUnit::Year,
    astrolabe_date_unit::DATE_UNIT_MONTH => DateUnit::Month,
    astrolabe_date_unit::DATE_UNIT_DAY => DateUnit::Day
  };
  let date = *((*astrolabe_date).data as *mut Date);
  match date.set(value,unit_rs) {
    Ok(v) => {
      *error_code = astrolabe_error::ASTROLABE_NONE;
      astrolabe_date_t {
        data:  Box::into_raw(Box::new(v)) as *mut c_void
      }
    },
    Err(e) => match e {
      AstrolabeError::OutOfRange(s) => {
        let err = match CString::new(s.to_string()) {
          Ok(v) => v.into_raw(),
          Err(_) => null_mut()
        };
        *error_code = astrolabe_error::ASTROLABE_OUT_OF_RANGE;
        astrolabe_date_t {
          error: err
        }
      },
      AstrolabeError::InvalidFormat(s) => {
        let err = match CString::new(s.to_string()) {
          Ok(v) => v.into_raw(),
          Err(_) => null_mut()
        };
        *error_code = astrolabe_error::ASTROLABE_INVALID_FORMAT;
        astrolabe_date_t {
          error: err
        }
      }
    }
  }
}

#[no_mangle]
unsafe extern "C" fn astrolabe_date_apply(astrolabe_date: *mut astrolabe_date_t, amount: i32, unit: astrolabe_date_unit, error_code: *mut astrolabe_error) -> astrolabe_date_t {
  let unit_rs = match unit {
    astrolabe_date_unit::DATE_UNIT_YEAR => DateUnit::Year,
    astrolabe_date_unit::DATE_UNIT_MONTH => DateUnit::Month,
    astrolabe_date_unit::DATE_UNIT_DAY => DateUnit::Day
  };
  let date = *((*astrolabe_date).data as *mut Date);
  match date.apply(amount,unit_rs) {
    Ok(v) => {
      *error_code = astrolabe_error::ASTROLABE_NONE;
      astrolabe_date_t {
        data:  Box::into_raw(Box::new(v)) as *mut c_void
      }
    },
    Err(e) => match e {
      AstrolabeError::OutOfRange(s) => {
        let err = match CString::new(s.to_string()) {
          Ok(v) => v.into_raw(),
          Err(_) => null_mut()
        };
        *error_code = astrolabe_error::ASTROLABE_OUT_OF_RANGE;
        astrolabe_date_t {
          error: err
        }
      },
      AstrolabeError::InvalidFormat(s) => {
        let err = match CString::new(s.to_string()) {
          Ok(v) => v.into_raw(),
          Err(_) => null_mut()
        };
        *error_code = astrolabe_error::ASTROLABE_INVALID_FORMAT;
        astrolabe_date_t {
          error: err
        }
      }
    }
  }
}

#[no_mangle]
unsafe extern "C" fn astrolabe_date_format(astrolabe_date: *mut astrolabe_date_t, format: *const c_char) -> *mut c_char {
  let format_rs = match CStr::from_ptr(format).to_str() {
    Ok(v) => v,
    Err(_) => return null_mut()
  };
  let date = *((*astrolabe_date).data as *mut Date);
  let res = date.format(format_rs);
  match CString::new(res) {
    Ok(v) => v.into_raw(),
    Err(_) => null_mut()
  }
}

#[cfg(test)]
mod tests {
  use crate::*;
  use std::ffi::CString;
  use std::ffi::CStr;

  #[test]
  fn astrolabe_date_now_test() {
    unsafe {
      let mut date = astrolabe_date_now();
      assert!(2021 < astrolabe_date_get(&mut date as *mut astrolabe_date_t, astrolabe_date_unit::DATE_UNIT_YEAR));
    }
  }

  #[test]
  fn astrolabe_date_from_ymd_test() {
    #[allow(temporary_cstring_as_ptr)]
    unsafe {
      let mut err = astrolabe_error::ASTROLABE_NONE;
      let mut date = astrolabe_date_from_ymd(2022, 05, 02, &mut err as *mut astrolabe_error);
      assert_eq!("2022/05/02", CStr::from_ptr(astrolabe_date_format(&mut date as *mut astrolabe_date_t, CString::new("yyyy/MM/dd").unwrap().as_ptr())).to_str().unwrap());
    }
  }

  #[test]
  fn astrolabe_date_from_timestamp_test() {
    #[allow(temporary_cstring_as_ptr)]
    unsafe {
      let mut err = astrolabe_error::ASTROLABE_NONE;
      let mut date = astrolabe_date_from_timestamp(0, &mut err as *mut astrolabe_error);
      assert_eq!("1970/01/01", CStr::from_ptr(astrolabe_date_format(&mut date as *mut astrolabe_date_t, CString::new("yyyy/MM/dd").unwrap().as_ptr())).to_str().unwrap());
    }
  }

  #[test]
  fn astrolabe_date_from_days_test() {
    #[allow(temporary_cstring_as_ptr)]
    unsafe {
      let mut date = astrolabe_date_from_days(738276);
      assert_eq!("2022/05/02", CStr::from_ptr(astrolabe_date_format(&mut date as *mut astrolabe_date_t, CString::new("yyyy/MM/dd").unwrap().as_ptr())).to_str().unwrap());
    }
  }

  #[test]
  fn astrolabe_date_as_days_test() {
    unsafe {
      let mut err = astrolabe_error::ASTROLABE_NONE;
      let mut date = astrolabe_date_from_ymd(1, 1, 1, &mut err as *mut astrolabe_error);
      assert_eq!(0, astrolabe_date_as_days(&mut date as *mut astrolabe_date_t));
    }
  }

  #[test]
  fn astrolabe_date_timestamp_test() {
    unsafe {
      let mut err = astrolabe_error::ASTROLABE_NONE;
      let mut date = astrolabe_date_from_ymd(2000, 1, 1, &mut err as *mut astrolabe_error);
      assert_eq!(946_684_800, astrolabe_date_timestamp(&mut date as *mut astrolabe_date_t));
    }
  }

  #[test]
  fn astrolabe_date_between_test() {
    unsafe {
      let mut err = astrolabe_error::ASTROLABE_NONE;
      let mut err2 = astrolabe_error::ASTROLABE_NONE;
      let mut date = astrolabe_date_from_ymd(1970, 1, 1, &mut err as *mut astrolabe_error);
      let mut date2 = astrolabe_date_from_ymd(1970, 2, 1, &mut err2 as *mut astrolabe_error);
      assert_eq!(31, astrolabe_date_between(&mut date as *mut astrolabe_date_t, &mut date2 as *mut astrolabe_date_t));
    }
  }

  #[test]
  fn astrolabe_date_get_test() {
    unsafe {
      let mut err = astrolabe_error::ASTROLABE_NONE;
      let mut date = astrolabe_date_from_ymd(2022, 5, 2, &mut err as *mut astrolabe_error);
      assert_eq!(2022, astrolabe_date_get(&mut date as *mut astrolabe_date_t, astrolabe_date_unit::DATE_UNIT_YEAR));
      assert_eq!(5, astrolabe_date_get(&mut date as *mut astrolabe_date_t, astrolabe_date_unit::DATE_UNIT_MONTH));
      assert_eq!(2, astrolabe_date_get(&mut date as *mut astrolabe_date_t, astrolabe_date_unit::DATE_UNIT_DAY));
    }
  }

  #[test]
  fn astrolabe_date_set_test() {
    #[allow(temporary_cstring_as_ptr)]
    unsafe {
      let mut err = astrolabe_error::ASTROLABE_NONE;
      let mut date = astrolabe_date_from_ymd(2022, 5, 2, &mut err as *mut astrolabe_error);
      date = astrolabe_date_set(&mut date as *mut astrolabe_date_t, 2000, astrolabe_date_unit::DATE_UNIT_YEAR, &mut err as *mut astrolabe_error);
      date = astrolabe_date_set(&mut date as *mut astrolabe_date_t, 10, astrolabe_date_unit::DATE_UNIT_DAY, &mut err as *mut astrolabe_error);
      assert_eq!("2000/05/10", CStr::from_ptr(astrolabe_date_format(&mut date as *mut astrolabe_date_t, CString::new("yyyy/MM/dd").unwrap().as_ptr())).to_str().unwrap());
    }
  }

  #[test]
  fn astrolabe_date_apply_test() {
    #[allow(temporary_cstring_as_ptr)]
    unsafe {
      let mut err = astrolabe_error::ASTROLABE_NONE;
      let mut err_applied = astrolabe_error::ASTROLABE_NONE;
      let mut date = astrolabe_date_from_ymd(1970, 1, 1, &mut err as *mut astrolabe_error);
      let mut applied = astrolabe_date_apply(&mut date as *mut astrolabe_date_t, 1, astrolabe_date_unit::DATE_UNIT_DAY, &mut err_applied as *mut astrolabe_error);
      assert_eq!("1970/01/01", CStr::from_ptr(astrolabe_date_format(&mut date as *mut astrolabe_date_t, CString::new("yyyy/MM/dd").unwrap().as_ptr())).to_str().unwrap());
      assert_eq!("1970/01/02", CStr::from_ptr(astrolabe_date_format(&mut applied as *mut astrolabe_date_t, CString::new("yyyy/MM/dd").unwrap().as_ptr())).to_str().unwrap());
    }
  }

  #[test]
  fn astrolabe_date_format_test() {
    #[allow(temporary_cstring_as_ptr)]
    unsafe {
      let mut err = astrolabe_error::ASTROLABE_NONE;
      let mut date = astrolabe_date_from_ymd(2022, 5, 2, &mut err as *mut astrolabe_error);
      assert_eq!("2022/05/02", CStr::from_ptr(astrolabe_date_format(&mut date as *mut astrolabe_date_t, CString::new("yyyy/MM/dd").unwrap().as_ptr())).to_str().unwrap());
      assert_eq!("2022/MM/dd", CStr::from_ptr(astrolabe_date_format(&mut date as *mut astrolabe_date_t, CString::new("yyyy/'MM/dd'").unwrap().as_ptr())).to_str().unwrap());
      assert_eq!("2022/'05/02'", CStr::from_ptr(astrolabe_date_format(&mut date as *mut astrolabe_date_t, CString::new("yyyy/''MM/dd''").unwrap().as_ptr())).to_str().unwrap());
    }
  }
}