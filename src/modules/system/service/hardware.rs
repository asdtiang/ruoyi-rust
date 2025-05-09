use super::super::domain::vo::{CpuVO, DiskVO, Memory, ServerVO, SysVO};
use bytesize::ByteSize;

pub fn get_server_info() -> ServerVO {
    // let mut sys = System::new_with_specifics(
    //     RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()),
    // );
    let mut sys = sysinfo::System::new_all();
    // First we update all information of our `System` struct.
    //  sys.refresh_memory();
    //  std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    // sys.refresh_cpu_usage();
    sys.refresh_all();

    // First we update all information of our `System` struct.

    // We display all disks' information:
    //   println!("=> disks:");
    let mut disks = vec![];
    for d in &sysinfo::Disks::new_with_refreshed_list() {
        let disk = DiskVO {
            dir_name: d.name().to_str().unwrap().to_string(),
            free: ByteSize::b(d.available_space()).to_string(),
            sys_type_name: d.file_system().to_str().unwrap_or_default().to_string(),
            type_name: d.mount_point().to_str().unwrap().to_string(),
            total: ByteSize::b(d.total_space()).to_string(),
            used: ByteSize::b(d.total_space() - d.available_space()).to_string(),
            usage: format!(
                "{:.1}",
                100.0
                    - 100.0 * (d.total_space() - d.available_space()) as f64
                        / d.total_space() as f64
            ),
        };
        disks.push(disk);
        //     println!("{:?}", disk);
        //   println!("{:?}", disk_);
    }

    let networks = sysinfo::Networks::new_with_refreshed_list();
    for (interface_name, network) in &networks {
        println!("[{interface_name}]: {network:?}");
    }
    for (interface_name, data) in &sysinfo::Networks::new_with_refreshed_list() {
        println!(
            "{}: {}/{} B",
            interface_name,
            data.received(),
            data.transmitted());
            data.ip_networks().iter().for_each(|network| {println!(" {:?}",network.addr.to_string());});
        // );
    }

    let mem = Memory {
        free: ByteSize::b(sys.used_memory())
            .to_string()
            .trim_end_matches(" GB")
            .to_string(), //fixme .trim_end_matches(" GB").to_string()为了兼容前端，以后需要删除
        total: ByteSize::b(sys.total_memory())
            .to_string()
            .trim_end_matches(" GB")
            .to_string(),
        used: ByteSize::b(sys.total_memory() - sys.used_memory())
            .to_string()
            .trim_end_matches(" GB")
            .to_string(),
        usage: format!(
            "{:.1}",
            100.0 - 100.0 * sys.used_memory() as f64 / sys.total_memory() as f64
        ),
    };
    // println!("{:?}", mem);

    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_cpu_usage();
    let cpu_usage = sys.global_cpu_usage();
    let cpu = CpuVO {
        cpu_num: sysinfo::System::physical_core_count().unwrap_or_default() as u8,
        free: 100.0 - cpu_usage,
        sys: 0.0,
        total: 0,
        used: cpu_usage,
        wait: 0.0,
    };
    println!("{:?}", cpu);
    let sys_vo = SysVO {
        computer_ip: "".to_string(),
        computer_name: sysinfo::System::host_name().unwrap_or_default(),
        os_arch: sysinfo::System::kernel_version().unwrap_or_default(),
        os_name: sysinfo::System::long_os_version().unwrap_or_default(),
    };
    println!("{:?}", sys_vo);

    ServerVO {
        cpu,
        mem,
        sys: sys_vo,
        sys_files: disks,
    }

}
