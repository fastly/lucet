use lucet_runtime_tests::host_tests;

cfg_if::cfg_if! {
    if #[cfg(feature = "uffd")] {
        host_tests!(
            mmap => lucet_runtime::MmapRegion,
            uffd => lucet_runtime::UffdRegion
        );
    } else {
        host_tests!(mmap => lucet_runtime::MmapRegion);
    }
}
