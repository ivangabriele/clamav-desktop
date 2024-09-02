# https://github.com/Cisco-Talos/clamav/blob/main/INSTALL-cross-linux-arm64.md#if-using-a-sysroot

# Platform
set(CMAKE_SYSTEM_NAME       Linux)
set(CMAKE_SYSTEM_PROCESSOR  arm64)
set(CMAKE_C_COMPILER        "aarch64-linux-gnu-gcc")
set(CMAKE_CXX_COMPILER      "aarch64-linux-gnu-g++")
set(RUST_COMPILER_TARGET    "aarch64-unknown-linux-gnu")

# Project Variables needed to cross compile
set(HAVE_ATTRIB_ALIGNED     1)
set(HAVE_ATTRIB_PACKED      1)
set(HAVE_UNAME_SYSCALL      1)
set(HAVE_SAR                1)
set(HAVE_FD_PASSING         1)
set(MMAP_FOR_CROSSCOMPILING ON)
set(ENABLE_SYSTEMD          OFF)

set( test_run_result
     "PLEASE_FILL_OUT-FAILED_TO_RUN"
     CACHE STRING "Result from try_run" FORCE)

set( test_run_result__TRYRUN_OUTPUT
     "PLEASE_FILL_OUT-NOTFOUND"
     CACHE STRING "Output from try_run" FORCE)

#
# Dependencies
#

# If using a sysroot / rootfs for the target, set these.
set(CMAKE_SYSROOT           /opt/aarch64-wrs-linux-sysroot)

# If your CMAKE_SYSROOT directory is readonly, or for some reason you want to install to a different staging prefix before copying  to your host, set this:
#set(CMAKE_STAGING_PREFIX    /home/user/stage)

# Note, you may need to set ENABLE_JSON_SHARED if your sysroot provides libjson-c.so instead of libjson-c.a.
#set(ENABLE_JSON_SHARED      ON)

# You may need to set the following if CMake has some trouble finding the dependencies.
# For example if you have `libjson-c.a` in your sysroot, here: `/opt/aarch64-wrs-linux-sysroot/usr/lib64/libjson-c.a`
# then you would set:
#set(JSONC_LIBRARY           "/usr/lib64/libjson-c.a")

#
# Uncomment these as needed:
#
#set(JSONC_INCLUDE_DIR       "/usr/include/json-c")
#set(JSONC_LIBRARY           "/usr/lib64/libjson-c.a")
#set(ENABLE_JSON_SHARED      OFF)

#set(BZIP2_INCLUDE_DIR       "/usr/include/")
#set(BZIP2_LIBRARY           "/usr/lib64/libbz2.a")

#set(OPENSSL_ROOT_DIR        "/usr/")
#set(OPENSSL_INCLUDE_DIR     "/usr/include/")
#set(OPENSSL_CRYPTO_LIBRARY  "/usr/lib64/libcrypto.so")
#set(OPENSSL_SSL_LIBRARY     "/usr/lib64/libssl.so")

#set(LIBXML2_INCLUDE_DIR     "/usr/include/libxml2")
#set(LIBXML2_LIBRARY         "/usr/lib64/libxml2.so")

#set(PCRE2_INCLUDE_DIR       "/usr/include/")
#set(PCRE2_LIBRARY           "/usr/lib64/libpcre2-8.so")

#set(CURSES_INCLUDE_DIR      "/usr/include/")
#set(CURSES_LIBRARY          "/usr/lib/aarch64-linux-gnu/libncurses.a;/usr/lib/aarch64-linux-gnu/libtinfo.a")
# Tip: You may not need to also link with libtinfo.a, depending on what your distribution provides:
#set(CURSES_LIBRARY          "/usr/lib/aarch64-linux-gnu/libncurses.a")
# Tip: Alternatively, you could link with the shared library:
#set(CURSES_LIBRARY          "/usr/lib/aarch64-linux-gnu/libncurses.so")

#set(ZLIB_INCLUDE_DIR        "/usr/include/")
#set(ZLIB_LIBRARY            "/usr/lib64/libz.so")

#set(LIBCHECK_INCLUDE_DIR    "/usr/include/")
#set(LIBCHECK_LIBRARY        "/usr/lib64/libcheck.a")
