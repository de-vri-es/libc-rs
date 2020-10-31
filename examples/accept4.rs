fn main() -> std::io::Result<()> {
    unsafe {
        let listener = check_i32(libc::socket(libc::AF_INET6, libc::SOCK_STREAM, 0))?;
        {
            let mut addr: libc::sockaddr_in6 = std::mem::zeroed();
            addr.sin6_family = libc::AF_INET6 as _;
            addr.sin6_port = 1337u16.to_be();
            check_i32(libc::bind(listener, &addr as *const _ as *const _, std::mem::size_of_val(&addr) as _))?;
            check_i32(libc::listen(listener, 1))?;
        }


        let mut addr: libc::sockaddr_in6 = std::mem::zeroed();
        let mut len: u32 = 0;
        let connection = check_i32(libc::accept4(listener, &mut addr as *mut _ as *mut _, &mut len, 0))?;

        let buffer = "Doei!\n";
        check_isize(libc::send(connection, buffer.as_ptr() as *const _, buffer.len(), 0))?;
    }

    Ok(())
}

fn check_i32(ret: i32) -> std::io::Result<i32> {
    if ret == -1 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(ret)
    }
}

fn check_isize(ret: isize) -> std::io::Result<isize> {
    if ret == -1 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(ret)
    }
}
