(module
  (type (;0;) (func))
  (type (;1;) (func (param i32)))
  (type (;2;) (func (param i32 i32) (result i32)))
  (type (;3;) (func (param i32) (result i64)))
  (type (;4;) (func (param i32) (result i32)))
  (type (;5;) (func (param i32 i32 i32)))
  (type (;6;) (func (param i32 i32)))
  (type (;7;) (func (param i32 i32 i32) (result i32)))
  (type (;8;) (func (param i32 i32 i32 i32) (result i32)))
  (type (;9;) (func (param i32 i32 i32 i32 i32 i64 i64 i32 i32) (result i32)))
  (type (;10;) (func (result i32)))
  (type (;11;) (func (param i32 i32 i32 i32)))
  (type (;12;) (func (param i32 i32 i32 i32 i32) (result i32)))
  (type (;13;) (func (param i32 i32 i32 i32 i32)))
  (type (;14;) (func (param i32 i32 i32 i32 i32 i32 i64 i64 i32)))
  (type (;15;) (func (param i32 i32 i32 i32 i32 i32) (result i32)))
  (type (;16;) (func (param i32 i32 i32 i32 i32 i32 i32)))
  (type (;17;) (func (param i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32) (result i32)))
  (type (;18;) (func (param i32 i32 i32 i32 i32 i32 i32) (result i32)))
  (type (;19;) (func (param i64 i32 i32) (result i32)))
  (import "wasi_snapshot_preview1" "fd_write" (func $_ZN4wasi13lib_generated22wasi_snapshot_preview18fd_write17h7ef83300274a0defE (type 8)))
  (import "wasi_snapshot_preview1" "path_open" (func $_ZN4wasi13lib_generated22wasi_snapshot_preview19path_open17h9cf26e64f10a7e40E (type 9)))
  (import "wasi_snapshot_preview1" "environ_get" (func $__imported_wasi_snapshot_preview1_environ_get (type 2)))
  (import "wasi_snapshot_preview1" "environ_sizes_get" (func $__imported_wasi_snapshot_preview1_environ_sizes_get (type 2)))
  (import "wasi_snapshot_preview1" "fd_close" (func $__imported_wasi_snapshot_preview1_fd_close (type 4)))
  (import "wasi_snapshot_preview1" "fd_prestat_get" (func $__imported_wasi_snapshot_preview1_fd_prestat_get (type 2)))
  (import "wasi_snapshot_preview1" "fd_prestat_dir_name" (func $__imported_wasi_snapshot_preview1_fd_prestat_dir_name (type 7)))
  (import "wasi_snapshot_preview1" "proc_exit" (func $__imported_wasi_snapshot_preview1_proc_exit (type 1)))
  (func $__wasm_call_ctors (type 0)
    call $__wasilibc_populate_preopens)
  (func $_start (type 0)
    (local i32)
    block  ;; label = @1
      block  ;; label = @2
        i32.const 0
        i32.load offset=1059048
        br_if 0 (;@2;)
        i32.const 0
        i32.const 1
        i32.store offset=1059048
        call $__wasm_call_ctors
        call $__main_void
        local.set 0
        call $__wasm_call_dtors
        local.get 0
        br_if 1 (;@1;)
        return
      end
      unreachable
      unreachable
    end
    local.get 0
    call $__wasi_proc_exit
    unreachable)
  (func $_ZN3std4path4Path3new17h3947b92e6e750231E (type 5) (param i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    local.set 3
    i32.const 16
    local.set 4
    local.get 3
    local.get 4
    i32.sub
    local.set 5
    local.get 5
    global.set $__stack_pointer
    i32.const 8
    local.set 6
    local.get 5
    local.get 6
    i32.add
    local.set 7
    local.get 7
    local.get 1
    local.get 2
    call $_ZN3std3ffi6os_str85_$LT$impl$u20$core..convert..AsRef$LT$std..ffi..os_str..OsStr$GT$$u20$for$u20$str$GT$6as_ref17hc23af37d18ee078bE
    local.get 5
    i32.load offset=8
    local.set 8
    local.get 5
    i32.load offset=12
    local.set 9
    local.get 0
    local.get 9
    i32.store offset=4
    local.get 0
    local.get 8
    i32.store
    i32.const 16
    local.set 10
    local.get 5
    local.get 10
    i32.add
    local.set 11
    local.get 11
    global.set $__stack_pointer
    return)
  (func $_ZN3std4path77_$LT$impl$u20$core..convert..AsRef$LT$std..path..Path$GT$$u20$for$u20$str$GT$6as_ref17h86bbea0ad92c0d4cE (type 5) (param i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    local.set 3
    i32.const 16
    local.set 4
    local.get 3
    local.get 4
    i32.sub
    local.set 5
    local.get 5
    global.set $__stack_pointer
    i32.const 8
    local.set 6
    local.get 5
    local.get 6
    i32.add
    local.set 7
    local.get 7
    local.get 1
    local.get 2
    call $_ZN3std4path4Path3new17h3947b92e6e750231E
    local.get 5
    i32.load offset=8
    local.set 8
    local.get 5
    i32.load offset=12
    local.set 9
    local.get 0
    local.get 9
    i32.store offset=4
    local.get 0
    local.get 8
    i32.store
    i32.const 16
    local.set 10
    local.get 5
    local.get 10
    i32.add
    local.set 11
    local.get 11
    global.set $__stack_pointer
    return)
  (func $_ZN55_$LT$$RF$T$u20$as$u20$core..convert..AsRef$LT$U$GT$$GT$6as_ref17hb0a2da4d62f79317E (type 6) (param i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    local.set 2
    i32.const 16
    local.set 3
    local.get 2
    local.get 3
    i32.sub
    local.set 4
    local.get 4
    global.set $__stack_pointer
    local.get 1
    i32.load
    local.set 5
    local.get 1
    i32.load offset=4
    local.set 6
    i32.const 8
    local.set 7
    local.get 4
    local.get 7
    i32.add
    local.set 8
    local.get 8
    local.get 5
    local.get 6
    call $_ZN79_$LT$std..path..Path$u20$as$u20$core..convert..AsRef$LT$std..path..Path$GT$$GT$6as_ref17he07adbbd0989d10dE
    local.get 4
    i32.load offset=8
    local.set 9
    local.get 4
    i32.load offset=12
    local.set 10
    local.get 0
    local.get 10
    i32.store offset=4
    local.get 0
    local.get 9
    i32.store
    i32.const 16
    local.set 11
    local.get 4
    local.get 11
    i32.add
    local.set 12
    local.get 12
    global.set $__stack_pointer
    return)
  (func $_ZN79_$LT$std..path..Path$u20$as$u20$core..convert..AsRef$LT$std..path..Path$GT$$GT$6as_ref17he07adbbd0989d10dE (type 5) (param i32 i32 i32)
    local.get 0
    local.get 2
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store
    return)
  (func $_ZN55_$LT$$RF$T$u20$as$u20$core..convert..AsRef$LT$U$GT$$GT$6as_ref17h5143b396fe4ad64fE (type 6) (param i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    local.set 2
    i32.const 16
    local.set 3
    local.get 2
    local.get 3
    i32.sub
    local.set 4
    local.get 4
    global.set $__stack_pointer
    local.get 1
    i32.load
    local.set 5
    local.get 1
    i32.load offset=4
    local.set 6
    i32.const 8
    local.set 7
    local.get 4
    local.get 7
    i32.add
    local.set 8
    local.get 8
    local.get 5
    local.get 6
    call $_ZN3std4path77_$LT$impl$u20$core..convert..AsRef$LT$std..path..Path$GT$$u20$for$u20$str$GT$6as_ref17h86bbea0ad92c0d4cE
    local.get 4
    i32.load offset=8
    local.set 9
    local.get 4
    i32.load offset=12
    local.set 10
    local.get 0
    local.get 10
    i32.store offset=4
    local.get 0
    local.get 9
    i32.store
    i32.const 16
    local.set 11
    local.get 4
    local.get 11
    i32.add
    local.set 12
    local.get 12
    global.set $__stack_pointer
    return)
  (func $_ZN69_$LT$std..os..fd..owned..OwnedFd$u20$as$u20$core..ops..drop..Drop$GT$4drop17he6169dc63e72538bE (type 1) (param i32)
    (local i32)
    local.get 0
    i32.load
    local.set 1
    local.get 1
    call $close
    drop
    return)
  (func $_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17h71f1bfe3b498cfb2E (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i64 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    local.set 2
    i32.const 16
    local.set 3
    local.get 2
    local.get 3
    i32.sub
    local.set 4
    local.get 4
    global.set $__stack_pointer
    local.get 0
    i32.load8_u
    local.set 5
    i32.const 4
    local.set 6
    i32.const 255
    local.set 7
    local.get 5
    local.get 7
    i32.and
    local.set 8
    i32.const 255
    local.set 9
    local.get 6
    local.get 9
    i32.and
    local.set 10
    local.get 8
    local.get 10
    i32.eq
    local.set 11
    i32.const 0
    local.set 12
    i32.const 1
    local.set 13
    i32.const 1
    local.set 14
    local.get 11
    local.get 14
    i32.and
    local.set 15
    local.get 12
    local.get 13
    local.get 15
    select
    local.set 16
    block  ;; label = @1
      local.get 16
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=4
      local.set 17
      i32.const 16
      local.set 18
      local.get 4
      local.get 18
      i32.add
      local.set 19
      local.get 19
      global.set $__stack_pointer
      local.get 17
      return
    end
    local.get 0
    i64.load align=4
    local.set 20
    local.get 4
    local.get 20
    i64.store offset=8
    i32.const 1048576
    local.set 21
    i32.const 43
    local.set 22
    i32.const 8
    local.set 23
    local.get 4
    local.get 23
    i32.add
    local.set 24
    local.get 24
    local.set 25
    i32.const 1048620
    local.set 26
    local.get 21
    local.get 22
    local.get 25
    local.get 26
    local.get 1
    call $_ZN4core6result13unwrap_failed17h0cc42b3b30eb09ddE
    unreachable)
  (func $_ZN3std3ffi6os_str85_$LT$impl$u20$core..convert..AsRef$LT$std..ffi..os_str..OsStr$GT$$u20$for$u20$str$GT$6as_ref17hc23af37d18ee078bE (type 5) (param i32 i32 i32)
    local.get 0
    local.get 2
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store
    return)
  (func $_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17had63d9c7cc120b0cE (type 10) (result i32)
    (local i32 i32 i32)
    i32.const 0
    local.set 0
    i32.const 1
    local.set 1
    local.get 0
    local.get 1
    i32.and
    local.set 2
    local.get 2
    return)
  (func $_ZN5hello4main17h4f085eaf1e607c70E (type 0)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    local.set 0
    i32.const 16
    local.set 1
    local.get 0
    local.get 1
    i32.sub
    local.set 2
    local.get 2
    global.set $__stack_pointer
    i32.const 8
    local.set 3
    local.get 2
    local.get 3
    i32.add
    local.set 4
    local.get 4
    local.set 5
    i32.const 1048636
    local.set 6
    i32.const 3
    local.set 7
    local.get 5
    local.get 6
    local.get 7
    call $_ZN3std2fs4File6create17hb303bb79fb1817f3E
    i32.const 8
    local.set 8
    local.get 2
    local.get 8
    i32.add
    local.set 9
    local.get 9
    local.set 10
    i32.const 1048656
    local.set 11
    local.get 10
    local.get 11
    call $_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17h71f1bfe3b498cfb2E
    local.set 12
    local.get 2
    local.get 12
    i32.store offset=4
    i32.const 4
    local.set 13
    local.get 2
    local.get 13
    i32.add
    local.set 14
    local.get 14
    local.set 15
    local.get 15
    call $_ZN4core3ptr34drop_in_place$LT$std..fs..File$GT$17h250bb08874e2cc92E
    i32.const 16
    local.set 16
    local.get 2
    local.get 16
    i32.add
    local.set 17
    local.get 17
    global.set $__stack_pointer
    return)
  (func $__main_void (type 10) (result i32)
    (local i32 i32 i32 i32)
    i32.const 3
    local.set 0
    i32.const 0
    local.set 1
    i32.const 0
    local.set 2
    local.get 0
    local.get 1
    local.get 1
    local.get 2
    call $_ZN3std2rt10lang_start17h143f8e995e466386E
    local.set 3
    local.get 3
    return)
  (func $_ZN5alloc5alloc8box_free17h67caeca95fed6a6cE (type 1) (param i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    local.set 1
    i32.const 48
    local.set 2
    local.get 1
    local.get 2
    i32.sub
    local.set 3
    local.get 3
    global.set $__stack_pointer
    local.get 3
    local.get 0
    i32.store offset=4
    i32.const 12
    local.set 4
    local.get 3
    local.get 4
    i32.store offset=40
    local.get 3
    i32.load offset=40
    local.set 5
    i32.const 4
    local.set 6
    local.get 3
    local.get 6
    i32.store offset=44
    local.get 3
    i32.load offset=44
    local.set 7
    local.get 3
    local.get 5
    i32.store offset=16
    local.get 3
    local.get 7
    i32.store offset=20
    local.get 3
    i32.load offset=4
    local.set 8
    local.get 3
    local.get 8
    i32.store offset=36
    local.get 3
    i32.load offset=36
    local.set 9
    local.get 3
    local.get 9
    i32.store offset=32
    local.get 3
    i32.load offset=32
    local.set 10
    local.get 3
    local.get 10
    i32.store offset=28
    local.get 3
    i32.load offset=28
    local.set 11
    local.get 3
    i32.load offset=16
    local.set 12
    local.get 3
    i32.load offset=20
    local.set 13
    i32.const 8
    local.set 14
    local.get 3
    local.get 14
    i32.add
    local.set 15
    local.get 15
    local.set 16
    local.get 16
    local.get 11
    local.get 12
    local.get 13
    call $_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h72169ff628d8bf71E
    i32.const 48
    local.set 17
    local.get 3
    local.get 17
    i32.add
    local.set 18
    local.get 18
    global.set $__stack_pointer
    return)
  (func $_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h72169ff628d8bf71E (type 11) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    local.set 4
    i32.const 32
    local.set 5
    local.get 4
    local.get 5
    i32.sub
    local.set 6
    local.get 6
    global.set $__stack_pointer
    local.get 6
    local.get 2
    i32.store offset=8
    local.get 6
    local.get 3
    i32.store offset=12
    local.get 6
    i32.load offset=8
    local.set 7
    block  ;; label = @1
      block  ;; label = @2
        local.get 7
        br_if 0 (;@2;)
        br 1 (;@1;)
      end
      local.get 6
      i32.load offset=8
      local.set 8
      local.get 6
      i32.load offset=12
      local.set 9
      local.get 6
      local.get 8
      i32.store offset=16
      local.get 6
      local.get 9
      i32.store offset=20
      local.get 6
      i32.load offset=16
      local.set 10
      local.get 6
      i32.load offset=20
      local.set 11
      local.get 6
      local.get 11
      i32.store offset=28
      local.get 6
      i32.load offset=28
      local.set 12
      local.get 1
      local.get 10
      local.get 12
      call $__rust_dealloc
    end
    i32.const 32
    local.set 13
    local.get 6
    local.get 13
    i32.add
    local.set 14
    local.get 14
    global.set $__stack_pointer
    return)
  (func $_ZN5alloc5alloc8box_free17h7a2634ad5b93478fE (type 6) (param i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    local.set 2
    i32.const 48
    local.set 3
    local.get 2
    local.get 3
    i32.sub
    local.set 4
    local.get 4
    global.set $__stack_pointer
    local.get 4
    local.get 0
    i32.store
    local.get 4
    local.get 1
    i32.store offset=4
    local.get 4
    i32.load offset=4
    local.set 5
    local.get 5
    i32.load offset=4
    local.set 6
    local.get 4
    local.get 6
    i32.store offset=40
    local.get 4
    i32.load offset=40
    local.set 7
    local.get 4
    i32.load offset=4
    local.set 8
    local.get 8
    i32.load offset=8
    local.set 9
    local.get 4
    local.get 9
    i32.store offset=44
    local.get 4
    i32.load offset=44
    local.set 10
    local.get 4
    local.get 7
    i32.store offset=16
    local.get 4
    local.get 10
    i32.store offset=20
    local.get 4
    i32.load
    local.set 11
    local.get 4
    local.get 11
    i32.store offset=36
    local.get 4
    i32.load offset=36
    local.set 12
    local.get 4
    local.get 12
    i32.store offset=32
    local.get 4
    i32.load offset=32
    local.set 13
    local.get 4
    local.get 13
    i32.store offset=28
    local.get 4
    i32.load offset=28
    local.set 14
    local.get 4
    i32.load offset=16
    local.set 15
    local.get 4
    i32.load offset=20
    local.set 16
    i32.const 8
    local.set 17
    local.get 4
    local.get 17
    i32.add
    local.set 18
    local.get 18
    local.set 19
    local.get 19
    local.get 14
    local.get 15
    local.get 16
    call $_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h72169ff628d8bf71E
    i32.const 48
    local.set 20
    local.get 4
    local.get 20
    i32.add
    local.set 21
    local.get 21
    global.set $__stack_pointer
    return)
  (func $_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h3be4647e131b3e8bE (type 1) (param i32)
    local.get 0
    call $_ZN4core3ops8function6FnOnce9call_once17h95e3bbfa0666b3d7E
    return)
  (func $_ZN3std2rt10lang_start17h143f8e995e466386E (type 8) (param i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    local.set 4
    i32.const 16
    local.set 5
    local.get 4
    local.get 5
    i32.sub
    local.set 6
    local.get 6
    global.set $__stack_pointer
    local.get 6
    local.get 0
    i32.store offset=12
    i32.const 12
    local.set 7
    local.get 6
    local.get 7
    i32.add
    local.set 8
    local.get 8
    local.set 9
    i32.const 1048672
    local.set 10
    local.get 9
    local.get 10
    local.get 1
    local.get 2
    local.get 3
    call $_ZN3std2rt19lang_start_internal17hf59d11c495a5d34cE
    local.set 11
    local.get 6
    local.get 11
    i32.store offset=8
    local.get 6
    i32.load offset=8
    local.set 12
    i32.const 16
    local.set 13
    local.get 6
    local.get 13
    i32.add
    local.set 14
    local.get 14
    global.set $__stack_pointer
    local.get 12
    return)
  (func $_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h1f94b09a6ec043aaE (type 4) (param i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    local.set 1
    i32.const 16
    local.set 2
    local.get 1
    local.get 2
    i32.sub
    local.set 3
    local.get 3
    global.set $__stack_pointer
    local.get 0
    i32.load
    local.set 4
    local.get 4
    call $_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h3be4647e131b3e8bE
    call $_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17had63d9c7cc120b0cE
    local.set 5
    i32.const 1
    local.set 6
    local.get 5
    local.get 6
    i32.and
    local.set 7
    local.get 3
    local.get 7
    i32.store8 offset=15
    i32.const 15
    local.set 8
    local.get 3
    local.get 8
    i32.add
    local.set 9
    local.get 9
    local.set 10
    local.get 10
    call $_ZN3std3sys4wasi7process8ExitCode6as_i3217hc69c91814d2ce79fE
    local.set 11
    i32.const 16
    local.set 12
    local.get 3
    local.get 12
    i32.add
    local.set 13
    local.get 13
    global.set $__stack_pointer
    local.get 11
    return)
  (func $_ZN3std2fs11OpenOptions4open17hbf0b4d7463c70648E (type 11) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    local.set 4
    i32.const 16
    local.set 5
    local.get 4
    local.get 5
    i32.sub
    local.set 6
    local.get 6
    global.set $__stack_pointer
    local.get 6
    local.get 2
    i32.store offset=8
    local.get 6
    local.get 3
    i32.store offset=12
    i32.const 8
    local.set 7
    local.get 6
    local.get 7
    i32.add
    local.set 8
    local.get 6
    local.get 8
    call $_ZN55_$LT$$RF$T$u20$as$u20$core..convert..AsRef$LT$U$GT$$GT$6as_ref17hb0a2da4d62f79317E
    local.get 6
    i32.load offset=4
    local.set 9
    local.get 6
    i32.load
    local.set 10
    local.get 0
    local.get 1
    local.get 10
    local.get 9
    call $_ZN3std2fs11OpenOptions5_open17hd9343526833faf36E
    i32.const 16
    local.set 11
    local.get 6
    local.get 11
    i32.add
    local.set 12
    local.get 12
    global.set $__stack_pointer
    return)
  (func $_ZN3std2fs4File6create17hb303bb79fb1817f3E (type 5) (param i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    local.set 3
    i32.const 64
    local.set 4
    local.get 3
    local.get 4
    i32.sub
    local.set 5
    local.get 5
    global.set $__stack_pointer
    local.get 5
    local.get 1
    i32.store offset=8
    local.get 5
    local.get 2
    i32.store offset=12
    i32.const 16
    local.set 6
    local.get 5
    local.get 6
    i32.add
    local.set 7
    local.get 7
    local.set 8
    local.get 8
    call $_ZN3std2fs11OpenOptions3new17h9b06b1aa8da3f811E
    i32.const 16
    local.set 9
    local.get 5
    local.get 9
    i32.add
    local.set 10
    local.get 10
    local.set 11
    i32.const 1
    local.set 12
    i32.const 1
    local.set 13
    local.get 12
    local.get 13
    i32.and
    local.set 14
    local.get 11
    local.get 14
    call $_ZN3std2fs11OpenOptions5write17h09aac53a2280d979E
    local.set 15
    i32.const 1
    local.set 16
    i32.const 1
    local.set 17
    local.get 16
    local.get 17
    i32.and
    local.set 18
    local.get 15
    local.get 18
    call $_ZN3std2fs11OpenOptions6create17hc7f26e82b4d268a3E
    local.set 19
    i32.const 1
    local.set 20
    i32.const 1
    local.set 21
    local.get 20
    local.get 21
    i32.and
    local.set 22
    local.get 19
    local.get 22
    call $_ZN3std2fs11OpenOptions8truncate17hdf1d682976bc01f6E
    local.set 23
    i32.const 8
    local.set 24
    local.get 5
    local.get 24
    i32.add
    local.set 25
    local.get 5
    local.get 25
    call $_ZN55_$LT$$RF$T$u20$as$u20$core..convert..AsRef$LT$U$GT$$GT$6as_ref17h5143b396fe4ad64fE
    local.get 5
    i32.load offset=4
    local.set 26
    local.get 5
    i32.load
    local.set 27
    local.get 0
    local.get 23
    local.get 27
    local.get 26
    call $_ZN3std2fs11OpenOptions4open17hbf0b4d7463c70648E
    i32.const 64
    local.set 28
    local.get 5
    local.get 28
    i32.add
    local.set 29
    local.get 29
    global.set $__stack_pointer
    return)
  (func $_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h32ac9f366558f692E (type 4) (param i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    local.set 1
    i32.const 16
    local.set 2
    local.get 1
    local.get 2
    i32.sub
    local.set 3
    local.get 3
    global.set $__stack_pointer
    local.get 0
    i32.load
    local.set 4
    local.get 4
    call $_ZN4core3ops8function6FnOnce9call_once17hdd6fcddf1672faa4E
    local.set 5
    i32.const 16
    local.set 6
    local.get 3
    local.get 6
    i32.add
    local.set 7
    local.get 7
    global.set $__stack_pointer
    local.get 5
    return)
  (func $_ZN4core3ops8function6FnOnce9call_once17hdd6fcddf1672faa4E (type 4) (param i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    local.set 1
    i32.const 16
    local.set 2
    local.get 1
    local.get 2
    i32.sub
    local.set 3
    local.get 3
    global.set $__stack_pointer
    local.get 3
    local.get 0
    i32.store offset=4
    i32.const 4
    local.set 4
    local.get 3
    local.get 4
    i32.add
    local.set 5
    local.get 5
    local.set 6
    local.get 6
    call $_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h1f94b09a6ec043aaE
    local.set 7
    i32.const 16
    local.set 8
    local.get 3
    local.get 8
    i32.add
    local.set 9
    local.get 9
    global.set $__stack_pointer
    local.get 7
    return)
  (func $_ZN4core3ops8function6FnOnce9call_once17h95e3bbfa0666b3d7E (type 1) (param i32)
    (local i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    local.set 1
    i32.const 16
    local.set 2
    local.get 1
    local.get 2
    i32.sub
    local.set 3
    local.get 3
    global.set $__stack_pointer
    local.get 0
    call_indirect (type 0)
    i32.const 16
    local.set 4
    local.get 3
    local.get 4
    i32.add
    local.set 5
    local.get 5
    global.set $__stack_pointer
    return)
  (func $_ZN4core3ptr101drop_in_place$LT$std..io..error..ErrorData$LT$alloc..boxed..Box$LT$std..io..error..Custom$GT$$GT$$GT$17hb69078d9cc435f79E (type 1) (param i32)
    (local i32 i32 i32 i32 i32)
    local.get 0
    i32.load8_u
    local.set 1
    i32.const 3
    local.set 2
    local.get 1
    local.get 2
    i32.lt_u
    local.set 3
    block  ;; label = @1
      local.get 3
      br_if 0 (;@1;)
      i32.const 4
      local.set 4
      local.get 0
      local.get 4
      i32.add
      local.set 5
      local.get 5
      call $_ZN4core3ptr68drop_in_place$LT$alloc..boxed..Box$LT$std..io..error..Custom$GT$$GT$17h25c6173c277858baE
    end
    return)
  (func $_ZN4core3ptr68drop_in_place$LT$alloc..boxed..Box$LT$std..io..error..Custom$GT$$GT$17h25c6173c277858baE (type 1) (param i32)
    (local i32 i32)
    local.get 0
    i32.load
    local.set 1
    local.get 1
    call $_ZN4core3ptr43drop_in_place$LT$std..io..error..Custom$GT$17h1e7c20f5c98cba72E
    local.get 0
    i32.load
    local.set 2
    local.get 2
    call $_ZN5alloc5alloc8box_free17h67caeca95fed6a6cE
    return)
  (func $_ZN4core3ptr118drop_in_place$LT$alloc..boxed..Box$LT$dyn$u20$core..error..Error$u2b$core..marker..Send$u2b$core..marker..Sync$GT$$GT$17hfbf6a84ae49e3dd6E (type 1) (param i32)
    (local i32 i32 i32 i32 i32)
    local.get 0
    i32.load
    local.set 1
    local.get 0
    i32.load offset=4
    local.set 2
    local.get 2
    i32.load
    local.set 3
    local.get 1
    local.get 3
    call_indirect (type 1)
    local.get 0
    i32.load
    local.set 4
    local.get 0
    i32.load offset=4
    local.set 5
    local.get 4
    local.get 5
    call $_ZN5alloc5alloc8box_free17h7a2634ad5b93478fE
    return)
  (func $_ZN4core3ptr34drop_in_place$LT$std..fs..File$GT$17h250bb08874e2cc92E (type 1) (param i32)
    local.get 0
    call $_ZN4core3ptr45drop_in_place$LT$std..sys..wasi..fs..File$GT$17heec58d353fa4b533E
    return)
  (func $_ZN4core3ptr45drop_in_place$LT$std..sys..wasi..fs..File$GT$17heec58d353fa4b533E (type 1) (param i32)
    local.get 0
    call $_ZN4core3ptr47drop_in_place$LT$std..sys..wasi..fd..WasiFd$GT$17h2a0d7d27fbfc50abE
    return)
  (func $_ZN4core3ptr42drop_in_place$LT$std..io..error..Error$GT$17h10101e7c97466622E (type 1) (param i32)
    local.get 0
    call $_ZN4core3ptr56drop_in_place$LT$std..io..error..repr_unpacked..Repr$GT$17h14fb891f355f1ab2E
    return)
  (func $_ZN4core3ptr56drop_in_place$LT$std..io..error..repr_unpacked..Repr$GT$17h14fb891f355f1ab2E (type 1) (param i32)
    local.get 0
    call $_ZN4core3ptr101drop_in_place$LT$std..io..error..ErrorData$LT$alloc..boxed..Box$LT$std..io..error..Custom$GT$$GT$$GT$17hb69078d9cc435f79E
    return)
  (func $_ZN4core3ptr43drop_in_place$LT$std..io..error..Custom$GT$17h1e7c20f5c98cba72E (type 1) (param i32)
    local.get 0
    call $_ZN4core3ptr118drop_in_place$LT$alloc..boxed..Box$LT$dyn$u20$core..error..Error$u2b$core..marker..Send$u2b$core..marker..Sync$GT$$GT$17hfbf6a84ae49e3dd6E
    return)
  (func $_ZN4core3ptr47drop_in_place$LT$std..sys..wasi..fd..WasiFd$GT$17h2a0d7d27fbfc50abE (type 1) (param i32)
    local.get 0
    call $_ZN4core3ptr48drop_in_place$LT$std..os..fd..owned..OwnedFd$GT$17h8cc8dbc10f51f8f4E
    return)
  (func $_ZN4core3ptr48drop_in_place$LT$std..os..fd..owned..OwnedFd$GT$17h8cc8dbc10f51f8f4E (type 1) (param i32)
    local.get 0
    call $_ZN69_$LT$std..os..fd..owned..OwnedFd$u20$as$u20$core..ops..drop..Drop$GT$4drop17he6169dc63e72538bE
    return)
  (func $_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h176bff3dde3ce504E (type 1) (param i32)
    return)
  (func $__rust_alloc (type 2) (param i32 i32) (result i32)
    (local i32)
    local.get 0
    local.get 1
    call $__rdl_alloc
    local.set 2
    local.get 2
    return)
  (func $__rust_dealloc (type 5) (param i32 i32 i32)
    local.get 0
    local.get 1
    local.get 2
    call $__rdl_dealloc
    return)
  (func $__rust_realloc (type 8) (param i32 i32 i32 i32) (result i32)
    (local i32)
    local.get 0
    local.get 1
    local.get 2
    local.get 3
    call $__rdl_realloc
    local.set 4
    local.get 4
    return)
  (func $__rust_alloc_error_handler (type 6) (param i32 i32)
    local.get 0
    local.get 1
    call $__rg_oom
    return)
  (func $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h55be51e7bf83a042E (type 3) (param i32) (result i64)
    i64.const -5213601185211584148)
  (func $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h77d2e9ebd7fed1efE (type 3) (param i32) (result i64)
    i64.const 5721260557098008181)
  (func $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17hc26280eb2ec5087fE (type 3) (param i32) (result i64)
    i64.const -4493808902380553279)
  (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h6c78f7e2c05482edE (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 1
    call $_ZN43_$LT$bool$u20$as$u20$core..fmt..Display$GT$3fmt17h3b22d81fd764e498E)
  (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h7015614129a78a15E (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 0
    i32.load offset=4
    local.get 1
    call $_ZN40_$LT$str$u20$as$u20$core..fmt..Debug$GT$3fmt17hab99d706fe6e9e7bE)
  (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h748212c8b37ccc7cE (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.set 0
    block  ;; label = @1
      local.get 1
      call $_ZN4core3fmt9Formatter15debug_lower_hex17h4338310221a5ab5bE
      br_if 0 (;@1;)
      block  ;; label = @2
        local.get 1
        call $_ZN4core3fmt9Formatter15debug_upper_hex17hba00f3ad3b67e305E
        br_if 0 (;@2;)
        local.get 0
        local.get 1
        call $_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17hf5ba33fe2dd172e1E
        return
      end
      local.get 0
      local.get 1
      call $_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i32$GT$3fmt17hf603227db3da36cbE
      return
    end
    local.get 0
    local.get 1
    call $_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i32$GT$3fmt17h67ac2fbea4a9fd5fE)
  (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h7501dcf2f1cfc31fE (type 2) (param i32 i32) (result i32)
    local.get 1
    i32.const 1048756
    i32.const 2
    call $_ZN4core3fmt9Formatter3pad17hea5c16334677f526E)
  (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h7c5b97921bca430bE (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 0
    i32.load offset=4
    local.get 1
    call $_ZN59_$LT$core..ffi..c_str..CStr$u20$as$u20$core..fmt..Debug$GT$3fmt17hdf615f64c1fe01a9E)
  (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h8102055cf4895771E (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    i32.load
    local.get 1
    call $_ZN64_$LT$alloc..ffi..c_str..NulError$u20$as$u20$core..fmt..Debug$GT$3fmt17h2161d80c8a9f6f64E)
  (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17ha5cca69876c36c70E (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.tee 0
    i32.load
    local.get 1
    local.get 0
    i32.const 4
    i32.add
    i32.load
    i32.load offset=12
    call_indirect (type 2))
  (func $_ZN3std2io5error82_$LT$impl$u20$core..fmt..Debug$u20$for$u20$std..io..error..repr_unpacked..Repr$GT$3fmt17h3b4105f9b47ac617E (type 2) (param i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 0
              i32.load8_u
              br_table 0 (;@5;) 1 (;@4;) 2 (;@3;) 3 (;@2;) 0 (;@5;)
            end
            local.get 2
            local.get 0
            i32.load offset=4
            i32.store offset=12
            local.get 2
            i32.const 16
            i32.add
            local.get 1
            i32.const 1049492
            i32.const 2
            call $_ZN4core3fmt9Formatter12debug_struct17hd068653c6d1e2675E
            local.get 2
            i32.const 16
            i32.add
            i32.const 1049494
            i32.const 4
            local.get 2
            i32.const 12
            i32.add
            i32.const 1049500
            call $_ZN4core3fmt8builders11DebugStruct5field17hc48bc4b5ba431201E
            local.set 0
            local.get 2
            local.get 2
            i32.load offset=12
            call $_ZN3std3sys4wasi17decode_error_kind17h037df60161d49d56E
            i32.const 255
            i32.and
            i32.store8 offset=31
            local.get 0
            i32.const 1049441
            i32.const 4
            local.get 2
            i32.const 31
            i32.add
            i32.const 1049448
            call $_ZN4core3fmt8builders11DebugStruct5field17hc48bc4b5ba431201E
            local.set 0
            local.get 2
            i32.const 32
            i32.add
            local.get 2
            i32.load offset=12
            call $_ZN3std3sys4wasi2os12error_string17h74596da84708f3a6E
            local.get 0
            i32.const 1049464
            i32.const 7
            local.get 2
            i32.const 32
            i32.add
            i32.const 1049516
            call $_ZN4core3fmt8builders11DebugStruct5field17hc48bc4b5ba431201E
            call $_ZN4core3fmt8builders11DebugStruct6finish17h3cefb3b4bef49d8cE
            local.set 0
            local.get 2
            i32.load offset=32
            local.tee 1
            i32.eqz
            br_if 3 (;@1;)
            local.get 2
            i32.load offset=36
            local.get 1
            i32.const 1
            call $__rust_dealloc
            br 3 (;@1;)
          end
          local.get 2
          local.get 0
          i32.load8_u offset=1
          i32.store8 offset=16
          local.get 2
          i32.const 32
          i32.add
          local.get 1
          i32.const 1049488
          i32.const 4
          call $_ZN4core3fmt9Formatter11debug_tuple17he1975a2c89512b73E
          local.get 2
          i32.const 32
          i32.add
          local.get 2
          i32.const 16
          i32.add
          i32.const 1049448
          call $_ZN4core3fmt8builders10DebugTuple5field17hbd92e7b522abdb91E
          call $_ZN4core3fmt8builders10DebugTuple6finish17hda7cfb178db66c11E
          local.set 0
          br 2 (;@1;)
        end
        local.get 0
        i32.load offset=4
        local.set 0
        local.get 2
        i32.const 32
        i32.add
        local.get 1
        i32.const 1049436
        i32.const 5
        call $_ZN4core3fmt9Formatter12debug_struct17hd068653c6d1e2675E
        local.get 2
        i32.const 32
        i32.add
        i32.const 1049441
        i32.const 4
        local.get 0
        i32.const 8
        i32.add
        i32.const 1049448
        call $_ZN4core3fmt8builders11DebugStruct5field17hc48bc4b5ba431201E
        i32.const 1049464
        i32.const 7
        local.get 0
        i32.const 1049472
        call $_ZN4core3fmt8builders11DebugStruct5field17hc48bc4b5ba431201E
        call $_ZN4core3fmt8builders11DebugStruct6finish17h3cefb3b4bef49d8cE
        local.set 0
        br 1 (;@1;)
      end
      local.get 2
      local.get 0
      i32.load offset=4
      local.tee 0
      i32.store offset=32
      local.get 1
      i32.const 1051259
      i32.const 6
      i32.const 1049441
      i32.const 4
      local.get 0
      i32.const 8
      i32.add
      i32.const 1049448
      i32.const 1051265
      i32.const 5
      local.get 2
      i32.const 32
      i32.add
      i32.const 1051272
      call $_ZN4core3fmt9Formatter26debug_struct_field2_finish17hdae384ded0a9d4beE
      local.set 0
    end
    local.get 2
    i32.const 48
    i32.add
    global.set $__stack_pointer
    local.get 0)
  (func $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h1d1cce09ac4fc977E (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 0
    i32.load offset=4
    local.get 1
    call $_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17haca8e2a25628fb8aE)
  (func $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h22b17f88a102613aE (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 1
    call $_ZN70_$LT$core..panic..location..Location$u20$as$u20$core..fmt..Display$GT$3fmt17h606141a768b76557E)
  (func $_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$i32$GT$3fmt17h363321e40170dea2E (type 2) (param i32 i32) (result i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        call $_ZN4core3fmt9Formatter15debug_lower_hex17h4338310221a5ab5bE
        br_if 0 (;@2;)
        local.get 1
        call $_ZN4core3fmt9Formatter15debug_upper_hex17hba00f3ad3b67e305E
        br_if 1 (;@1;)
        local.get 0
        local.get 1
        call $_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17hf5ba33fe2dd172e1E
        return
      end
      local.get 0
      local.get 1
      call $_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i32$GT$3fmt17h67ac2fbea4a9fd5fE
      return
    end
    local.get 0
    local.get 1
    call $_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i32$GT$3fmt17hf603227db3da36cbE)
  (func $_ZN4core3fmt5Write10write_char17h1dbd9303b03125e4E (type 2) (param i32 i32) (result i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    i32.const 0
    i32.store offset=12
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.const 128
        i32.lt_u
        br_if 0 (;@2;)
        block  ;; label = @3
          local.get 1
          i32.const 2048
          i32.lt_u
          br_if 0 (;@3;)
          block  ;; label = @4
            local.get 1
            i32.const 65536
            i32.lt_u
            br_if 0 (;@4;)
            local.get 2
            local.get 1
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=15
            local.get 2
            local.get 1
            i32.const 6
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=14
            local.get 2
            local.get 1
            i32.const 12
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=13
            local.get 2
            local.get 1
            i32.const 18
            i32.shr_u
            i32.const 7
            i32.and
            i32.const 240
            i32.or
            i32.store8 offset=12
            i32.const 4
            local.set 3
            br 3 (;@1;)
          end
          local.get 2
          local.get 1
          i32.const 63
          i32.and
          i32.const 128
          i32.or
          i32.store8 offset=14
          local.get 2
          local.get 1
          i32.const 12
          i32.shr_u
          i32.const 224
          i32.or
          i32.store8 offset=12
          local.get 2
          local.get 1
          i32.const 6
          i32.shr_u
          i32.const 63
          i32.and
          i32.const 128
          i32.or
          i32.store8 offset=13
          i32.const 3
          local.set 3
          br 2 (;@1;)
        end
        local.get 2
        local.get 1
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=13
        local.get 2
        local.get 1
        i32.const 6
        i32.shr_u
        i32.const 192
        i32.or
        i32.store8 offset=12
        i32.const 2
        local.set 3
        br 1 (;@1;)
      end
      local.get 2
      local.get 1
      i32.store8 offset=12
      i32.const 1
      local.set 3
    end
    block  ;; label = @1
      local.get 0
      i32.load offset=8
      local.tee 1
      i32.load
      local.get 1
      i32.load offset=8
      local.tee 0
      i32.sub
      local.get 3
      i32.ge_u
      br_if 0 (;@1;)
      local.get 1
      local.get 0
      local.get 3
      call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17h8af0b8a49dc47a67E
      local.get 1
      i32.load offset=8
      local.set 0
    end
    local.get 1
    i32.load offset=4
    local.get 0
    i32.add
    local.get 2
    i32.const 12
    i32.add
    local.get 3
    call $memcpy
    drop
    local.get 1
    local.get 0
    local.get 3
    i32.add
    i32.store offset=8
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    i32.const 0)
  (func $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17h8af0b8a49dc47a67E (type 5) (param i32 i32 i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        local.get 2
        i32.add
        local.tee 2
        local.get 1
        i32.lt_u
        br_if 0 (;@2;)
        local.get 0
        i32.load
        local.tee 1
        i32.const 1
        i32.shl
        local.tee 4
        local.get 2
        local.get 4
        local.get 2
        i32.gt_u
        select
        local.tee 2
        i32.const 8
        local.get 2
        i32.const 8
        i32.gt_u
        select
        local.tee 2
        i32.const -1
        i32.xor
        i32.const 31
        i32.shr_u
        local.set 4
        block  ;; label = @3
          block  ;; label = @4
            local.get 1
            i32.eqz
            br_if 0 (;@4;)
            local.get 3
            i32.const 1
            i32.store offset=24
            local.get 3
            local.get 1
            i32.store offset=20
            local.get 3
            local.get 0
            i32.const 4
            i32.add
            i32.load
            i32.store offset=16
            br 1 (;@3;)
          end
          local.get 3
          i32.const 0
          i32.store offset=24
        end
        local.get 3
        local.get 2
        local.get 4
        local.get 3
        i32.const 16
        i32.add
        call $_ZN5alloc7raw_vec11finish_grow17h9bb404aea4571a15E
        local.get 3
        i32.load offset=4
        local.set 1
        block  ;; label = @3
          local.get 3
          i32.load
          br_if 0 (;@3;)
          local.get 0
          local.get 2
          i32.store
          local.get 0
          local.get 1
          i32.store offset=4
          br 2 (;@1;)
        end
        local.get 3
        i32.const 8
        i32.add
        i32.load
        local.tee 0
        i32.const -2147483647
        i32.eq
        br_if 1 (;@1;)
        local.get 0
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        local.get 0
        call $_ZN5alloc5alloc18handle_alloc_error17h4888856d90202140E
        unreachable
      end
      call $_ZN5alloc7raw_vec17capacity_overflow17hbb754ab7ed70e9c1E
      unreachable
    end
    local.get 3
    i32.const 32
    i32.add
    global.set $__stack_pointer)
  (func $_ZN4core3fmt5Write10write_char17h5e18a2590c434ed3E (type 2) (param i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    i32.const 0
    i32.store offset=12
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.const 128
        i32.lt_u
        br_if 0 (;@2;)
        block  ;; label = @3
          local.get 1
          i32.const 2048
          i32.lt_u
          br_if 0 (;@3;)
          block  ;; label = @4
            local.get 1
            i32.const 65536
            i32.lt_u
            br_if 0 (;@4;)
            local.get 2
            local.get 1
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=15
            local.get 2
            local.get 1
            i32.const 6
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=14
            local.get 2
            local.get 1
            i32.const 12
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=13
            local.get 2
            local.get 1
            i32.const 18
            i32.shr_u
            i32.const 7
            i32.and
            i32.const 240
            i32.or
            i32.store8 offset=12
            i32.const 4
            local.set 1
            br 3 (;@1;)
          end
          local.get 2
          local.get 1
          i32.const 63
          i32.and
          i32.const 128
          i32.or
          i32.store8 offset=14
          local.get 2
          local.get 1
          i32.const 12
          i32.shr_u
          i32.const 224
          i32.or
          i32.store8 offset=12
          local.get 2
          local.get 1
          i32.const 6
          i32.shr_u
          i32.const 63
          i32.and
          i32.const 128
          i32.or
          i32.store8 offset=13
          i32.const 3
          local.set 1
          br 2 (;@1;)
        end
        local.get 2
        local.get 1
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=13
        local.get 2
        local.get 1
        i32.const 6
        i32.shr_u
        i32.const 192
        i32.or
        i32.store8 offset=12
        i32.const 2
        local.set 1
        br 1 (;@1;)
      end
      local.get 2
      local.get 1
      i32.store8 offset=12
      i32.const 1
      local.set 1
    end
    local.get 0
    local.get 2
    i32.const 12
    i32.add
    local.get 1
    call $_ZN80_$LT$std..io..Write..write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17h05a7908f2c5d636aE
    local.set 1
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 1)
  (func $_ZN80_$LT$std..io..Write..write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17h05a7908f2c5d636aE (type 7) (param i32 i32 i32) (result i32)
    (local i32 i32 i64 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    i32.const 0
    local.set 4
    block  ;; label = @1
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      i32.const 1049760
      i64.extend_i32_u
      i64.const 32
      i64.shl
      i64.const 2
      i64.or
      local.set 5
      loop  ;; label = @2
        local.get 3
        local.get 2
        i32.store offset=12
        local.get 3
        local.get 1
        i32.store offset=8
        local.get 3
        i32.const 16
        i32.add
        i32.const 2
        local.get 3
        i32.const 8
        i32.add
        i32.const 1
        call $_ZN4wasi13lib_generated8fd_write17h78b16dde3600cdd5E
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 3
                i32.load16_u offset=16
                br_if 0 (;@6;)
                local.get 3
                i32.load offset=20
                local.tee 6
                i32.eqz
                br_if 1 (;@5;)
                local.get 2
                local.get 6
                i32.lt_u
                br_if 2 (;@4;)
                local.get 1
                local.get 6
                i32.add
                local.set 1
                local.get 2
                local.get 6
                i32.sub
                local.set 2
                br 3 (;@3;)
              end
              local.get 3
              local.get 3
              i32.load16_u offset=18
              i32.store16 offset=30
              local.get 3
              i32.const 30
              i32.add
              call $_ZN4wasi13lib_generated5Errno3raw17h169d0715fcc2f361E
              local.tee 6
              i32.const 65535
              i32.and
              call $_ZN3std3sys4wasi17decode_error_kind17h037df60161d49d56E
              i32.const 255
              i32.and
              i32.const 35
              i32.eq
              br_if 2 (;@3;)
              local.get 6
              i64.extend_i32_u
              i64.const 65535
              i64.and
              i64.const 32
              i64.shl
              local.set 5
            end
            local.get 5
            i64.const 255
            i64.and
            i64.const 4
            i64.eq
            br_if 3 (;@1;)
            block  ;; label = @5
              local.get 0
              i32.load8_u
              i32.const 3
              i32.ne
              br_if 0 (;@5;)
              local.get 0
              i32.load offset=4
              local.tee 2
              i32.load
              local.get 2
              i32.load offset=4
              i32.load
              call_indirect (type 1)
              block  ;; label = @6
                local.get 2
                i32.load offset=4
                local.tee 1
                i32.const 4
                i32.add
                i32.load
                local.tee 6
                i32.eqz
                br_if 0 (;@6;)
                local.get 2
                i32.load
                local.get 6
                local.get 1
                i32.const 8
                i32.add
                i32.load
                call $__rust_dealloc
              end
              local.get 2
              i32.const 12
              i32.const 4
              call $__rust_dealloc
            end
            local.get 0
            local.get 5
            i64.store align=4
            i32.const 1
            local.set 4
            br 3 (;@1;)
          end
          local.get 6
          local.get 2
          i32.const 1049716
          call $_ZN4core5slice5index26slice_start_index_len_fail17h12ea13b84dc5aa33E
          unreachable
        end
        local.get 2
        br_if 0 (;@2;)
      end
    end
    local.get 3
    i32.const 32
    i32.add
    global.set $__stack_pointer
    local.get 4)
  (func $_ZN4core3fmt5Write9write_fmt17h561b73a22a7a7b11E (type 2) (param i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    local.get 0
    i32.store offset=4
    local.get 2
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    local.get 1
    i64.load align=4
    i64.store offset=8
    local.get 2
    i32.const 4
    i32.add
    i32.const 1048760
    local.get 2
    i32.const 8
    i32.add
    call $_ZN4core3fmt5write17h56e014c9dc514616E
    local.set 1
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
    local.get 1)
  (func $_ZN4core3fmt5Write9write_fmt17hac4919787652327dE (type 2) (param i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    local.get 0
    i32.store offset=4
    local.get 2
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    local.get 1
    i64.load align=4
    i64.store offset=8
    local.get 2
    i32.const 4
    i32.add
    i32.const 1048784
    local.get 2
    i32.const 8
    i32.add
    call $_ZN4core3fmt5write17h56e014c9dc514616E
    local.set 1
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
    local.get 1)
  (func $_ZN3std9panicking12default_hook17hb9a783738c35b420E (type 1) (param i32)
    (local i32 i32 i32 i64 i32)
    global.get $__stack_pointer
    i32.const 96
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    i32.const 1
    local.set 2
    block  ;; label = @1
      i32.const 0
      i32.load offset=1059156
      i32.const 1
      i32.gt_u
      br_if 0 (;@1;)
      call $_ZN3std5panic19get_backtrace_style17h7338251cd2c38d6bE
      i32.const 255
      i32.and
      local.set 2
    end
    local.get 1
    local.get 2
    i32.store8 offset=19
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 0
            call $_ZN4core5panic10panic_info9PanicInfo8location17hac97990c2b530673E
            local.tee 2
            i32.eqz
            br_if 0 (;@4;)
            local.get 1
            local.get 2
            i32.store offset=20
            local.get 1
            i32.const 8
            i32.add
            local.get 0
            call $_ZN4core5panic10panic_info9PanicInfo7payload17h9a735917efe7934bE
            i32.const 12
            local.set 3
            local.get 1
            i32.load offset=8
            local.tee 2
            local.get 1
            i32.load offset=12
            i32.const 12
            i32.add
            i32.load
            call_indirect (type 3)
            local.set 4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 2
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 4
                    i64.const -4493808902380553279
                    i64.eq
                    br_if 1 (;@7;)
                  end
                  local.get 1
                  local.get 0
                  call $_ZN4core5panic10panic_info9PanicInfo7payload17h9a735917efe7934bE
                  i32.const 1050584
                  local.set 0
                  local.get 1
                  i32.load
                  local.tee 2
                  local.get 1
                  i32.load offset=4
                  i32.const 12
                  i32.add
                  i32.load
                  call_indirect (type 3)
                  local.set 4
                  local.get 2
                  i32.eqz
                  br_if 2 (;@5;)
                  local.get 4
                  i64.const 5721260557098008181
                  i64.ne
                  br_if 2 (;@5;)
                  local.get 2
                  i32.const 8
                  i32.add
                  local.set 0
                  local.get 2
                  i32.const 4
                  i32.add
                  local.set 2
                  br 1 (;@6;)
                end
                local.get 2
                i32.const 4
                i32.add
                local.set 0
              end
              local.get 0
              i32.load
              local.set 3
              local.get 2
              i32.load
              local.set 0
            end
            local.get 1
            local.get 3
            i32.store offset=28
            local.get 1
            local.get 0
            i32.store offset=24
            i32.const 9
            local.set 2
            i32.const 1050596
            local.set 3
            block  ;; label = @5
              call $_ZN3std10sys_common11thread_info14current_thread17h11644fc46f8bf6b5E
              local.tee 0
              i32.eqz
              br_if 0 (;@5;)
              local.get 0
              i32.load offset=8
              local.tee 5
              i32.eqz
              br_if 0 (;@5;)
              local.get 0
              i32.const 12
              i32.add
              i32.load
              i32.const -1
              i32.add
              local.set 2
              local.get 5
              local.set 3
            end
            local.get 1
            local.get 2
            i32.store offset=36
            local.get 1
            local.get 3
            i32.store offset=32
            local.get 1
            local.get 1
            i32.const 19
            i32.add
            i32.store offset=52
            local.get 1
            local.get 1
            i32.const 20
            i32.add
            i32.store offset=48
            local.get 1
            local.get 1
            i32.const 24
            i32.add
            i32.store offset=44
            local.get 1
            local.get 1
            i32.const 32
            i32.add
            i32.store offset=40
            i32.const 0
            i32.load8_u offset=1059054
            i32.eqz
            br_if 2 (;@2;)
            i32.const 0
            i32.const 1
            i32.store8 offset=1059054
            block  ;; label = @5
              i32.const 0
              i32.load8_u offset=1059136
              br_if 0 (;@5;)
              i32.const 0
              i32.const 1
              i32.store8 offset=1059136
              i32.const 0
              i32.const 0
              i32.store offset=1059140
              br 3 (;@2;)
            end
            i32.const 0
            i32.load offset=1059140
            local.set 2
            i32.const 0
            i32.const 0
            i32.store offset=1059140
            local.get 2
            i32.eqz
            br_if 2 (;@2;)
            local.get 2
            i32.load8_u offset=8
            local.set 3
            local.get 2
            i32.const 1
            i32.store8 offset=8
            local.get 1
            local.get 3
            i32.store8 offset=63
            local.get 3
            br_if 1 (;@3;)
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  i32.const 0
                  i32.load offset=1059120
                  i32.const 2147483647
                  i32.and
                  i32.eqz
                  br_if 0 (;@7;)
                  call $_ZN3std9panicking11panic_count17is_zero_slow_path17h17f5a317560a2ca8E
                  local.set 3
                  local.get 1
                  i32.const 40
                  i32.add
                  local.get 2
                  i32.const 12
                  i32.add
                  i32.const 1050648
                  call $_ZN3std9panicking12default_hook28_$u7b$$u7b$closure$u7d$$u7d$17hf6bb6a273094adb0E
                  local.get 3
                  br_if 1 (;@6;)
                  br 2 (;@5;)
                end
                local.get 1
                i32.const 40
                i32.add
                local.get 2
                i32.const 12
                i32.add
                i32.const 1050648
                call $_ZN3std9panicking12default_hook28_$u7b$$u7b$closure$u7d$$u7d$17hf6bb6a273094adb0E
              end
              i32.const 0
              i32.load offset=1059120
              i32.const 2147483647
              i32.and
              i32.eqz
              br_if 0 (;@5;)
              call $_ZN3std9panicking11panic_count17is_zero_slow_path17h17f5a317560a2ca8E
              br_if 0 (;@5;)
              local.get 2
              i32.const 1
              i32.store8 offset=9
            end
            i32.const 0
            i32.const 1
            i32.store8 offset=1059054
            local.get 2
            i32.const 0
            i32.store8 offset=8
            block  ;; label = @5
              i32.const 0
              i32.load8_u offset=1059136
              br_if 0 (;@5;)
              i32.const 0
              local.get 2
              i32.store offset=1059140
              i32.const 0
              i32.const 1
              i32.store8 offset=1059136
              br 4 (;@1;)
            end
            i32.const 0
            i32.load offset=1059140
            local.set 3
            i32.const 0
            local.get 2
            i32.store offset=1059140
            local.get 3
            i32.eqz
            br_if 3 (;@1;)
            local.get 3
            local.get 3
            i32.load
            local.tee 2
            i32.const -1
            i32.add
            i32.store
            local.get 2
            i32.const 1
            i32.ne
            br_if 3 (;@1;)
            local.get 3
            call $_ZN5alloc4sync12Arc$LT$T$GT$9drop_slow17h91ba70c8683e2d07E
            br 3 (;@1;)
          end
          i32.const 1048868
          i32.const 43
          i32.const 1050688
          call $_ZN4core9panicking5panic17h33aac59f7456cb9eE
          unreachable
        end
        local.get 1
        i32.const 0
        i32.store offset=84
        local.get 1
        i32.const 1048832
        i32.store offset=80
        local.get 1
        i32.const 1
        i32.store offset=76
        local.get 1
        i32.const 1052056
        i32.store offset=72
        local.get 1
        i32.const 0
        i32.store offset=64
        local.get 1
        i32.const 63
        i32.add
        local.get 1
        i32.const 64
        i32.add
        call $_ZN4core9panicking13assert_failed17haed743e2be7b9a20E
        unreachable
      end
      local.get 1
      i32.const 40
      i32.add
      local.get 1
      i32.const 88
      i32.add
      i32.const 1050608
      call $_ZN3std9panicking12default_hook28_$u7b$$u7b$closure$u7d$$u7d$17hf6bb6a273094adb0E
    end
    block  ;; label = @1
      local.get 0
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      local.get 0
      i32.load
      local.tee 2
      i32.const -1
      i32.add
      i32.store
      local.get 2
      i32.const 1
      i32.ne
      br_if 0 (;@1;)
      local.get 0
      call $_ZN5alloc4sync12Arc$LT$T$GT$9drop_slow17h54811740ab234a11E
    end
    local.get 1
    i32.const 96
    i32.add
    global.set $__stack_pointer)
  (func $_ZN4core3ptr100drop_in_place$LT$$RF$mut$u20$std..io..Write..write_fmt..Adapter$LT$alloc..vec..Vec$LT$u8$GT$$GT$$GT$17h1052ad729efca941E (type 1) (param i32))
  (func $_ZN3std9panicking11panic_count17is_zero_slow_path17h17f5a317560a2ca8E (type 10) (result i32)
    i32.const 0
    i32.load offset=1059156
    i32.eqz)
  (func $_ZN4core3ptr205drop_in_place$LT$$LT$alloc..boxed..Box$LT$dyn$u20$core..error..Error$u2b$core..marker..Send$u2b$core..marker..Sync$GT$$u20$as$u20$core..convert..From$LT$alloc..string..String$GT$$GT$..from..StringError$GT$17h0a9c176b58215858E (type 1) (param i32)
    (local i32)
    block  ;; label = @1
      local.get 0
      i32.load
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.const 4
      i32.add
      i32.load
      local.get 1
      i32.const 1
      call $__rust_dealloc
    end)
  (func $_ZN5alloc4sync12Arc$LT$T$GT$9drop_slow17h54811740ab234a11E (type 1) (param i32)
    (local i32)
    block  ;; label = @1
      local.get 0
      i32.load offset=8
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      i32.const 0
      i32.store8
      local.get 0
      i32.const 12
      i32.add
      i32.load
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=8
      local.get 1
      i32.const 1
      call $__rust_dealloc
    end
    block  ;; label = @1
      local.get 0
      i32.const -1
      i32.eq
      br_if 0 (;@1;)
      local.get 0
      local.get 0
      i32.load offset=4
      local.tee 1
      i32.const -1
      i32.add
      i32.store offset=4
      local.get 1
      i32.const 1
      i32.ne
      br_if 0 (;@1;)
      local.get 0
      i32.const 32
      i32.const 8
      call $__rust_dealloc
    end)
  (func $_ZN4core3ptr70drop_in_place$LT$core..option..Option$LT$alloc..string..String$GT$$GT$17h493eaf1a04cd9ae5E (type 1) (param i32)
    (local i32)
    block  ;; label = @1
      local.get 0
      i32.const 4
      i32.add
      i32.load
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.load
      local.tee 0
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      local.get 0
      i32.const 1
      call $__rust_dealloc
    end)
  (func $_ZN4core3ptr81drop_in_place$LT$core..result..Result$LT$$LP$$RP$$C$std..io..error..Error$GT$$GT$17h578f989f6735c81dE (type 1) (param i32)
    (local i32 i32 i32)
    block  ;; label = @1
      local.get 0
      i32.load8_u
      i32.const 3
      i32.ne
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=4
      local.tee 1
      i32.load
      local.get 1
      i32.load offset=4
      i32.load
      call_indirect (type 1)
      block  ;; label = @2
        local.get 1
        i32.load offset=4
        local.tee 2
        i32.const 4
        i32.add
        i32.load
        local.tee 3
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        i32.load
        local.get 3
        local.get 2
        i32.const 8
        i32.add
        i32.load
        call $__rust_dealloc
      end
      local.get 0
      i32.load offset=4
      i32.const 12
      i32.const 4
      call $__rust_dealloc
    end)
  (func $_ZN4core3ptr88drop_in_place$LT$std..io..Write..write_fmt..Adapter$LT$alloc..vec..Vec$LT$u8$GT$$GT$$GT$17h5717bcf441afcf24E (type 1) (param i32)
    (local i32 i32 i32)
    block  ;; label = @1
      local.get 0
      i32.load8_u
      i32.const 3
      i32.ne
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=4
      local.tee 1
      i32.load
      local.get 1
      i32.load offset=4
      i32.load
      call_indirect (type 1)
      block  ;; label = @2
        local.get 1
        i32.load offset=4
        local.tee 2
        i32.const 4
        i32.add
        i32.load
        local.tee 3
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        i32.load
        local.get 3
        local.get 2
        i32.const 8
        i32.add
        i32.load
        call $__rust_dealloc
      end
      local.get 0
      i32.load offset=4
      i32.const 12
      i32.const 4
      call $__rust_dealloc
    end)
  (func $_ZN4core5error5Error5cause17hba07ba1b29ce5bffE (type 6) (param i32 i32)
    local.get 0
    i32.const 0
    i32.store)
  (func $_ZN4core5error5Error7provide17h0c60d749fab417ebE (type 5) (param i32 i32 i32))
  (func $_ZN4core5error5Error7type_id17h53f8e79ab2115a0eE (type 3) (param i32) (result i64)
    i64.const 310519794269800718)
  (func $_ZN4core9panicking13assert_failed17h56c36f852b6e67b2E (type 6) (param i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    i32.const 1051820
    i32.store offset=4
    local.get 2
    local.get 0
    i32.store
    local.get 2
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    local.get 1
    i64.load align=4
    i64.store offset=8
    i32.const 1
    local.get 2
    i32.const 1048836
    local.get 2
    i32.const 4
    i32.add
    i32.const 1048836
    local.get 2
    i32.const 8
    i32.add
    i32.const 1051824
    call $_ZN4core9panicking19assert_failed_inner17hc1caefd2be2cf3dcE
    unreachable)
  (func $_ZN4core9panicking13assert_failed17haed743e2be7b9a20E (type 6) (param i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    i32.const 1048832
    i32.store offset=4
    local.get 2
    local.get 0
    i32.store
    local.get 2
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    local.get 1
    i64.load align=4
    i64.store offset=8
    i32.const 0
    local.get 2
    i32.const 1048852
    local.get 2
    i32.const 4
    i32.add
    i32.const 1048852
    local.get 2
    i32.const 8
    i32.add
    i32.const 1052120
    call $_ZN4core9panicking19assert_failed_inner17hc1caefd2be2cf3dcE
    unreachable)
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h1085e6bde15e7786E (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 1
    call $_ZN4core3fmt5Write10write_char17h1dbd9303b03125e4E)
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h5a1a43da4a400424E (type 2) (param i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 0
    i32.load
    local.set 0
    local.get 2
    i32.const 0
    i32.store offset=12
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.const 128
        i32.lt_u
        br_if 0 (;@2;)
        block  ;; label = @3
          local.get 1
          i32.const 2048
          i32.lt_u
          br_if 0 (;@3;)
          block  ;; label = @4
            local.get 1
            i32.const 65536
            i32.lt_u
            br_if 0 (;@4;)
            local.get 2
            local.get 1
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=15
            local.get 2
            local.get 1
            i32.const 6
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=14
            local.get 2
            local.get 1
            i32.const 12
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=13
            local.get 2
            local.get 1
            i32.const 18
            i32.shr_u
            i32.const 7
            i32.and
            i32.const 240
            i32.or
            i32.store8 offset=12
            i32.const 4
            local.set 1
            br 3 (;@1;)
          end
          local.get 2
          local.get 1
          i32.const 63
          i32.and
          i32.const 128
          i32.or
          i32.store8 offset=14
          local.get 2
          local.get 1
          i32.const 12
          i32.shr_u
          i32.const 224
          i32.or
          i32.store8 offset=12
          local.get 2
          local.get 1
          i32.const 6
          i32.shr_u
          i32.const 63
          i32.and
          i32.const 128
          i32.or
          i32.store8 offset=13
          i32.const 3
          local.set 1
          br 2 (;@1;)
        end
        local.get 2
        local.get 1
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=13
        local.get 2
        local.get 1
        i32.const 6
        i32.shr_u
        i32.const 192
        i32.or
        i32.store8 offset=12
        i32.const 2
        local.set 1
        br 1 (;@1;)
      end
      local.get 2
      local.get 1
      i32.store8 offset=12
      i32.const 1
      local.set 1
    end
    local.get 0
    local.get 2
    i32.const 12
    i32.add
    local.get 1
    call $_ZN80_$LT$std..io..Write..write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17h05a7908f2c5d636aE
    local.set 1
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 1)
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h9212a16a243fbfafE (type 2) (param i32 i32) (result i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 0
    i32.load
    local.set 0
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.const 127
        i32.gt_u
        br_if 0 (;@2;)
        block  ;; label = @3
          local.get 0
          i32.load offset=8
          local.tee 3
          local.get 0
          i32.load
          i32.ne
          br_if 0 (;@3;)
          local.get 0
          local.get 3
          call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17h4a659729829988f5E
          local.get 0
          i32.load offset=8
          local.set 3
        end
        local.get 0
        local.get 3
        i32.const 1
        i32.add
        i32.store offset=8
        local.get 0
        i32.load offset=4
        local.get 3
        i32.add
        local.get 1
        i32.store8
        br 1 (;@1;)
      end
      local.get 2
      i32.const 0
      i32.store offset=12
      block  ;; label = @2
        block  ;; label = @3
          local.get 1
          i32.const 2048
          i32.lt_u
          br_if 0 (;@3;)
          block  ;; label = @4
            local.get 1
            i32.const 65536
            i32.lt_u
            br_if 0 (;@4;)
            local.get 2
            local.get 1
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=15
            local.get 2
            local.get 1
            i32.const 6
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=14
            local.get 2
            local.get 1
            i32.const 12
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=13
            local.get 2
            local.get 1
            i32.const 18
            i32.shr_u
            i32.const 7
            i32.and
            i32.const 240
            i32.or
            i32.store8 offset=12
            i32.const 4
            local.set 1
            br 2 (;@2;)
          end
          local.get 2
          local.get 1
          i32.const 63
          i32.and
          i32.const 128
          i32.or
          i32.store8 offset=14
          local.get 2
          local.get 1
          i32.const 12
          i32.shr_u
          i32.const 224
          i32.or
          i32.store8 offset=12
          local.get 2
          local.get 1
          i32.const 6
          i32.shr_u
          i32.const 63
          i32.and
          i32.const 128
          i32.or
          i32.store8 offset=13
          i32.const 3
          local.set 1
          br 1 (;@2;)
        end
        local.get 2
        local.get 1
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=13
        local.get 2
        local.get 1
        i32.const 6
        i32.shr_u
        i32.const 192
        i32.or
        i32.store8 offset=12
        i32.const 2
        local.set 1
      end
      block  ;; label = @2
        local.get 0
        i32.load
        local.get 0
        i32.load offset=8
        local.tee 3
        i32.sub
        local.get 1
        i32.ge_u
        br_if 0 (;@2;)
        local.get 0
        local.get 3
        local.get 1
        call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17h8af0b8a49dc47a67E
        local.get 0
        i32.load offset=8
        local.set 3
      end
      local.get 0
      i32.load offset=4
      local.get 3
      i32.add
      local.get 2
      i32.const 12
      i32.add
      local.get 1
      call $memcpy
      drop
      local.get 0
      local.get 3
      local.get 1
      i32.add
      i32.store offset=8
    end
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    i32.const 0)
  (func $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17h4a659729829988f5E (type 6) (param i32 i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.const 1
        i32.add
        local.tee 1
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        i32.load
        local.tee 3
        i32.const 1
        i32.shl
        local.tee 4
        local.get 1
        local.get 4
        local.get 1
        i32.gt_u
        select
        local.tee 1
        i32.const 8
        local.get 1
        i32.const 8
        i32.gt_u
        select
        local.tee 1
        i32.const -1
        i32.xor
        i32.const 31
        i32.shr_u
        local.set 4
        block  ;; label = @3
          block  ;; label = @4
            local.get 3
            i32.eqz
            br_if 0 (;@4;)
            local.get 2
            i32.const 1
            i32.store offset=24
            local.get 2
            local.get 3
            i32.store offset=20
            local.get 2
            local.get 0
            i32.const 4
            i32.add
            i32.load
            i32.store offset=16
            br 1 (;@3;)
          end
          local.get 2
          i32.const 0
          i32.store offset=24
        end
        local.get 2
        local.get 1
        local.get 4
        local.get 2
        i32.const 16
        i32.add
        call $_ZN5alloc7raw_vec11finish_grow17h9bb404aea4571a15E
        local.get 2
        i32.load offset=4
        local.set 3
        block  ;; label = @3
          local.get 2
          i32.load
          br_if 0 (;@3;)
          local.get 0
          local.get 1
          i32.store
          local.get 0
          local.get 3
          i32.store offset=4
          br 2 (;@1;)
        end
        local.get 2
        i32.const 8
        i32.add
        i32.load
        local.tee 0
        i32.const -2147483647
        i32.eq
        br_if 1 (;@1;)
        local.get 0
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        local.get 0
        call $_ZN5alloc5alloc18handle_alloc_error17h4888856d90202140E
        unreachable
      end
      call $_ZN5alloc7raw_vec17capacity_overflow17hbb754ab7ed70e9c1E
      unreachable
    end
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer)
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17h1e452089e8e4bbeaE (type 2) (param i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 0
    i32.load
    local.set 0
    local.get 2
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    local.get 1
    i64.load align=4
    i64.store offset=8
    local.get 2
    local.get 0
    i32.store offset=4
    local.get 2
    i32.const 4
    i32.add
    i32.const 1048760
    local.get 2
    i32.const 8
    i32.add
    call $_ZN4core3fmt5write17h56e014c9dc514616E
    local.set 1
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
    local.get 1)
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17h58488dcbcdd00066E (type 2) (param i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 0
    i32.load
    local.set 0
    local.get 2
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    local.get 1
    i64.load align=4
    i64.store offset=8
    local.get 2
    local.get 0
    i32.store offset=4
    local.get 2
    i32.const 4
    i32.add
    i32.const 1048808
    local.get 2
    i32.const 8
    i32.add
    call $_ZN4core3fmt5write17h56e014c9dc514616E
    local.set 1
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
    local.get 1)
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17hddfc5c6b607045dbE (type 2) (param i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 0
    i32.load
    local.set 0
    local.get 2
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    local.get 1
    i64.load align=4
    i64.store offset=8
    local.get 2
    local.get 0
    i32.store offset=4
    local.get 2
    i32.const 4
    i32.add
    i32.const 1048784
    local.get 2
    i32.const 8
    i32.add
    call $_ZN4core3fmt5write17h56e014c9dc514616E
    local.set 1
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
    local.get 1)
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17h33842eeb8c82131cE (type 7) (param i32 i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 1
    local.get 2
    call $_ZN80_$LT$std..io..Write..write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17h05a7908f2c5d636aE)
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17h763c09e7c5464cdeE (type 7) (param i32 i32 i32) (result i32)
    (local i32)
    block  ;; label = @1
      local.get 0
      i32.load
      i32.load offset=8
      local.tee 0
      i32.load
      local.get 0
      i32.load offset=8
      local.tee 3
      i32.sub
      local.get 2
      i32.ge_u
      br_if 0 (;@1;)
      local.get 0
      local.get 3
      local.get 2
      call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17h8af0b8a49dc47a67E
      local.get 0
      i32.load offset=8
      local.set 3
    end
    local.get 0
    i32.load offset=4
    local.get 3
    i32.add
    local.get 1
    local.get 2
    call $memcpy
    drop
    local.get 0
    local.get 3
    local.get 2
    i32.add
    i32.store offset=8
    i32.const 0)
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17he3cf788bc3442563E (type 7) (param i32 i32 i32) (result i32)
    (local i32)
    block  ;; label = @1
      local.get 0
      i32.load
      local.tee 0
      i32.load
      local.get 0
      i32.load offset=8
      local.tee 3
      i32.sub
      local.get 2
      i32.ge_u
      br_if 0 (;@1;)
      local.get 0
      local.get 3
      local.get 2
      call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17h8af0b8a49dc47a67E
      local.get 0
      i32.load offset=8
      local.set 3
    end
    local.get 0
    i32.load offset=4
    local.get 3
    i32.add
    local.get 1
    local.get 2
    call $memcpy
    drop
    local.get 0
    local.get 3
    local.get 2
    i32.add
    i32.store offset=8
    i32.const 0)
  (func $_ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Debug$GT$3fmt17h88d09d9d08f2cd1aE (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load offset=4
    local.get 0
    i32.load offset=8
    local.get 1
    call $_ZN40_$LT$str$u20$as$u20$core..fmt..Debug$GT$3fmt17hab99d706fe6e9e7bE)
  (func $_ZN5alloc4sync12Arc$LT$T$GT$9drop_slow17h91ba70c8683e2d07E (type 1) (param i32)
    (local i32)
    block  ;; label = @1
      local.get 0
      i32.const 12
      i32.add
      i32.load
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.const 16
      i32.add
      i32.load
      local.get 1
      i32.const 1
      call $__rust_dealloc
    end
    block  ;; label = @1
      local.get 0
      i32.const -1
      i32.eq
      br_if 0 (;@1;)
      local.get 0
      local.get 0
      i32.load offset=4
      local.tee 1
      i32.const -1
      i32.add
      i32.store offset=4
      local.get 1
      i32.const 1
      i32.ne
      br_if 0 (;@1;)
      local.get 0
      i32.const 24
      i32.const 4
      call $__rust_dealloc
    end)
  (func $_ZN5alloc7raw_vec11finish_grow17h9bb404aea4571a15E (type 11) (param i32 i32 i32 i32)
    (local i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        i32.eqz
        br_if 0 (;@2;)
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 1
                  i32.const -1
                  i32.le_s
                  br_if 0 (;@7;)
                  local.get 3
                  i32.load offset=8
                  i32.eqz
                  br_if 2 (;@5;)
                  local.get 3
                  i32.load offset=4
                  local.tee 4
                  br_if 1 (;@6;)
                  local.get 1
                  br_if 3 (;@4;)
                  local.get 2
                  local.set 3
                  br 4 (;@3;)
                end
                local.get 0
                i32.const 8
                i32.add
                i32.const 0
                i32.store
                br 5 (;@1;)
              end
              local.get 3
              i32.load
              local.get 4
              local.get 2
              local.get 1
              call $__rust_realloc
              local.set 3
              br 2 (;@3;)
            end
            local.get 1
            br_if 0 (;@4;)
            local.get 2
            local.set 3
            br 1 (;@3;)
          end
          local.get 1
          local.get 2
          call $__rust_alloc
          local.set 3
        end
        block  ;; label = @3
          local.get 3
          i32.eqz
          br_if 0 (;@3;)
          local.get 0
          local.get 3
          i32.store offset=4
          local.get 0
          i32.const 8
          i32.add
          local.get 1
          i32.store
          local.get 0
          i32.const 0
          i32.store
          return
        end
        local.get 0
        local.get 1
        i32.store offset=4
        local.get 0
        i32.const 8
        i32.add
        local.get 2
        i32.store
        br 1 (;@1;)
      end
      local.get 0
      local.get 1
      i32.store offset=4
      local.get 0
      i32.const 8
      i32.add
      i32.const 0
      i32.store
    end
    local.get 0
    i32.const 1
    i32.store)
  (func $_ZN70_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hc0ba9e9c7e604a19E (type 2) (param i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load
        br_if 0 (;@2;)
        local.get 2
        local.get 0
        i32.store offset=8
        local.get 1
        i32.const 1049047
        i32.const 2
        local.get 2
        i32.const 8
        i32.add
        i32.const 1049068
        call $_ZN4core3fmt9Formatter25debug_tuple_field1_finish17h5f6e45e01c2a3168E
        local.set 0
        br 1 (;@1;)
      end
      local.get 2
      local.get 0
      i32.store offset=12
      local.get 1
      i32.const 1049044
      i32.const 3
      local.get 2
      i32.const 12
      i32.add
      i32.const 1049052
      call $_ZN4core3fmt9Formatter25debug_tuple_field1_finish17h5f6e45e01c2a3168E
      local.set 0
    end
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 0)
  (func $_ZN3std3sys4wasi4once4Once4call17hd3dc100d98d62d15E (type 1) (param i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  i32.const 0
                  i32.load8_u offset=1059053
                  br_table 0 (;@7;) 2 (;@5;) 6 (;@1;) 1 (;@6;) 0 (;@7;)
                end
                i32.const 0
                i32.const 2
                i32.store8 offset=1059053
                local.get 0
                i32.load8_u
                local.set 2
                local.get 0
                i32.const 0
                i32.store8
                local.get 2
                i32.eqz
                br_if 2 (;@4;)
                local.get 1
                i32.const 0
                i32.store8 offset=31
                block  ;; label = @7
                  block  ;; label = @8
                    i32.const 0
                    i32.load8_u offset=1059088
                    i32.const 3
                    i32.eq
                    br_if 0 (;@8;)
                    local.get 1
                    i32.const 31
                    i32.add
                    call $_ZN3std4sync9once_lock17OnceLock$LT$T$GT$10initialize17hf3f033e66ad6a56aE
                    local.get 1
                    i32.load8_u offset=31
                    br_if 1 (;@7;)
                  end
                  block  ;; label = @8
                    block  ;; label = @9
                      i32.const 0
                      i32.load offset=1059056
                      i32.const 1059144
                      i32.eq
                      br_if 0 (;@9;)
                      i32.const 0
                      i32.load8_u offset=1059084
                      local.set 2
                      i32.const 1
                      local.set 0
                      i32.const 0
                      i32.const 1
                      i32.store8 offset=1059084
                      local.get 2
                      br_if 2 (;@7;)
                      i32.const 0
                      i32.const 1059144
                      i32.store offset=1059056
                      br 1 (;@8;)
                    end
                    i32.const 0
                    i32.load offset=1059060
                    i32.const 1
                    i32.add
                    local.tee 0
                    i32.eqz
                    br_if 5 (;@3;)
                  end
                  i32.const 0
                  local.get 0
                  i32.store offset=1059060
                  i32.const 0
                  i32.load offset=1059064
                  br_if 5 (;@2;)
                  i32.const 0
                  i32.const -1
                  i32.store offset=1059064
                  block  ;; label = @8
                    i32.const 0
                    i32.load8_u offset=1059080
                    br_if 0 (;@8;)
                    local.get 1
                    i32.const 1059068
                    call $_ZN3std2io8buffered9bufwriter18BufWriter$LT$W$GT$9flush_buf17h025b1526cfabd17eE
                    local.get 1
                    i32.load8_u
                    i32.const 3
                    i32.ne
                    br_if 0 (;@8;)
                    local.get 1
                    i32.load offset=4
                    local.tee 0
                    i32.load
                    local.get 0
                    i32.load offset=4
                    i32.load
                    call_indirect (type 1)
                    block  ;; label = @9
                      local.get 0
                      i32.load offset=4
                      local.tee 2
                      i32.const 4
                      i32.add
                      i32.load
                      local.tee 3
                      i32.eqz
                      br_if 0 (;@9;)
                      local.get 0
                      i32.load
                      local.get 3
                      local.get 2
                      i32.const 8
                      i32.add
                      i32.load
                      call $__rust_dealloc
                    end
                    local.get 0
                    i32.const 12
                    i32.const 4
                    call $__rust_dealloc
                  end
                  block  ;; label = @8
                    i32.const 0
                    i32.load offset=1059068
                    local.tee 0
                    i32.eqz
                    br_if 0 (;@8;)
                    i32.const 0
                    i32.load offset=1059072
                    local.get 0
                    i32.const 1
                    call $__rust_dealloc
                  end
                  i32.const 0
                  i64.const 4294967296
                  i64.store offset=1059068 align=4
                  i32.const 0
                  i32.const 0
                  i32.load offset=1059064
                  i32.const 1
                  i32.add
                  i32.store offset=1059064
                  i32.const 0
                  i32.const 0
                  i32.load offset=1059060
                  i32.const -1
                  i32.add
                  local.tee 0
                  i32.store offset=1059060
                  i32.const 0
                  i32.const 0
                  i32.store8 offset=1059080
                  i32.const 0
                  i32.const 0
                  i32.store offset=1059076
                  local.get 0
                  br_if 0 (;@7;)
                  i32.const 0
                  i32.const 0
                  i32.store8 offset=1059084
                  i32.const 0
                  i32.const 0
                  i32.store offset=1059056
                end
                i32.const 0
                i32.const 3
                i32.store8 offset=1059053
              end
              local.get 1
              i32.const 32
              i32.add
              global.set $__stack_pointer
              return
            end
            local.get 1
            i32.const 12
            i32.add
            i32.const 1
            i32.store
            local.get 1
            i32.const 20
            i32.add
            i32.const 0
            i32.store
            local.get 1
            i32.const 1052280
            i32.store offset=8
            local.get 1
            i32.const 1048832
            i32.store offset=16
            local.get 1
            i32.const 0
            i32.store
            local.get 1
            i32.const 1049200
            call $_ZN4core9panicking9panic_fmt17h0724b0a601bd3679E
            unreachable
          end
          i32.const 1048868
          i32.const 43
          i32.const 1049980
          call $_ZN4core9panicking5panic17h33aac59f7456cb9eE
          unreachable
        end
        i32.const 1050028
        i32.const 38
        i32.const 1050100
        call $_ZN4core6option13expect_failed17h001f1263852c0fc9E
        unreachable
      end
      i32.const 1049532
      i32.const 16
      local.get 1
      i32.const 1049548
      i32.const 1049592
      call $_ZN4core6result13unwrap_failed17h0cc42b3b30eb09ddE
      unreachable
    end
    local.get 1
    i32.const 12
    i32.add
    i32.const 1
    i32.store
    local.get 1
    i32.const 20
    i32.add
    i32.const 0
    i32.store
    local.get 1
    i32.const 1052228
    i32.store offset=8
    local.get 1
    i32.const 1048832
    i32.store offset=16
    local.get 1
    i32.const 0
    i32.store
    local.get 1
    i32.const 1049200
    call $_ZN4core9panicking9panic_fmt17h0724b0a601bd3679E
    unreachable)
  (func $_ZN3std2rt19lang_start_internal17hf59d11c495a5d34cE (type 12) (param i32 i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i64 i64 i64)
    global.get $__stack_pointer
    i32.const 128
    i32.sub
    local.tee 5
    global.set $__stack_pointer
    local.get 5
    i32.const 24
    i32.add
    i32.const 1049084
    i32.const 4
    call $_ZN72_$LT$$RF$str$u20$as$u20$alloc..ffi..c_str..CString..new..SpecNewImpl$GT$13spec_new_impl17hc2e87543d0ee2275E
    block  ;; label = @1
      local.get 5
      i32.load offset=32
      br_if 0 (;@1;)
      local.get 5
      i32.load offset=28
      local.set 6
      local.get 5
      i32.load offset=24
      local.set 7
      local.get 5
      i32.const 16
      i32.add
      i32.const 24
      i32.const 8
      call $_ZN5alloc4sync32arcinner_layout_for_value_layout17h85ba5c6b8bd0fdb9E
      local.get 5
      i32.load offset=20
      local.set 8
      local.get 5
      i32.load offset=16
      local.set 9
      local.get 5
      i32.const 8
      i32.add
      i32.const 24
      i32.const 8
      call $_ZN5alloc4sync32arcinner_layout_for_value_layout17h85ba5c6b8bd0fdb9E
      local.get 5
      i32.load offset=12
      local.set 10
      block  ;; label = @2
        local.get 5
        i32.load offset=8
        local.tee 11
        i32.eqz
        br_if 0 (;@2;)
        local.get 11
        local.get 10
        call $__rust_alloc
        local.set 10
      end
      block  ;; label = @2
        block  ;; label = @3
          local.get 10
          i32.eqz
          br_if 0 (;@3;)
          local.get 10
          i64.const 4294967297
          i64.store align=4
          local.get 10
          local.get 6
          i32.store offset=12
          local.get 10
          local.get 7
          i32.store offset=8
          i32.const 0
          i64.load offset=1059128
          local.set 12
          loop  ;; label = @4
            local.get 12
            i64.const 1
            i64.add
            local.tee 13
            i64.eqz
            br_if 2 (;@2;)
            i32.const 0
            local.get 13
            i32.const 0
            i64.load offset=1059128
            local.tee 14
            local.get 14
            local.get 12
            i64.eq
            local.tee 6
            select
            i64.store offset=1059128
            local.get 14
            local.set 12
            local.get 6
            i32.eqz
            br_if 0 (;@4;)
          end
          local.get 10
          i32.const 0
          i32.store16 offset=20
          local.get 10
          i32.const 16
          i32.add
          i32.const 0
          i32.store
          local.get 10
          i32.const 24
          i32.add
          local.get 13
          i64.store
          block  ;; label = @4
            local.get 5
            i32.load offset=32
            local.tee 6
            i32.eqz
            br_if 0 (;@4;)
            local.get 5
            i32.load offset=28
            local.tee 7
            i32.eqz
            br_if 0 (;@4;)
            local.get 6
            local.get 7
            i32.const 1
            call $__rust_dealloc
          end
          local.get 10
          call $_ZN3std10sys_common11thread_info3set17h44e226fcebce2f66E
          local.get 0
          local.get 1
          i32.const 20
          i32.add
          i32.load
          call_indirect (type 4)
          local.set 6
          block  ;; label = @4
            i32.const 0
            i32.load8_u offset=1059053
            i32.const 3
            i32.eq
            br_if 0 (;@4;)
            local.get 5
            i32.const 1
            i32.store8 offset=88
            local.get 5
            i32.const 88
            i32.add
            call $_ZN3std3sys4wasi4once4Once4call17hd3dc100d98d62d15E
          end
          local.get 5
          i32.const 128
          i32.add
          global.set $__stack_pointer
          local.get 6
          return
        end
        local.get 9
        local.get 8
        call $_ZN5alloc5alloc18handle_alloc_error17h4888856d90202140E
        unreachable
      end
      call $_ZN3std6thread8ThreadId3new9exhausted17he44694043c719b0aE
      unreachable
    end
    local.get 5
    local.get 5
    i32.const 24
    i32.add
    i32.store offset=44
    local.get 5
    i32.const 56
    i32.add
    i32.const 12
    i32.add
    i32.const 2
    i32.store
    local.get 5
    i32.const 56
    i32.add
    i32.const 20
    i32.add
    i32.const 1
    i32.store
    local.get 5
    i32.const 88
    i32.add
    i32.const 12
    i32.add
    i32.const 1
    i32.store
    local.get 5
    i32.const 88
    i32.add
    i32.const 20
    i32.add
    i32.const 1
    i32.store
    local.get 5
    i32.const 1049112
    i32.store offset=64
    local.get 5
    i32.const 0
    i32.store offset=56
    local.get 5
    i32.const 7
    i32.store offset=84
    local.get 5
    i32.const 1049168
    i32.store offset=96
    local.get 5
    i32.const 0
    i32.store offset=88
    local.get 5
    i32.const 8
    i32.store offset=116
    local.get 5
    local.get 5
    i32.const 80
    i32.add
    i32.store offset=72
    local.get 5
    local.get 5
    i32.const 88
    i32.add
    i32.store offset=80
    local.get 5
    local.get 5
    i32.const 112
    i32.add
    i32.store offset=104
    local.get 5
    local.get 5
    i32.const 44
    i32.add
    i32.store offset=112
    local.get 5
    i32.const 48
    i32.add
    local.get 5
    i32.const 120
    i32.add
    local.get 5
    i32.const 56
    i32.add
    call $_ZN3std2io5Write9write_fmt17h73c1d085b221682dE
    local.get 5
    i32.const 48
    i32.add
    call $_ZN4core3ptr81drop_in_place$LT$core..result..Result$LT$$LP$$RP$$C$std..io..error..Error$GT$$GT$17h578f989f6735c81dE
    call $_ZN3std3sys4wasi14abort_internal17h9975d127165b10b0E
    unreachable)
  (func $_ZN3std10sys_common11thread_info3set17h44e226fcebce2f66E (type 1) (param i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 80
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        i32.const 0
        i32.load offset=1059148
        br_if 0 (;@2;)
        i32.const 0
        i32.const -1
        i32.store offset=1059148
        i32.const 0
        i32.load offset=1059152
        br_if 1 (;@1;)
        i32.const 0
        local.get 0
        i32.store offset=1059152
        i32.const 0
        i32.const 0
        i32.store offset=1059148
        local.get 1
        i32.const 80
        i32.add
        global.set $__stack_pointer
        return
      end
      i32.const 1049532
      i32.const 16
      local.get 1
      i32.const 72
      i32.add
      i32.const 1049548
      i32.const 1050368
      call $_ZN4core6result13unwrap_failed17h0cc42b3b30eb09ddE
      unreachable
    end
    local.get 1
    i32.const 16
    i32.add
    i32.const 12
    i32.add
    i32.const 2
    i32.store
    local.get 1
    i32.const 16
    i32.add
    i32.const 20
    i32.add
    i32.const 1
    i32.store
    local.get 1
    i32.const 48
    i32.add
    i32.const 12
    i32.add
    i32.const 1
    i32.store
    local.get 1
    i32.const 48
    i32.add
    i32.const 20
    i32.add
    i32.const 0
    i32.store
    local.get 1
    i32.const 1049112
    i32.store offset=24
    local.get 1
    i32.const 0
    i32.store offset=16
    local.get 1
    i32.const 7
    i32.store offset=44
    local.get 1
    i32.const 1050424
    i32.store offset=56
    local.get 1
    i32.const 1048832
    i32.store offset=64
    local.get 1
    i32.const 0
    i32.store offset=48
    local.get 1
    local.get 1
    i32.const 40
    i32.add
    i32.store offset=32
    local.get 1
    local.get 1
    i32.const 48
    i32.add
    i32.store offset=40
    local.get 1
    i32.const 8
    i32.add
    local.get 1
    i32.const 72
    i32.add
    local.get 1
    i32.const 16
    i32.add
    call $_ZN3std2io5Write9write_fmt17h73c1d085b221682dE
    local.get 1
    i32.const 8
    i32.add
    call $_ZN4core3ptr81drop_in_place$LT$core..result..Result$LT$$LP$$RP$$C$std..io..error..Error$GT$$GT$17h578f989f6735c81dE
    call $_ZN3std3sys4wasi14abort_internal17h9975d127165b10b0E
    unreachable)
  (func $_ZN3std6thread8ThreadId3new9exhausted17he44694043c719b0aE (type 0)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 0
    global.set $__stack_pointer
    local.get 0
    i32.const 20
    i32.add
    i32.const 1
    i32.store
    local.get 0
    i32.const 28
    i32.add
    i32.const 0
    i32.store
    local.get 0
    i32.const 1049300
    i32.store offset=16
    local.get 0
    i32.const 1048832
    i32.store offset=24
    local.get 0
    i32.const 0
    i32.store offset=8
    local.get 0
    i32.const 8
    i32.add
    i32.const 1049308
    call $_ZN4core9panicking9panic_fmt17h0724b0a601bd3679E
    unreachable)
  (func $_ZN3std2io5Write9write_fmt17h73c1d085b221682dE (type 5) (param i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    i32.const 4
    i32.store8 offset=8
    local.get 3
    local.get 1
    i32.store offset=16
    local.get 3
    i32.const 24
    i32.add
    i32.const 16
    i32.add
    local.get 2
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 3
    i32.const 24
    i32.add
    i32.const 8
    i32.add
    local.get 2
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 3
    local.get 2
    i64.load align=4
    i64.store offset=24
    block  ;; label = @1
      block  ;; label = @2
        local.get 3
        i32.const 8
        i32.add
        i32.const 1049824
        local.get 3
        i32.const 24
        i32.add
        call $_ZN4core3fmt5write17h56e014c9dc514616E
        i32.eqz
        br_if 0 (;@2;)
        block  ;; label = @3
          local.get 3
          i32.load8_u offset=8
          i32.const 4
          i32.ne
          br_if 0 (;@3;)
          local.get 0
          i32.const 1049812
          i32.store offset=4
          local.get 0
          i32.const 2
          i32.store
          br 2 (;@1;)
        end
        local.get 0
        local.get 3
        i64.load offset=8
        i64.store align=4
        br 1 (;@1;)
      end
      local.get 0
      i32.const 4
      i32.store8
      local.get 3
      i32.load8_u offset=8
      i32.const 3
      i32.ne
      br_if 0 (;@1;)
      local.get 3
      i32.load offset=12
      local.tee 2
      i32.load
      local.get 2
      i32.load offset=4
      i32.load
      call_indirect (type 1)
      block  ;; label = @2
        local.get 2
        i32.load offset=4
        local.tee 0
        i32.const 4
        i32.add
        i32.load
        local.tee 1
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        i32.load
        local.get 1
        local.get 0
        i32.const 8
        i32.add
        i32.load
        call $__rust_dealloc
      end
      local.get 3
      i32.load offset=12
      i32.const 12
      i32.const 4
      call $__rust_dealloc
    end
    local.get 3
    i32.const 48
    i32.add
    global.set $__stack_pointer)
  (func $_ZN3std3sys4wasi14abort_internal17h9975d127165b10b0E (type 0)
    call $abort
    unreachable)
  (func $_ZN3std10sys_common11thread_info14current_thread17h11644fc46f8bf6b5E (type 10) (result i32)
    (local i32 i32 i32 i32 i32 i64 i64 i64)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 0
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            i32.const 0
            i32.load offset=1059148
            br_if 0 (;@4;)
            i32.const 0
            i32.const -1
            i32.store offset=1059148
            block  ;; label = @5
              i32.const 0
              i32.load offset=1059152
              local.tee 1
              br_if 0 (;@5;)
              local.get 0
              i32.const 16
              i32.add
              i32.const 24
              i32.const 8
              call $_ZN5alloc4sync32arcinner_layout_for_value_layout17h85ba5c6b8bd0fdb9E
              local.get 0
              i32.load offset=20
              local.set 2
              local.get 0
              i32.load offset=16
              local.set 3
              local.get 0
              i32.const 8
              i32.add
              i32.const 24
              i32.const 8
              call $_ZN5alloc4sync32arcinner_layout_for_value_layout17h85ba5c6b8bd0fdb9E
              local.get 0
              i32.load offset=12
              local.set 1
              block  ;; label = @6
                local.get 0
                i32.load offset=8
                local.tee 4
                i32.eqz
                br_if 0 (;@6;)
                local.get 4
                local.get 1
                call $__rust_alloc
                local.set 1
              end
              local.get 1
              i32.eqz
              br_if 3 (;@2;)
              local.get 1
              i64.const 4294967297
              i64.store align=4
              local.get 1
              i32.const 0
              i32.store offset=8
              i32.const 0
              i64.load offset=1059128
              local.set 5
              loop  ;; label = @6
                local.get 5
                i64.const 1
                i64.add
                local.tee 6
                i64.eqz
                br_if 5 (;@1;)
                i32.const 0
                local.get 6
                i32.const 0
                i64.load offset=1059128
                local.tee 7
                local.get 7
                local.get 5
                i64.eq
                local.tee 4
                select
                i64.store offset=1059128
                local.get 7
                local.set 5
                local.get 4
                i32.eqz
                br_if 0 (;@6;)
              end
              local.get 1
              i32.const 0
              i32.store16 offset=20
              i32.const 0
              local.get 1
              i32.store offset=1059152
              local.get 1
              i32.const 16
              i32.add
              i32.const 0
              i32.store
              local.get 1
              i32.const 24
              i32.add
              local.get 6
              i64.store
            end
            local.get 1
            local.get 1
            i32.load
            local.tee 4
            i32.const 1
            i32.add
            i32.store
            local.get 4
            i32.const -1
            i32.gt_s
            br_if 1 (;@3;)
            unreachable
            unreachable
          end
          i32.const 1049532
          i32.const 16
          local.get 0
          i32.const 24
          i32.add
          i32.const 1049548
          i32.const 1050352
          call $_ZN4core6result13unwrap_failed17h0cc42b3b30eb09ddE
          unreachable
        end
        i32.const 0
        i32.const 0
        i32.load offset=1059148
        i32.const 1
        i32.add
        i32.store offset=1059148
        local.get 0
        i32.const 32
        i32.add
        global.set $__stack_pointer
        local.get 1
        return
      end
      local.get 3
      local.get 2
      call $_ZN5alloc5alloc18handle_alloc_error17h4888856d90202140E
      unreachable
    end
    call $_ZN3std6thread8ThreadId3new9exhausted17he44694043c719b0aE
    unreachable)
  (func $_ZN3std3env11current_dir17h8a78ebea161e6b13E (type 1) (param i32)
    (local i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    i32.const 512
    local.set 2
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            i32.const 512
            i32.const 1
            call $__rust_alloc
            local.tee 3
            i32.eqz
            br_if 0 (;@4;)
            local.get 1
            local.get 3
            i32.store offset=4
            local.get 1
            i32.const 512
            i32.store
            local.get 3
            i32.const 512
            call $getcwd
            br_if 1 (;@3;)
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  i32.const 0
                  i32.load offset=1059656
                  local.tee 2
                  i32.const 68
                  i32.ne
                  br_if 0 (;@7;)
                  i32.const 512
                  local.set 2
                  br 1 (;@6;)
                end
                local.get 0
                i64.const 1
                i64.store align=4
                local.get 0
                i32.const 8
                i32.add
                local.get 2
                i32.store
                i32.const 512
                local.set 2
                br 1 (;@5;)
              end
              loop  ;; label = @6
                local.get 1
                local.get 2
                i32.store offset=8
                local.get 1
                local.get 2
                i32.const 1
                call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17h8af0b8a49dc47a67E
                local.get 1
                i32.load offset=4
                local.tee 3
                local.get 1
                i32.load
                local.tee 2
                call $getcwd
                br_if 3 (;@3;)
                i32.const 0
                i32.load offset=1059656
                local.tee 4
                i32.const 68
                i32.eq
                br_if 0 (;@6;)
              end
              local.get 0
              i64.const 1
              i64.store align=4
              local.get 0
              i32.const 8
              i32.add
              local.get 4
              i32.store
              local.get 2
              i32.eqz
              br_if 3 (;@2;)
            end
            local.get 3
            local.get 2
            i32.const 1
            call $__rust_dealloc
            br 2 (;@2;)
          end
          i32.const 512
          i32.const 1
          call $_ZN5alloc5alloc18handle_alloc_error17h4888856d90202140E
          unreachable
        end
        local.get 1
        local.get 3
        call $_ZN4core3ffi5c_str4CStr8from_ptr9strlen_rt17h15b37b953b28d4f9E
        local.tee 4
        i32.store offset=8
        block  ;; label = @3
          local.get 2
          local.get 4
          i32.le_u
          br_if 0 (;@3;)
          block  ;; label = @4
            block  ;; label = @5
              local.get 4
              br_if 0 (;@5;)
              i32.const 1
              local.set 5
              local.get 3
              local.get 2
              i32.const 1
              call $__rust_dealloc
              br 1 (;@4;)
            end
            local.get 3
            local.get 2
            i32.const 1
            local.get 4
            call $__rust_realloc
            local.tee 5
            i32.eqz
            br_if 3 (;@1;)
          end
          local.get 1
          local.get 4
          i32.store
          local.get 1
          local.get 5
          i32.store offset=4
        end
        local.get 0
        local.get 1
        i64.load
        i64.store offset=4 align=4
        local.get 0
        i32.const 0
        i32.store
        local.get 0
        i32.const 12
        i32.add
        local.get 1
        i32.const 8
        i32.add
        i32.load
        i32.store
      end
      local.get 1
      i32.const 16
      i32.add
      global.set $__stack_pointer
      return
    end
    local.get 4
    i32.const 1
    call $_ZN5alloc5alloc18handle_alloc_error17h4888856d90202140E
    unreachable)
  (func $_ZN3std3env7_var_os17h37aa4fcd300c1389E (type 5) (param i32 i32 i32)
    (local i32 i64 i32 i32)
    global.get $__stack_pointer
    i32.const 416
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        i32.const 383
        i32.gt_u
        br_if 0 (;@2;)
        local.get 3
        i32.const 16
        i32.add
        local.get 1
        local.get 2
        call $memcpy
        drop
        local.get 3
        i32.const 16
        i32.add
        local.get 2
        i32.add
        i32.const 0
        i32.store8
        local.get 3
        i32.const 400
        i32.add
        local.get 3
        i32.const 16
        i32.add
        local.get 2
        i32.const 1
        i32.add
        call $_ZN4core3ffi5c_str4CStr19from_bytes_with_nul17h2353c14c48c58dfeE
        block  ;; label = @3
          local.get 3
          i32.load offset=400
          br_if 0 (;@3;)
          i32.const 4
          local.set 2
          local.get 3
          i32.load offset=404
          call $getenv
          local.set 1
          local.get 3
          i32.const 4
          i32.store8 offset=8
          local.get 3
          local.get 1
          i32.store offset=12
          br 2 (;@1;)
        end
        local.get 3
        i32.const 0
        i64.load offset=1050176
        local.tee 4
        i64.store offset=8
        local.get 4
        i32.wrap_i64
        local.set 2
        br 1 (;@1;)
      end
      local.get 3
      i32.const 8
      i32.add
      local.get 1
      local.get 2
      call $_ZN3std3sys6common14small_c_string24run_with_cstr_allocating17h64df75db97c056b9E
      local.get 3
      i32.load8_u offset=8
      local.set 2
    end
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        i32.const 255
        i32.and
        i32.const 4
        i32.eq
        br_if 0 (;@2;)
        block  ;; label = @3
          local.get 3
          i32.load8_u offset=8
          i32.const 3
          i32.ne
          br_if 0 (;@3;)
          local.get 3
          i32.load offset=12
          local.tee 2
          i32.load
          local.get 2
          i32.load offset=4
          i32.load
          call_indirect (type 1)
          block  ;; label = @4
            local.get 2
            i32.load offset=4
            local.tee 1
            i32.const 4
            i32.add
            i32.load
            local.tee 5
            i32.eqz
            br_if 0 (;@4;)
            local.get 2
            i32.load
            local.get 5
            local.get 1
            i32.const 8
            i32.add
            i32.load
            call $__rust_dealloc
          end
          local.get 2
          i32.const 12
          i32.const 4
          call $__rust_dealloc
        end
        local.get 0
        i32.const 0
        i32.store offset=4
        br 1 (;@1;)
      end
      block  ;; label = @2
        local.get 3
        i32.load offset=12
        local.tee 1
        i32.eqz
        br_if 0 (;@2;)
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 1
                call $_ZN4core3ffi5c_str4CStr8from_ptr9strlen_rt17h15b37b953b28d4f9E
                local.tee 2
                br_if 0 (;@6;)
                i32.const 1
                local.set 5
                br 1 (;@5;)
              end
              local.get 2
              i32.const -1
              i32.gt_s
              local.tee 6
              i32.eqz
              br_if 1 (;@4;)
              local.get 2
              local.get 6
              call $__rust_alloc
              local.tee 5
              i32.eqz
              br_if 2 (;@3;)
            end
            local.get 5
            local.get 1
            local.get 2
            call $memcpy
            local.set 1
            local.get 0
            local.get 2
            i32.store offset=8
            local.get 0
            local.get 1
            i32.store offset=4
            local.get 0
            local.get 2
            i32.store
            br 3 (;@1;)
          end
          call $_ZN5alloc7raw_vec17capacity_overflow17hbb754ab7ed70e9c1E
          unreachable
        end
        local.get 2
        local.get 6
        call $_ZN5alloc5alloc18handle_alloc_error17h4888856d90202140E
        unreachable
      end
      local.get 0
      i32.const 0
      i32.store offset=4
    end
    local.get 3
    i32.const 416
    i32.add
    global.set $__stack_pointer)
  (func $_ZN3std3sys6common14small_c_string24run_with_cstr_allocating17h64df75db97c056b9E (type 5) (param i32 i32 i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    local.get 1
    local.get 2
    call $_ZN72_$LT$$RF$str$u20$as$u20$alloc..ffi..c_str..CString..new..SpecNewImpl$GT$13spec_new_impl17hc2e87543d0ee2275E
    block  ;; label = @1
      block  ;; label = @2
        local.get 3
        i32.load offset=8
        local.tee 2
        br_if 0 (;@2;)
        local.get 3
        i32.load offset=4
        local.set 2
        local.get 3
        i32.load
        local.tee 1
        call $getenv
        local.set 4
        local.get 0
        i32.const 4
        i32.store8
        local.get 0
        local.get 4
        i32.store offset=4
        local.get 1
        i32.const 0
        i32.store8
        local.get 2
        i32.eqz
        br_if 1 (;@1;)
        local.get 1
        local.get 2
        i32.const 1
        call $__rust_dealloc
        br 1 (;@1;)
      end
      local.get 0
      i32.const 0
      i64.load offset=1050176
      i64.store align=4
      local.get 3
      i32.load offset=4
      local.tee 0
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      local.get 0
      i32.const 1
      call $__rust_dealloc
    end
    local.get 3
    i32.const 16
    i32.add
    global.set $__stack_pointer)
  (func $_ZN3std3sys4wasi2fs4File4open17h2a143480455f3200E (type 11) (param i32 i32 i32 i32)
    (local i32 i64)
    global.get $__stack_pointer
    i32.const 416
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        i32.const 383
        i32.gt_u
        br_if 0 (;@2;)
        local.get 4
        i32.const 16
        i32.add
        local.get 1
        local.get 2
        call $memcpy
        drop
        local.get 4
        i32.const 16
        i32.add
        local.get 2
        i32.add
        i32.const 0
        i32.store8
        local.get 4
        i32.const 400
        i32.add
        local.get 4
        i32.const 16
        i32.add
        local.get 2
        i32.const 1
        i32.add
        call $_ZN4core3ffi5c_str4CStr19from_bytes_with_nul17h2353c14c48c58dfeE
        block  ;; label = @3
          local.get 4
          i32.load offset=400
          br_if 0 (;@3;)
          local.get 4
          local.get 4
          i32.load offset=404
          local.get 4
          i32.const 408
          i32.add
          i32.load
          call $_ZN3std3sys4wasi2fs11open_parent28_$u7b$$u7b$closure$u7d$$u7d$17h7132f8abd0247809E
          br 2 (;@1;)
        end
        local.get 4
        i32.const -1
        i32.store
        local.get 4
        i32.const 0
        i64.load offset=1050176
        i64.store offset=4 align=4
        br 1 (;@1;)
      end
      local.get 4
      local.get 1
      local.get 2
      call $_ZN3std3sys6common14small_c_string24run_with_cstr_allocating17h138d4eb697526d6cE
    end
    local.get 4
    i64.load offset=4 align=4
    local.set 5
    block  ;; label = @1
      block  ;; label = @2
        local.get 4
        i32.load
        local.tee 2
        i32.const -1
        i32.eq
        br_if 0 (;@2;)
        local.get 0
        local.get 2
        local.get 5
        i64.const 32
        i64.shr_u
        i32.wrap_i64
        local.tee 1
        local.get 4
        i32.load offset=12
        local.get 3
        call $_ZN3std3sys4wasi2fs7open_at17h6ed0c206cbba7569E
        local.get 5
        i32.wrap_i64
        local.tee 2
        i32.eqz
        br_if 1 (;@1;)
        local.get 1
        local.get 2
        i32.const 1
        call $__rust_dealloc
        br 1 (;@1;)
      end
      local.get 0
      local.get 5
      i64.store align=4
    end
    local.get 4
    i32.const 416
    i32.add
    global.set $__stack_pointer)
  (func $_ZN3std2fs11OpenOptions3new17h9b06b1aa8da3f811E (type 1) (param i32)
    local.get 0
    i64.const 1
    i64.store offset=32
    local.get 0
    i64.const 0
    i64.store offset=16
    local.get 0
    i64.const 0
    i64.store
    local.get 0
    i32.const 39
    i32.add
    i32.const 0
    i32.store align=1)
  (func $_ZN3std2fs11OpenOptions5write17h09aac53a2280d979E (type 2) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.store8 offset=41
    local.get 0)
  (func $_ZN3std2fs11OpenOptions8truncate17hdf1d682976bc01f6E (type 2) (param i32 i32) (result i32)
    local.get 0
    local.get 0
    i32.load16_u offset=38
    i32.const 65527
    i32.and
    i32.const 8
    i32.const 0
    local.get 1
    select
    i32.or
    i32.store16 offset=38
    local.get 0)
  (func $_ZN3std2fs11OpenOptions6create17hc7f26e82b4d268a3E (type 2) (param i32 i32) (result i32)
    local.get 0
    local.get 0
    i32.load16_u offset=38
    i32.const 65534
    i32.and
    local.get 1
    i32.or
    i32.store16 offset=38
    local.get 0)
  (func $_ZN3std2fs11OpenOptions5_open17hd9343526833faf36E (type 11) (param i32 i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    local.get 4
    i32.const 8
    i32.add
    local.get 2
    local.get 3
    local.get 1
    call $_ZN3std3sys4wasi2fs4File4open17h2a143480455f3200E
    block  ;; label = @1
      block  ;; label = @2
        local.get 4
        i32.load8_u offset=8
        i32.const 4
        i32.ne
        br_if 0 (;@2;)
        local.get 0
        local.get 4
        i32.load offset=12
        i32.store offset=4
        local.get 0
        i32.const 4
        i32.store8
        br 1 (;@1;)
      end
      local.get 0
      local.get 4
      i64.load offset=8
      i64.store align=4
    end
    local.get 4
    i32.const 16
    i32.add
    global.set $__stack_pointer)
  (func $_ZN3std3sys4wasi17decode_error_kind17h037df60161d49d56E (type 4) (param i32) (result i32)
    (local i32)
    i32.const 40
    local.set 1
    block  ;; label = @1
      local.get 0
      i32.const 65535
      i32.gt_u
      br_if 0 (;@1;)
      i32.const 2
      local.set 1
      i32.const 1052398
      call $_ZN4wasi13lib_generated5Errno3raw17h169d0715fcc2f361E
      i32.const 65535
      i32.and
      local.get 0
      i32.eq
      br_if 0 (;@1;)
      i32.const 3
      local.set 1
      i32.const 1052400
      call $_ZN4wasi13lib_generated5Errno3raw17h169d0715fcc2f361E
      i32.const 65535
      i32.and
      local.get 0
      i32.eq
      br_if 0 (;@1;)
      i32.const 1
      local.set 1
      i32.const 1052402
      call $_ZN4wasi13lib_generated5Errno3raw17h169d0715fcc2f361E
      i32.const 65535
      i32.and
      local.get 0
      i32.eq
      br_if 0 (;@1;)
      i32.const 1052404
      call $_ZN4wasi13lib_generated5Errno3raw17h169d0715fcc2f361E
      i32.const 65535
      i32.and
      local.get 0
      i32.eq
      br_if 0 (;@1;)
      i32.const 11
      local.set 1
      i32.const 1052406
      call $_ZN4wasi13lib_generated5Errno3raw17h169d0715fcc2f361E
      i32.const 65535
      i32.and
      local.get 0
      i32.eq
      br_if 0 (;@1;)
      i32.const 7
      local.set 1
      i32.const 1052408
      call $_ZN4wasi13lib_generated5Errno3raw17h169d0715fcc2f361E
      i32.const 65535
      i32.and
      local.get 0
      i32.eq
      br_if 0 (;@1;)
      i32.const 6
      local.set 1
      i32.const 1052410
      call $_ZN4wasi13lib_generated5Errno3raw17h169d0715fcc2f361E
      i32.const 65535
      i32.and
      local.get 0
      i32.eq
      br_if 0 (;@1;)
      i32.const 9
      local.set 1
      i32.const 1052412
      call $_ZN4wasi13lib_generated5Errno3raw17h169d0715fcc2f361E
      i32.const 65535
      i32.and
      local.get 0
      i32.eq
      br_if 0 (;@1;)
      i32.const 8
      local.set 1
      i32.const 1052414
      call $_ZN4wasi13lib_generated5Errno3raw17h169d0715fcc2f361E
      i32.const 65535
      i32.and
      local.get 0
      i32.eq
      br_if 0 (;@1;)
      i32.const 0
      local.set 1
      i32.const 1052416
      call $_ZN4wasi13lib_generated5Errno3raw17h169d0715fcc2f361E
      i32.const 65535
      i32.and
      local.get 0
      i32.eq
      br_if 0 (;@1;)
      i32.const 35
      local.set 1
      i32.const 1052418
      call $_ZN4wasi13lib_generated5Errno3raw17h169d0715fcc2f361E
      i32.const 65535
      i32.and
      local.get 0
      i32.eq
      br_if 0 (;@1;)
      i32.const 20
      local.set 1
      i32.const 1052420
      call $_ZN4wasi13lib_generated5Errno3raw17h169d0715fcc2f361E
      i32.const 65535
      i32.and
      local.get 0
      i32.eq
      br_if 0 (;@1;)
      i32.const 22
      local.set 1
      i32.const 1052422
      call $_ZN4wasi13lib_generated5Errno3raw17h169d0715fcc2f361E
      i32.const 65535
      i32.and
      local.get 0
      i32.eq
      br_if 0 (;@1;)
      i32.const 12
      local.set 1
      i32.const 1052424
      call $_ZN4wasi13lib_generated5Errno3raw17h169d0715fcc2f361E
      i32.const 65535
      i32.and
      local.get 0
      i32.eq
      br_if 0 (;@1;)
      i32.const 13
      local.set 1
      i32.const 1052426
      call $_ZN4wasi13lib_generated5Errno3raw17h169d0715fcc2f361E
      i32.const 65535
      i32.and
      local.get 0
      i32.eq
      br_if 0 (;@1;)
      i32.const 36
      local.set 1
      i32.const 1052428
      call $_ZN4wasi13lib_generated5Errno3raw17h169d0715fcc2f361E
      i32.const 65535
      i32.and
      local.get 0
      i32.eq
      br_if 0 (;@1;)
      i32.const 38
      i32.const 40
      i32.const 1052430
      call $_ZN4wasi13lib_generated5Errno3raw17h169d0715fcc2f361E
      i32.const 65535
      i32.and
      local.get 0
      i32.eq
      select
      local.set 1
    end
    local.get 1)
  (func $_ZN3std2io8buffered9bufwriter18BufWriter$LT$W$GT$9flush_buf17h025b1526cfabd17eE (type 6) (param i32 i32)
    (local i32 i32 i32 i64 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 1
          i32.load offset=8
          local.tee 3
          br_if 0 (;@3;)
          local.get 0
          i32.const 4
          i32.store8
          br 1 (;@2;)
        end
        local.get 1
        i32.load offset=4
        local.set 4
        i32.const 1049416
        i64.extend_i32_u
        i64.const 32
        i64.shl
        i64.const 2
        i64.or
        local.set 5
        i32.const 0
        local.set 6
        loop  ;; label = @3
          local.get 2
          local.get 3
          local.get 6
          i32.sub
          local.tee 7
          i32.store offset=12
          local.get 2
          local.get 4
          local.get 6
          i32.add
          local.tee 8
          i32.store offset=8
          local.get 2
          i32.const 16
          i32.add
          i32.const 1
          local.get 2
          i32.const 8
          i32.add
          i32.const 1
          call $_ZN4wasi13lib_generated8fd_write17h78b16dde3600cdd5E
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 2
                    i32.load16_u offset=16
                    br_if 0 (;@8;)
                    local.get 2
                    i32.load offset=20
                    local.set 9
                    local.get 1
                    i32.const 0
                    i32.store8 offset=12
                    br 1 (;@7;)
                  end
                  local.get 2
                  local.get 2
                  i32.load16_u offset=18
                  i32.store16 offset=30
                  local.get 2
                  i32.const 30
                  i32.add
                  call $_ZN4wasi13lib_generated5Errno3raw17h169d0715fcc2f361E
                  local.set 10
                  i32.const 1052396
                  call $_ZN4wasi13lib_generated5Errno3raw17h169d0715fcc2f361E
                  local.set 11
                  local.get 1
                  i32.const 0
                  i32.store8 offset=12
                  local.get 7
                  local.set 9
                  local.get 10
                  i32.const 65535
                  i32.and
                  local.tee 12
                  local.get 11
                  i32.const 65535
                  i32.and
                  i32.ne
                  br_if 1 (;@6;)
                end
                local.get 9
                i32.eqz
                br_if 1 (;@5;)
                local.get 9
                local.get 6
                i32.add
                local.set 6
                br 2 (;@4;)
              end
              local.get 12
              call $_ZN3std3sys4wasi17decode_error_kind17h037df60161d49d56E
              i32.const 255
              i32.and
              i32.const 35
              i32.eq
              br_if 1 (;@4;)
              local.get 10
              i64.extend_i32_u
              i64.const 65535
              i64.and
              i64.const 32
              i64.shl
              local.set 5
            end
            local.get 0
            local.get 5
            i64.store align=4
            local.get 6
            i32.eqz
            br_if 2 (;@2;)
            block  ;; label = @5
              local.get 3
              local.get 6
              i32.lt_u
              br_if 0 (;@5;)
              local.get 1
              i32.const 0
              i32.store offset=8
              local.get 3
              local.get 6
              i32.eq
              br_if 3 (;@2;)
              local.get 4
              local.get 8
              local.get 7
              call $memmove
              drop
              local.get 1
              local.get 7
              i32.store offset=8
              br 3 (;@2;)
            end
            local.get 6
            local.get 3
            i32.const 1049028
            call $_ZN4core5slice5index24slice_end_index_len_fail17h6cad56aacf5f762dE
            unreachable
          end
          local.get 3
          local.get 6
          i32.gt_u
          br_if 0 (;@3;)
        end
        local.get 0
        i32.const 4
        i32.store8
        local.get 6
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        local.get 6
        i32.lt_u
        br_if 1 (;@1;)
        local.get 1
        i32.const 0
        i32.store offset=8
        local.get 3
        local.get 6
        i32.eq
        br_if 0 (;@2;)
        local.get 1
        i32.load offset=4
        local.tee 9
        local.get 9
        local.get 6
        i32.add
        local.get 3
        local.get 6
        i32.sub
        local.tee 6
        call $memmove
        drop
        local.get 1
        local.get 6
        i32.store offset=8
      end
      local.get 2
      i32.const 32
      i32.add
      global.set $__stack_pointer
      return
    end
    local.get 6
    local.get 3
    i32.const 1049028
    call $_ZN4core5slice5index24slice_end_index_len_fail17h6cad56aacf5f762dE
    unreachable)
  (func $_ZN58_$LT$std..io..error..Error$u20$as$u20$core..fmt..Debug$GT$3fmt17h3fdbb843780dfbdaE (type 2) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call $_ZN3std2io5error82_$LT$impl$u20$core..fmt..Debug$u20$for$u20$std..io..error..repr_unpacked..Repr$GT$3fmt17h3b4105f9b47ac617E)
  (func $_ZN3std3sys4wasi2os12error_string17h74596da84708f3a6E (type 6) (param i32 i32)
    (local i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 1056
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 1
          local.get 2
          i32.const 0
          i32.const 1024
          call $memset
          local.tee 2
          i32.const 1024
          call $strerror_r
          i32.const 0
          i32.lt_s
          br_if 0 (;@3;)
          local.get 2
          i32.const 1024
          i32.add
          local.get 2
          local.get 2
          call $_ZN4core3ffi5c_str4CStr8from_ptr9strlen_rt17h15b37b953b28d4f9E
          call $_ZN4core3str8converts9from_utf817he9b4783fc054fc5dE
          block  ;; label = @4
            local.get 2
            i32.load offset=1024
            br_if 0 (;@4;)
            local.get 2
            i32.load offset=1028
            local.set 3
            block  ;; label = @5
              block  ;; label = @6
                local.get 2
                i32.const 1032
                i32.add
                i32.load
                local.tee 1
                br_if 0 (;@6;)
                i32.const 1
                local.set 4
                br 1 (;@5;)
              end
              local.get 1
              i32.const -1
              i32.gt_s
              local.tee 5
              i32.eqz
              br_if 3 (;@2;)
              local.get 1
              local.get 5
              call $__rust_alloc
              local.tee 4
              i32.eqz
              br_if 4 (;@1;)
            end
            local.get 4
            local.get 3
            local.get 1
            call $memcpy
            local.set 3
            local.get 0
            local.get 1
            i32.store offset=8
            local.get 0
            local.get 3
            i32.store offset=4
            local.get 0
            local.get 1
            i32.store
            local.get 2
            i32.const 1056
            i32.add
            global.set $__stack_pointer
            return
          end
          local.get 2
          local.get 2
          i64.load offset=1028 align=4
          i64.store offset=1048
          i32.const 1049338
          i32.const 43
          local.get 2
          i32.const 1048
          i32.add
          i32.const 1052364
          i32.const 1052380
          call $_ZN4core6result13unwrap_failed17h0cc42b3b30eb09ddE
          unreachable
        end
        local.get 2
        i32.const 1036
        i32.add
        i32.const 1
        i32.store
        local.get 2
        i32.const 1044
        i32.add
        i32.const 0
        i32.store
        local.get 2
        i32.const 1052308
        i32.store offset=1032
        local.get 2
        i32.const 1048832
        i32.store offset=1040
        local.get 2
        i32.const 0
        i32.store offset=1024
        local.get 2
        i32.const 1024
        i32.add
        i32.const 1052348
        call $_ZN4core9panicking9panic_fmt17h0724b0a601bd3679E
        unreachable
      end
      call $_ZN5alloc7raw_vec17capacity_overflow17hbb754ab7ed70e9c1E
      unreachable
    end
    local.get 1
    local.get 5
    call $_ZN5alloc5alloc18handle_alloc_error17h4888856d90202140E
    unreachable)
  (func $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$5write17h390bc906e4bb57aeE (type 11) (param i32 i32 i32 i32)
    (local i32)
    block  ;; label = @1
      local.get 1
      i32.load
      local.get 1
      i32.load offset=8
      local.tee 4
      i32.sub
      local.get 3
      i32.ge_u
      br_if 0 (;@1;)
      local.get 1
      local.get 4
      local.get 3
      call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17h8af0b8a49dc47a67E
      local.get 1
      i32.load offset=8
      local.set 4
    end
    local.get 1
    i32.load offset=4
    local.get 4
    i32.add
    local.get 2
    local.get 3
    call $memcpy
    drop
    local.get 0
    local.get 3
    i32.store offset=4
    local.get 1
    local.get 4
    local.get 3
    i32.add
    i32.store offset=8
    local.get 0
    i32.const 4
    i32.store8)
  (func $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$14write_vectored17h0a5a512fe70cd54eE (type 11) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 3
        br_if 0 (;@2;)
        i32.const 0
        local.set 4
        br 1 (;@1;)
      end
      local.get 3
      i32.const -1
      i32.add
      i32.const 536870911
      i32.and
      local.tee 5
      i32.const 1
      i32.add
      local.tee 4
      i32.const 7
      i32.and
      local.set 6
      block  ;; label = @2
        block  ;; label = @3
          local.get 5
          i32.const 7
          i32.ge_u
          br_if 0 (;@3;)
          i32.const 0
          local.set 4
          local.get 2
          local.set 5
          br 1 (;@2;)
        end
        local.get 2
        i32.const 60
        i32.add
        local.set 5
        local.get 4
        i32.const 1073741816
        i32.and
        local.set 7
        i32.const 0
        local.set 4
        loop  ;; label = @3
          local.get 5
          i32.load
          local.get 5
          i32.const -8
          i32.add
          i32.load
          local.get 5
          i32.const -16
          i32.add
          i32.load
          local.get 5
          i32.const -24
          i32.add
          i32.load
          local.get 5
          i32.const -32
          i32.add
          i32.load
          local.get 5
          i32.const -40
          i32.add
          i32.load
          local.get 5
          i32.const -48
          i32.add
          i32.load
          local.get 5
          i32.const -56
          i32.add
          i32.load
          local.get 4
          i32.add
          i32.add
          i32.add
          i32.add
          i32.add
          i32.add
          i32.add
          i32.add
          local.set 4
          local.get 5
          i32.const 64
          i32.add
          local.set 5
          local.get 7
          i32.const -8
          i32.add
          local.tee 7
          br_if 0 (;@3;)
        end
        local.get 5
        i32.const -60
        i32.add
        local.set 5
      end
      block  ;; label = @2
        local.get 6
        i32.eqz
        br_if 0 (;@2;)
        local.get 5
        i32.const 4
        i32.add
        local.set 5
        loop  ;; label = @3
          local.get 5
          i32.load
          local.get 4
          i32.add
          local.set 4
          local.get 5
          i32.const 8
          i32.add
          local.set 5
          local.get 6
          i32.const -1
          i32.add
          local.tee 6
          br_if 0 (;@3;)
        end
      end
      block  ;; label = @2
        local.get 1
        i32.load
        local.get 1
        i32.load offset=8
        local.tee 5
        i32.sub
        local.get 4
        i32.ge_u
        br_if 0 (;@2;)
        local.get 1
        local.get 5
        local.get 4
        call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17h8af0b8a49dc47a67E
      end
      local.get 3
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      local.get 3
      i32.const 3
      i32.shl
      i32.add
      local.set 3
      local.get 1
      i32.load offset=8
      local.set 5
      loop  ;; label = @2
        local.get 2
        i32.load
        local.set 7
        block  ;; label = @3
          local.get 1
          i32.load
          local.get 5
          i32.sub
          local.get 2
          i32.const 4
          i32.add
          i32.load
          local.tee 6
          i32.ge_u
          br_if 0 (;@3;)
          local.get 1
          local.get 5
          local.get 6
          call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17h8af0b8a49dc47a67E
          local.get 1
          i32.load offset=8
          local.set 5
        end
        local.get 1
        i32.load offset=4
        local.get 5
        i32.add
        local.get 7
        local.get 6
        call $memcpy
        drop
        local.get 1
        local.get 5
        local.get 6
        i32.add
        local.tee 5
        i32.store offset=8
        local.get 2
        i32.const 8
        i32.add
        local.tee 2
        local.get 3
        i32.ne
        br_if 0 (;@2;)
      end
    end
    local.get 0
    i32.const 4
    i32.store8
    local.get 0
    local.get 4
    i32.store offset=4)
  (func $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$17is_write_vectored17heaadadfc4ca80892E (type 4) (param i32) (result i32)
    i32.const 1)
  (func $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$9write_all17hc56765d60e1b01daE (type 11) (param i32 i32 i32 i32)
    (local i32)
    block  ;; label = @1
      local.get 1
      i32.load
      local.get 1
      i32.load offset=8
      local.tee 4
      i32.sub
      local.get 3
      i32.ge_u
      br_if 0 (;@1;)
      local.get 1
      local.get 4
      local.get 3
      call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17h8af0b8a49dc47a67E
      local.get 1
      i32.load offset=8
      local.set 4
    end
    local.get 1
    i32.load offset=4
    local.get 4
    i32.add
    local.get 2
    local.get 3
    call $memcpy
    drop
    local.get 0
    i32.const 4
    i32.store8
    local.get 1
    local.get 4
    local.get 3
    i32.add
    i32.store offset=8)
  (func $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$5flush17hf40739bb6c56c242E (type 6) (param i32 i32)
    local.get 0
    i32.const 4
    i32.store8)
  (func $_ZN3std2io5Write18write_all_vectored17hb70535989338ad69E (type 11) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i64 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 3
            i32.eqz
            br_if 0 (;@4;)
            local.get 2
            i32.const 4
            i32.add
            local.set 5
            local.get 3
            i32.const 3
            i32.shl
            local.set 6
            local.get 3
            i32.const -1
            i32.add
            i32.const 536870911
            i32.and
            i32.const 1
            i32.add
            local.set 7
            i32.const 0
            local.set 8
            block  ;; label = @5
              loop  ;; label = @6
                local.get 5
                i32.load
                br_if 1 (;@5;)
                local.get 5
                i32.const 8
                i32.add
                local.set 5
                local.get 8
                i32.const 1
                i32.add
                local.set 8
                local.get 6
                i32.const -8
                i32.add
                local.tee 6
                br_if 0 (;@6;)
              end
              local.get 7
              local.set 8
            end
            block  ;; label = @5
              local.get 8
              local.get 3
              i32.le_u
              br_if 0 (;@5;)
              local.get 8
              local.get 3
              i32.const 1049636
              call $_ZN4core5slice5index26slice_start_index_len_fail17h12ea13b84dc5aa33E
              unreachable
            end
            local.get 3
            local.get 8
            i32.sub
            local.tee 9
            i32.eqz
            br_if 0 (;@4;)
            local.get 2
            local.get 8
            i32.const 3
            i32.shl
            i32.add
            local.set 10
            i32.const 1049760
            i64.extend_i32_u
            i64.const 32
            i64.shl
            i64.const 2
            i64.or
            local.set 11
            loop  ;; label = @5
              local.get 4
              i32.const 8
              i32.add
              i32.const 2
              local.get 10
              local.get 9
              call $_ZN4wasi13lib_generated8fd_write17h78b16dde3600cdd5E
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 4
                      i32.load16_u offset=8
                      br_if 0 (;@9;)
                      local.get 4
                      i32.load offset=12
                      local.tee 7
                      i32.eqz
                      br_if 1 (;@8;)
                      local.get 10
                      i32.const 4
                      i32.add
                      local.set 5
                      local.get 9
                      i32.const 3
                      i32.shl
                      local.set 2
                      local.get 9
                      i32.const -1
                      i32.add
                      i32.const 536870911
                      i32.and
                      i32.const 1
                      i32.add
                      local.set 12
                      i32.const 0
                      local.set 8
                      i32.const 0
                      local.set 6
                      block  ;; label = @10
                        loop  ;; label = @11
                          local.get 5
                          i32.load
                          local.get 6
                          i32.add
                          local.tee 3
                          local.get 7
                          i32.gt_u
                          br_if 1 (;@10;)
                          local.get 5
                          i32.const 8
                          i32.add
                          local.set 5
                          local.get 8
                          i32.const 1
                          i32.add
                          local.set 8
                          local.get 3
                          local.set 6
                          local.get 2
                          i32.const -8
                          i32.add
                          local.tee 2
                          br_if 0 (;@11;)
                        end
                        local.get 3
                        local.set 6
                        local.get 12
                        local.set 8
                      end
                      local.get 9
                      local.get 8
                      i32.lt_u
                      br_if 7 (;@2;)
                      local.get 9
                      local.get 8
                      i32.sub
                      local.set 3
                      local.get 10
                      local.get 8
                      i32.const 3
                      i32.shl
                      local.tee 2
                      i32.add
                      local.set 5
                      local.get 9
                      local.get 8
                      i32.ne
                      br_if 2 (;@7;)
                      local.get 5
                      local.set 10
                      local.get 3
                      local.set 9
                      local.get 7
                      local.get 6
                      i32.eq
                      br_if 3 (;@6;)
                      local.get 4
                      i32.const 20
                      i32.add
                      i32.const 1
                      i32.store
                      local.get 4
                      i32.const 28
                      i32.add
                      i32.const 0
                      i32.store
                      local.get 4
                      i32.const 1049692
                      i32.store offset=16
                      local.get 4
                      i32.const 1048832
                      i32.store offset=24
                      local.get 4
                      i32.const 0
                      i32.store offset=8
                      local.get 4
                      i32.const 8
                      i32.add
                      i32.const 1049700
                      call $_ZN4core9panicking9panic_fmt17h0724b0a601bd3679E
                      unreachable
                    end
                    local.get 4
                    local.get 4
                    i32.load16_u offset=10
                    i32.store16 offset=6
                    local.get 4
                    i32.const 6
                    i32.add
                    call $_ZN4wasi13lib_generated5Errno3raw17h169d0715fcc2f361E
                    local.tee 5
                    i32.const 65535
                    i32.and
                    call $_ZN3std3sys4wasi17decode_error_kind17h037df60161d49d56E
                    i32.const 255
                    i32.and
                    i32.const 35
                    i32.eq
                    br_if 2 (;@6;)
                    local.get 5
                    i64.extend_i32_u
                    i64.const 65535
                    i64.and
                    i64.const 32
                    i64.shl
                    local.set 11
                  end
                  local.get 0
                  local.get 11
                  i64.store align=4
                  br 4 (;@3;)
                end
                local.get 10
                local.get 2
                i32.add
                local.tee 2
                i32.load offset=4
                local.tee 9
                local.get 7
                local.get 6
                i32.sub
                local.tee 8
                i32.lt_u
                br_if 5 (;@1;)
                local.get 2
                i32.const 4
                i32.add
                local.get 9
                local.get 8
                i32.sub
                i32.store
                local.get 5
                local.get 5
                i32.load
                local.get 8
                i32.add
                i32.store
                local.get 5
                local.set 10
                local.get 3
                local.set 9
              end
              local.get 9
              br_if 0 (;@5;)
            end
          end
          local.get 0
          i32.const 4
          i32.store8
        end
        local.get 4
        i32.const 32
        i32.add
        global.set $__stack_pointer
        return
      end
      local.get 8
      local.get 9
      i32.const 1049636
      call $_ZN4core5slice5index26slice_start_index_len_fail17h12ea13b84dc5aa33E
      unreachable
    end
    local.get 4
    i32.const 20
    i32.add
    i32.const 1
    i32.store
    local.get 4
    i32.const 28
    i32.add
    i32.const 0
    i32.store
    local.get 4
    i32.const 1051968
    i32.store offset=16
    local.get 4
    i32.const 1048832
    i32.store offset=24
    local.get 4
    i32.const 0
    i32.store offset=8
    local.get 4
    i32.const 8
    i32.add
    i32.const 1052008
    call $_ZN4core9panicking9panic_fmt17h0724b0a601bd3679E
    unreachable)
  (func $_ZN3std2io5Write9write_all17h8a8bac0433a121c1E (type 11) (param i32 i32 i32 i32)
    (local i32 i64 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        local.get 3
        i32.eqz
        br_if 0 (;@2;)
        i32.const 1049760
        i64.extend_i32_u
        i64.const 32
        i64.shl
        i64.const 2
        i64.or
        local.set 5
        loop  ;; label = @3
          local.get 4
          local.get 3
          i32.store offset=12
          local.get 4
          local.get 2
          i32.store offset=8
          local.get 4
          i32.const 16
          i32.add
          i32.const 2
          local.get 4
          i32.const 8
          i32.add
          i32.const 1
          call $_ZN4wasi13lib_generated8fd_write17h78b16dde3600cdd5E
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 4
                  i32.load16_u offset=16
                  br_if 0 (;@7;)
                  local.get 4
                  i32.load offset=20
                  local.tee 6
                  i32.eqz
                  br_if 1 (;@6;)
                  local.get 3
                  local.get 6
                  i32.lt_u
                  br_if 2 (;@5;)
                  local.get 2
                  local.get 6
                  i32.add
                  local.set 2
                  local.get 3
                  local.get 6
                  i32.sub
                  local.set 3
                  br 3 (;@4;)
                end
                local.get 4
                local.get 4
                i32.load16_u offset=18
                i32.store16 offset=30
                local.get 4
                i32.const 30
                i32.add
                call $_ZN4wasi13lib_generated5Errno3raw17h169d0715fcc2f361E
                local.tee 6
                i32.const 65535
                i32.and
                call $_ZN3std3sys4wasi17decode_error_kind17h037df60161d49d56E
                i32.const 255
                i32.and
                i32.const 35
                i32.eq
                br_if 2 (;@4;)
                local.get 6
                i64.extend_i32_u
                i64.const 65535
                i64.and
                i64.const 32
                i64.shl
                local.set 5
              end
              local.get 0
              local.get 5
              i64.store align=4
              br 4 (;@1;)
            end
            local.get 6
            local.get 3
            i32.const 1049716
            call $_ZN4core5slice5index26slice_start_index_len_fail17h12ea13b84dc5aa33E
            unreachable
          end
          local.get 3
          br_if 0 (;@3;)
        end
      end
      local.get 0
      i32.const 4
      i32.store8
    end
    local.get 4
    i32.const 32
    i32.add
    global.set $__stack_pointer)
  (func $_ZN3std2io5Write18write_all_vectored17he23b35b7a95124f8E (type 11) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 3
            i32.eqz
            br_if 0 (;@4;)
            local.get 2
            i32.const 4
            i32.add
            local.set 5
            local.get 3
            i32.const 3
            i32.shl
            local.set 6
            local.get 3
            i32.const -1
            i32.add
            i32.const 536870911
            i32.and
            i32.const 1
            i32.add
            local.set 7
            i32.const 0
            local.set 8
            block  ;; label = @5
              loop  ;; label = @6
                local.get 5
                i32.load
                br_if 1 (;@5;)
                local.get 5
                i32.const 8
                i32.add
                local.set 5
                local.get 8
                i32.const 1
                i32.add
                local.set 8
                local.get 6
                i32.const -8
                i32.add
                local.tee 6
                br_if 0 (;@6;)
              end
              local.get 7
              local.set 8
            end
            block  ;; label = @5
              local.get 8
              local.get 3
              i32.le_u
              br_if 0 (;@5;)
              local.get 8
              local.get 3
              i32.const 1049636
              call $_ZN4core5slice5index26slice_start_index_len_fail17h12ea13b84dc5aa33E
              unreachable
            end
            local.get 3
            local.get 8
            i32.sub
            local.tee 9
            i32.eqz
            br_if 0 (;@4;)
            local.get 2
            local.get 8
            i32.const 3
            i32.shl
            i32.add
            local.set 10
            loop  ;; label = @5
              local.get 9
              i32.const -1
              i32.add
              i32.const 536870911
              i32.and
              local.tee 6
              i32.const 1
              i32.add
              local.tee 11
              i32.const 7
              i32.and
              local.set 8
              i32.const 0
              local.set 3
              local.get 10
              local.set 5
              block  ;; label = @6
                local.get 6
                i32.const 7
                i32.lt_u
                br_if 0 (;@6;)
                local.get 10
                i32.const 60
                i32.add
                local.set 5
                local.get 11
                i32.const 1073741816
                i32.and
                local.set 6
                i32.const 0
                local.set 3
                loop  ;; label = @7
                  local.get 5
                  i32.load
                  local.get 5
                  i32.const -8
                  i32.add
                  i32.load
                  local.get 5
                  i32.const -16
                  i32.add
                  i32.load
                  local.get 5
                  i32.const -24
                  i32.add
                  i32.load
                  local.get 5
                  i32.const -32
                  i32.add
                  i32.load
                  local.get 5
                  i32.const -40
                  i32.add
                  i32.load
                  local.get 5
                  i32.const -48
                  i32.add
                  i32.load
                  local.get 5
                  i32.const -56
                  i32.add
                  i32.load
                  local.get 3
                  i32.add
                  i32.add
                  i32.add
                  i32.add
                  i32.add
                  i32.add
                  i32.add
                  i32.add
                  local.set 3
                  local.get 5
                  i32.const 64
                  i32.add
                  local.set 5
                  local.get 6
                  i32.const -8
                  i32.add
                  local.tee 6
                  br_if 0 (;@7;)
                end
                local.get 5
                i32.const -60
                i32.add
                local.set 5
              end
              block  ;; label = @6
                local.get 8
                i32.eqz
                br_if 0 (;@6;)
                local.get 5
                i32.const 4
                i32.add
                local.set 5
                loop  ;; label = @7
                  local.get 5
                  i32.load
                  local.get 3
                  i32.add
                  local.set 3
                  local.get 5
                  i32.const 8
                  i32.add
                  local.set 5
                  local.get 8
                  i32.const -1
                  i32.add
                  local.tee 8
                  br_if 0 (;@7;)
                end
              end
              local.get 9
              i32.const 3
              i32.shl
              local.set 12
              block  ;; label = @6
                local.get 1
                i32.load
                local.get 1
                i32.load offset=8
                local.tee 5
                i32.sub
                local.get 3
                i32.ge_u
                br_if 0 (;@6;)
                local.get 1
                local.get 5
                local.get 3
                call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17h8af0b8a49dc47a67E
                local.get 1
                i32.load offset=8
                local.set 5
              end
              local.get 10
              local.get 12
              i32.add
              local.set 7
              local.get 10
              local.set 8
              loop  ;; label = @6
                local.get 8
                i32.load
                local.set 2
                block  ;; label = @7
                  local.get 1
                  i32.load
                  local.get 5
                  i32.sub
                  local.get 8
                  i32.const 4
                  i32.add
                  i32.load
                  local.tee 6
                  i32.ge_u
                  br_if 0 (;@7;)
                  local.get 1
                  local.get 5
                  local.get 6
                  call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17h8af0b8a49dc47a67E
                  local.get 1
                  i32.load offset=8
                  local.set 5
                end
                local.get 1
                i32.load offset=4
                local.get 5
                i32.add
                local.get 2
                local.get 6
                call $memcpy
                drop
                local.get 1
                local.get 5
                local.get 6
                i32.add
                local.tee 5
                i32.store offset=8
                local.get 8
                i32.const 8
                i32.add
                local.tee 8
                local.get 7
                i32.ne
                br_if 0 (;@6;)
              end
              block  ;; label = @6
                local.get 3
                br_if 0 (;@6;)
                local.get 0
                i32.const 1049760
                i64.extend_i32_u
                i64.const 32
                i64.shl
                i64.const 2
                i64.or
                i64.store align=4
                br 3 (;@3;)
              end
              local.get 10
              i32.const 4
              i32.add
              local.set 5
              i32.const 0
              local.set 8
              i32.const 0
              local.set 6
              block  ;; label = @6
                loop  ;; label = @7
                  local.get 5
                  i32.load
                  local.get 6
                  i32.add
                  local.tee 2
                  local.get 3
                  i32.gt_u
                  br_if 1 (;@6;)
                  local.get 5
                  i32.const 8
                  i32.add
                  local.set 5
                  local.get 8
                  i32.const 1
                  i32.add
                  local.set 8
                  local.get 2
                  local.set 6
                  local.get 12
                  i32.const -8
                  i32.add
                  local.tee 12
                  br_if 0 (;@7;)
                end
                local.get 2
                local.set 6
                local.get 11
                local.set 8
              end
              local.get 9
              local.get 8
              i32.lt_u
              br_if 3 (;@2;)
              local.get 10
              local.get 8
              i32.const 3
              i32.shl
              local.tee 2
              i32.add
              local.set 5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 9
                  local.get 8
                  i32.ne
                  br_if 0 (;@7;)
                  local.get 3
                  local.get 6
                  i32.eq
                  br_if 1 (;@6;)
                  local.get 4
                  i32.const 20
                  i32.add
                  i32.const 1
                  i32.store
                  local.get 4
                  i32.const 28
                  i32.add
                  i32.const 0
                  i32.store
                  local.get 4
                  i32.const 1049692
                  i32.store offset=16
                  local.get 4
                  i32.const 1048832
                  i32.store offset=24
                  local.get 4
                  i32.const 0
                  i32.store offset=8
                  local.get 4
                  i32.const 8
                  i32.add
                  i32.const 1049700
                  call $_ZN4core9panicking9panic_fmt17h0724b0a601bd3679E
                  unreachable
                end
                local.get 10
                local.get 2
                i32.add
                local.tee 2
                i32.load offset=4
                local.tee 7
                local.get 3
                local.get 6
                i32.sub
                local.tee 6
                i32.lt_u
                br_if 5 (;@1;)
                local.get 2
                i32.const 4
                i32.add
                local.get 7
                local.get 6
                i32.sub
                i32.store
                local.get 5
                local.get 5
                i32.load
                local.get 6
                i32.add
                i32.store
              end
              local.get 5
              local.set 10
              local.get 9
              local.get 8
              i32.sub
              local.tee 9
              br_if 0 (;@5;)
            end
          end
          local.get 0
          i32.const 4
          i32.store8
        end
        local.get 4
        i32.const 32
        i32.add
        global.set $__stack_pointer
        return
      end
      local.get 8
      local.get 9
      i32.const 1049636
      call $_ZN4core5slice5index26slice_start_index_len_fail17h12ea13b84dc5aa33E
      unreachable
    end
    local.get 4
    i32.const 20
    i32.add
    i32.const 1
    i32.store
    local.get 4
    i32.const 28
    i32.add
    i32.const 0
    i32.store
    local.get 4
    i32.const 1051968
    i32.store offset=16
    local.get 4
    i32.const 1048832
    i32.store offset=24
    local.get 4
    i32.const 0
    i32.store offset=8
    local.get 4
    i32.const 8
    i32.add
    i32.const 1052008
    call $_ZN4core9panicking9panic_fmt17h0724b0a601bd3679E
    unreachable)
  (func $_ZN3std2io5Write9write_fmt17h3a41373f70f0d61fE (type 5) (param i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    i32.const 4
    i32.store8 offset=8
    local.get 3
    local.get 1
    i32.store offset=16
    local.get 3
    i32.const 24
    i32.add
    i32.const 16
    i32.add
    local.get 2
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 3
    i32.const 24
    i32.add
    i32.const 8
    i32.add
    local.get 2
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 3
    local.get 2
    i64.load align=4
    i64.store offset=24
    block  ;; label = @1
      block  ;; label = @2
        local.get 3
        i32.const 8
        i32.add
        i32.const 1049772
        local.get 3
        i32.const 24
        i32.add
        call $_ZN4core3fmt5write17h56e014c9dc514616E
        i32.eqz
        br_if 0 (;@2;)
        block  ;; label = @3
          local.get 3
          i32.load8_u offset=8
          i32.const 4
          i32.ne
          br_if 0 (;@3;)
          local.get 0
          i32.const 1049812
          i32.store offset=4
          local.get 0
          i32.const 2
          i32.store
          br 2 (;@1;)
        end
        local.get 0
        local.get 3
        i64.load offset=8
        i64.store align=4
        br 1 (;@1;)
      end
      local.get 0
      i32.const 4
      i32.store8
      local.get 3
      i32.load8_u offset=8
      i32.const 3
      i32.ne
      br_if 0 (;@1;)
      local.get 3
      i32.load offset=12
      local.tee 2
      i32.load
      local.get 2
      i32.load offset=4
      i32.load
      call_indirect (type 1)
      block  ;; label = @2
        local.get 2
        i32.load offset=4
        local.tee 0
        i32.const 4
        i32.add
        i32.load
        local.tee 1
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        i32.load
        local.get 1
        local.get 0
        i32.const 8
        i32.add
        i32.load
        call $__rust_dealloc
      end
      local.get 3
      i32.load offset=12
      i32.const 12
      i32.const 4
      call $__rust_dealloc
    end
    local.get 3
    i32.const 48
    i32.add
    global.set $__stack_pointer)
  (func $_ZN80_$LT$std..io..Write..write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17h7eb9288352c31cc7E (type 7) (param i32 i32 i32) (result i32)
    (local i32)
    block  ;; label = @1
      local.get 0
      i32.load offset=8
      local.tee 0
      i32.load
      local.get 0
      i32.load offset=8
      local.tee 3
      i32.sub
      local.get 2
      i32.ge_u
      br_if 0 (;@1;)
      local.get 0
      local.get 3
      local.get 2
      call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17h8af0b8a49dc47a67E
      local.get 0
      i32.load offset=8
      local.set 3
    end
    local.get 0
    i32.load offset=4
    local.get 3
    i32.add
    local.get 1
    local.get 2
    call $memcpy
    drop
    local.get 0
    local.get 3
    local.get 2
    i32.add
    i32.store offset=8
    i32.const 0)
  (func $_ZN3std5panic19get_backtrace_style17h7338251cd2c38d6bE (type 10) (result i32)
    (local i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 0
    global.set $__stack_pointer
    i32.const 0
    local.set 1
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              i32.const 0
              i32.load offset=1059092
              br_table 3 (;@2;) 4 (;@1;) 1 (;@4;) 2 (;@3;) 0 (;@5;)
            end
            i32.const 1048911
            i32.const 40
            i32.const 1049936
            call $_ZN4core9panicking5panic17h33aac59f7456cb9eE
            unreachable
          end
          i32.const 1
          local.set 1
          br 2 (;@1;)
        end
        i32.const 2
        local.set 1
        br 1 (;@1;)
      end
      local.get 0
      i32.const 1049324
      i32.const 14
      call $_ZN3std3env7_var_os17h37aa4fcd300c1389E
      block  ;; label = @2
        block  ;; label = @3
          local.get 0
          i32.load offset=4
          local.tee 2
          i32.eqz
          br_if 0 (;@3;)
          i32.const 0
          local.set 1
          local.get 0
          i32.load
          local.set 3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 0
                i32.load offset=8
                i32.const -1
                i32.add
                br_table 0 (;@6;) 2 (;@4;) 2 (;@4;) 1 (;@5;) 2 (;@4;)
              end
              local.get 2
              i32.load8_u
              i32.const 48
              i32.eq
              i32.const 1
              i32.shl
              local.set 1
              br 1 (;@4;)
            end
            local.get 2
            i32.load align=1
            i32.const 1819047270
            i32.eq
            local.set 1
          end
          block  ;; label = @4
            local.get 3
            i32.eqz
            br_if 0 (;@4;)
            local.get 2
            local.get 3
            i32.const 1
            call $__rust_dealloc
          end
          local.get 1
          i32.const 1
          i32.add
          local.set 2
          br 1 (;@2;)
        end
        i32.const 3
        local.set 2
        i32.const 2
        local.set 1
      end
      i32.const 0
      local.get 2
      i32.store offset=1059092
    end
    local.get 0
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 1)
  (func $_ZN3std7process5abort17h9f8484c62a35da1fE (type 0)
    call $_ZN3std3sys4wasi14abort_internal17h9975d127165b10b0E
    unreachable)
  (func $_ZN3std4sync9once_lock17OnceLock$LT$T$GT$10initialize17hf3f033e66ad6a56aE (type 1) (param i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    block  ;; label = @1
      i32.const 0
      i32.load8_u offset=1059088
      i32.const 3
      i32.eq
      br_if 0 (;@1;)
      local.get 1
      i32.const 1059056
      i32.store offset=12
      local.get 1
      local.get 0
      i32.store offset=8
      local.get 1
      local.get 1
      i32.const 24
      i32.add
      i32.store offset=16
      local.get 1
      i32.const 8
      i32.add
      call $_ZN3std3sys4wasi4once4Once4call17h3e8c13c60ef4e141E
    end
    local.get 1
    i32.const 32
    i32.add
    global.set $__stack_pointer)
  (func $_ZN3std3sys4wasi4once4Once4call17h3e8c13c60ef4e141E (type 1) (param i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            i32.const 0
            i32.load8_u offset=1059088
            local.tee 2
            i32.const 2
            i32.lt_u
            br_if 0 (;@4;)
            local.get 2
            i32.const -2
            i32.add
            br_table 3 (;@1;) 1 (;@3;) 3 (;@1;)
          end
          i32.const 0
          i32.const 2
          i32.store8 offset=1059088
          local.get 0
          i32.load
          local.set 2
          local.get 0
          i32.const 0
          i32.store
          local.get 2
          i32.eqz
          br_if 1 (;@2;)
          local.get 0
          i32.load offset=4
          local.set 0
          local.get 2
          i32.const 1
          i32.store8
          local.get 0
          i32.const 8
          i32.add
          i64.const 0
          i64.store align=4
          local.get 0
          i64.const 0
          i64.store align=4
          local.get 0
          i32.const 0
          i32.store8 offset=28
          local.get 0
          i32.const 0
          i32.store8 offset=24
          local.get 0
          i64.const 1
          i64.store offset=16 align=4
          i32.const 0
          i32.const 3
          i32.store8 offset=1059088
        end
        local.get 1
        i32.const 32
        i32.add
        global.set $__stack_pointer
        return
      end
      i32.const 1048868
      i32.const 43
      i32.const 1050012
      call $_ZN4core9panicking5panic17h33aac59f7456cb9eE
      unreachable
    end
    local.get 1
    i32.const 20
    i32.add
    i32.const 1
    i32.store
    local.get 1
    i32.const 28
    i32.add
    i32.const 0
    i32.store
    local.get 1
    i32.const 1052228
    i32.store offset=16
    local.get 1
    i32.const 1048832
    i32.store offset=24
    local.get 1
    i32.const 0
    i32.store offset=8
    local.get 1
    i32.const 8
    i32.add
    i32.const 1049996
    call $_ZN4core9panicking9panic_fmt17h0724b0a601bd3679E
    unreachable)
  (func $_ZN3std3sys6common14small_c_string24run_with_cstr_allocating17h138d4eb697526d6cE (type 5) (param i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    local.get 1
    local.get 2
    call $_ZN72_$LT$$RF$str$u20$as$u20$alloc..ffi..c_str..CString..new..SpecNewImpl$GT$13spec_new_impl17hc2e87543d0ee2275E
    block  ;; label = @1
      block  ;; label = @2
        local.get 3
        i32.load offset=8
        local.tee 2
        br_if 0 (;@2;)
        local.get 0
        local.get 3
        i32.load
        local.tee 1
        local.get 3
        i32.load offset=4
        local.tee 2
        call $_ZN3std3sys4wasi2fs11open_parent28_$u7b$$u7b$closure$u7d$$u7d$17h7132f8abd0247809E
        local.get 1
        i32.const 0
        i32.store8
        local.get 2
        i32.eqz
        br_if 1 (;@1;)
        local.get 1
        local.get 2
        i32.const 1
        call $__rust_dealloc
        br 1 (;@1;)
      end
      local.get 0
      i32.const -1
      i32.store
      local.get 0
      i32.const 0
      i64.load offset=1050176
      i64.store offset=4 align=4
      local.get 3
      i32.load offset=4
      local.tee 0
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      local.get 0
      i32.const 1
      call $__rust_dealloc
    end
    local.get 3
    i32.const 16
    i32.add
    global.set $__stack_pointer)
  (func $_ZN3std3sys4wasi2fs11open_parent28_$u7b$$u7b$closure$u7d$$u7d$17h7132f8abd0247809E (type 5) (param i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 80
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    local.get 2
    i32.store offset=4
    local.get 3
    local.get 1
    i32.store
    i32.const 512
    local.set 2
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                i32.const 512
                i32.const 1
                call $__rust_alloc
                local.tee 4
                i32.eqz
                br_if 0 (;@6;)
                local.get 3
                local.get 4
                i32.store offset=12
                local.get 3
                i32.const 512
                i32.store offset=8
                local.get 3
                local.get 4
                i32.store offset=24
                local.get 3
                i32.const 0
                i32.store offset=28
                local.get 1
                local.get 3
                i32.const 28
                i32.add
                local.get 3
                i32.const 24
                i32.add
                i32.const 512
                call $__wasilibc_find_relpath
                local.tee 5
                i32.const -1
                i32.ne
                br_if 2 (;@4;)
                i32.const 512
                local.set 2
                i32.const 0
                i32.load offset=1059656
                i32.const 48
                i32.ne
                br_if 1 (;@5;)
                i32.const 512
                local.set 2
                loop  ;; label = @7
                  local.get 3
                  local.get 2
                  i32.store offset=16
                  local.get 3
                  i32.const 8
                  i32.add
                  local.get 2
                  i32.const 1
                  call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17h8af0b8a49dc47a67E
                  local.get 3
                  local.get 3
                  i32.load offset=12
                  local.tee 4
                  i32.store offset=24
                  local.get 3
                  i32.const 0
                  i32.store offset=28
                  local.get 1
                  local.get 3
                  i32.const 28
                  i32.add
                  local.get 3
                  i32.const 24
                  i32.add
                  local.get 3
                  i32.load offset=8
                  local.tee 2
                  call $__wasilibc_find_relpath
                  local.tee 5
                  i32.const -1
                  i32.ne
                  br_if 3 (;@4;)
                  i32.const 0
                  i32.load offset=1059656
                  i32.const 48
                  i32.ne
                  br_if 2 (;@5;)
                  br 0 (;@7;)
                end
              end
              i32.const 512
              i32.const 1
              call $_ZN5alloc5alloc18handle_alloc_error17h4888856d90202140E
              unreachable
            end
            local.get 3
            i32.const 48
            i32.add
            i32.const 12
            i32.add
            i32.const 2
            i32.store
            local.get 3
            i32.const 68
            i32.add
            i32.const 1
            i32.store
            local.get 3
            i32.const 1051916
            i32.store offset=56
            local.get 3
            i32.const 0
            i32.store offset=48
            local.get 3
            i32.const 9
            i32.store offset=76
            local.get 3
            local.get 3
            i32.const 72
            i32.add
            i32.store offset=64
            local.get 3
            local.get 3
            i32.store offset=72
            local.get 3
            i32.const 32
            i32.add
            local.get 3
            i32.const 48
            i32.add
            call $_ZN5alloc3fmt6format12format_inner17hff04a732548263f0E
            block  ;; label = @5
              block  ;; label = @6
                i32.const 12
                i32.const 4
                call $__rust_alloc
                local.tee 5
                i32.eqz
                br_if 0 (;@6;)
                local.get 5
                local.get 3
                i64.load offset=32
                i64.store align=4
                local.get 5
                i32.const 8
                i32.add
                local.get 3
                i32.const 32
                i32.add
                i32.const 8
                i32.add
                i32.load
                i32.store
                i32.const 12
                i32.const 4
                call $__rust_alloc
                local.tee 1
                i32.eqz
                br_if 1 (;@5;)
                local.get 1
                i32.const 40
                i32.store8 offset=8
                local.get 1
                i32.const 1048712
                i32.store offset=4
                local.get 1
                local.get 5
                i32.store
                local.get 0
                i32.const -1
                i32.store
                local.get 0
                local.get 1
                i64.extend_i32_u
                i64.const 32
                i64.shl
                i64.const 3
                i64.or
                i64.store offset=4 align=4
                br 3 (;@3;)
              end
              i32.const 12
              i32.const 4
              call $_ZN5alloc5alloc18handle_alloc_error17h4888856d90202140E
              unreachable
            end
            i32.const 12
            i32.const 4
            call $_ZN5alloc5alloc18handle_alloc_error17h4888856d90202140E
            unreachable
          end
          block  ;; label = @4
            block  ;; label = @5
              local.get 3
              i32.load offset=24
              local.tee 6
              call $_ZN4core3ffi5c_str4CStr8from_ptr9strlen_rt17h15b37b953b28d4f9E
              local.tee 1
              br_if 0 (;@5;)
              i32.const 1
              local.set 7
              br 1 (;@4;)
            end
            local.get 1
            i32.const -1
            i32.gt_s
            local.tee 8
            i32.eqz
            br_if 2 (;@2;)
            local.get 1
            local.get 8
            call $__rust_alloc
            local.tee 7
            i32.eqz
            br_if 3 (;@1;)
          end
          local.get 7
          local.get 6
          local.get 1
          call $memcpy
          local.set 6
          local.get 0
          local.get 1
          i32.store offset=12
          local.get 0
          local.get 6
          i32.store offset=8
          local.get 0
          local.get 1
          i32.store offset=4
          local.get 0
          local.get 5
          i32.store
        end
        block  ;; label = @3
          local.get 2
          i32.eqz
          br_if 0 (;@3;)
          local.get 4
          local.get 2
          i32.const 1
          call $__rust_dealloc
        end
        local.get 3
        i32.const 80
        i32.add
        global.set $__stack_pointer
        return
      end
      call $_ZN5alloc7raw_vec17capacity_overflow17hbb754ab7ed70e9c1E
      unreachable
    end
    local.get 1
    local.get 8
    call $_ZN5alloc5alloc18handle_alloc_error17h4888856d90202140E
    unreachable)
  (func $_ZN3std10sys_common9backtrace5print17ha2c0437b18512c3dE (type 11) (param i32 i32 i32 i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    i32.const 0
    i32.load8_u offset=1059096
    local.set 5
    i32.const 1
    local.set 6
    i32.const 0
    i32.const 1
    i32.store8 offset=1059096
    local.get 4
    local.get 5
    i32.store8 offset=32
    block  ;; label = @1
      local.get 5
      br_if 0 (;@1;)
      block  ;; label = @2
        i32.const 0
        i32.load offset=1059120
        i32.const 2147483647
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        call $_ZN3std9panicking11panic_count17is_zero_slow_path17h17f5a317560a2ca8E
        local.set 6
      end
      local.get 2
      i32.const 36
      i32.add
      i32.load
      local.set 5
      local.get 4
      i32.const 20
      i32.add
      i32.const 1
      i32.store
      local.get 4
      i32.const 28
      i32.add
      i32.const 1
      i32.store
      local.get 4
      i32.const 1049428
      i32.store offset=16
      local.get 4
      i32.const 0
      i32.store offset=8
      local.get 4
      i32.const 10
      i32.store offset=36
      local.get 4
      local.get 3
      i32.store8 offset=47
      local.get 4
      local.get 4
      i32.const 32
      i32.add
      i32.store offset=24
      local.get 4
      local.get 4
      i32.const 47
      i32.add
      i32.store offset=32
      local.get 0
      local.get 1
      local.get 4
      i32.const 8
      i32.add
      local.get 5
      call_indirect (type 5)
      block  ;; label = @2
        local.get 6
        i32.eqz
        br_if 0 (;@2;)
        i32.const 0
        i32.load offset=1059120
        i32.const 2147483647
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        call $_ZN3std9panicking11panic_count17is_zero_slow_path17h17f5a317560a2ca8E
        br_if 0 (;@2;)
        i32.const 0
        i32.const 1
        i32.store8 offset=1059097
      end
      i32.const 0
      i32.const 0
      i32.store8 offset=1059096
      local.get 4
      i32.const 48
      i32.add
      global.set $__stack_pointer
      return
    end
    local.get 4
    i32.const 0
    i32.store offset=28
    local.get 4
    i32.const 1048832
    i32.store offset=24
    local.get 4
    i32.const 1
    i32.store offset=20
    local.get 4
    i32.const 1052056
    i32.store offset=16
    local.get 4
    i32.const 0
    i32.store offset=8
    local.get 4
    i32.const 32
    i32.add
    local.get 4
    i32.const 8
    i32.add
    call $_ZN4core9panicking13assert_failed17haed743e2be7b9a20E
    unreachable)
  (func $_ZN91_$LT$std..sys_common..backtrace.._print..DisplayBacktrace$u20$as$u20$core..fmt..Display$GT$3fmt17h1b268c0cbb1a2de3E (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 0
    i32.load8_u
    local.set 3
    local.get 2
    i32.const 8
    i32.add
    call $_ZN3std3env11current_dir17h8a78ebea161e6b13E
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        i32.load offset=8
        br_if 0 (;@2;)
        local.get 2
        i32.const 16
        i32.add
        i32.load
        local.set 0
        local.get 2
        i32.load offset=12
        local.set 4
        br 1 (;@1;)
      end
      i32.const 0
      local.set 0
      block  ;; label = @2
        local.get 2
        i32.load8_u offset=12
        i32.const 3
        i32.ne
        br_if 0 (;@2;)
        local.get 2
        i32.const 8
        i32.add
        i32.const 8
        i32.add
        i32.load
        local.tee 5
        i32.load
        local.get 5
        i32.load offset=4
        i32.load
        call_indirect (type 1)
        block  ;; label = @3
          local.get 5
          i32.load offset=4
          local.tee 4
          i32.const 4
          i32.add
          i32.load
          local.tee 6
          i32.eqz
          br_if 0 (;@3;)
          local.get 5
          i32.load
          local.get 6
          local.get 4
          i32.const 8
          i32.add
          i32.load
          call $__rust_dealloc
        end
        local.get 5
        i32.const 12
        i32.const 4
        call $__rust_dealloc
      end
    end
    i32.const 1
    local.set 5
    local.get 2
    i32.const 20
    i32.add
    i32.const 1
    i32.store
    local.get 2
    i32.const 28
    i32.add
    i32.const 0
    i32.store
    local.get 2
    i32.const 1050204
    i32.store offset=16
    local.get 2
    i32.const 1048832
    i32.store offset=24
    local.get 2
    i32.const 0
    i32.store offset=8
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 1
          local.get 2
          i32.const 8
          i32.add
          call $_ZN4core3fmt9Formatter9write_fmt17hd7aea38882dc75edE
          br_if 0 (;@3;)
          block  ;; label = @4
            local.get 3
            i32.const 255
            i32.and
            br_if 0 (;@4;)
            local.get 2
            i32.const 20
            i32.add
            i32.const 1
            i32.store
            local.get 2
            i32.const 28
            i32.add
            i32.const 0
            i32.store
            local.get 2
            i32.const 1050300
            i32.store offset=16
            local.get 2
            i32.const 1048832
            i32.store offset=24
            local.get 2
            i32.const 0
            i32.store offset=8
            local.get 1
            local.get 2
            i32.const 8
            i32.add
            call $_ZN4core3fmt9Formatter9write_fmt17hd7aea38882dc75edE
            br_if 1 (;@3;)
          end
          i32.const 0
          local.set 5
          local.get 0
          i32.eqz
          br_if 2 (;@1;)
          local.get 4
          i32.eqz
          br_if 2 (;@1;)
          br 1 (;@2;)
        end
        local.get 0
        i32.eqz
        br_if 1 (;@1;)
        local.get 4
        i32.eqz
        br_if 1 (;@1;)
      end
      local.get 0
      local.get 4
      i32.const 1
      call $__rust_dealloc
    end
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
    local.get 5)
  (func $_ZN3std10sys_common9backtrace26__rust_end_short_backtrace17h0b174033ce8c7f4eE (type 1) (param i32)
    local.get 0
    call $_ZN3std9panicking19begin_panic_handler28_$u7b$$u7b$closure$u7d$$u7d$17h376f292379b1d694E
    unreachable)
  (func $_ZN3std9panicking19begin_panic_handler28_$u7b$$u7b$closure$u7d$$u7d$17h376f292379b1d694E (type 1) (param i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    local.get 0
    i32.load
    local.tee 2
    i32.const 20
    i32.add
    i32.load
    local.set 3
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 2
            i32.const 12
            i32.add
            i32.load
            br_table 0 (;@4;) 1 (;@3;) 3 (;@1;)
          end
          local.get 3
          br_if 2 (;@1;)
          i32.const 1048832
          local.set 2
          i32.const 0
          local.set 3
          br 1 (;@2;)
        end
        local.get 3
        br_if 1 (;@1;)
        local.get 2
        i32.load offset=8
        local.tee 2
        i32.load offset=4
        local.set 3
        local.get 2
        i32.load
        local.set 2
      end
      local.get 1
      local.get 3
      i32.store offset=4
      local.get 1
      local.get 2
      i32.store
      local.get 1
      i32.const 1050936
      local.get 0
      i32.load offset=4
      local.tee 2
      call $_ZN4core5panic10panic_info9PanicInfo7message17hf52e4c2cd1887affE
      local.get 0
      i32.load offset=8
      local.get 2
      call $_ZN4core5panic10panic_info9PanicInfo10can_unwind17h3bf06bc3f77600faE
      call $_ZN3std9panicking20rust_panic_with_hook17h51603b60b23a0fb8E
      unreachable
    end
    local.get 1
    i32.const 0
    i32.store offset=4
    local.get 1
    local.get 2
    i32.store offset=12
    local.get 1
    i32.const 1050916
    local.get 0
    i32.load offset=4
    local.tee 2
    call $_ZN4core5panic10panic_info9PanicInfo7message17hf52e4c2cd1887affE
    local.get 0
    i32.load offset=8
    local.get 2
    call $_ZN4core5panic10panic_info9PanicInfo10can_unwind17h3bf06bc3f77600faE
    call $_ZN3std9panicking20rust_panic_with_hook17h51603b60b23a0fb8E
    unreachable)
  (func $_ZN3std5alloc24default_alloc_error_hook17h910158b746051dc0E (type 6) (param i32 i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 64
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block  ;; label = @1
      i32.const 0
      i32.load8_u offset=1059052
      br_if 0 (;@1;)
      local.get 2
      i32.const 11
      i32.store offset=4
      local.get 2
      local.get 0
      i32.store offset=12
      local.get 2
      local.get 2
      i32.const 12
      i32.add
      i32.store
      local.get 2
      i32.const 4
      i32.store8 offset=16
      local.get 2
      local.get 2
      i32.const 56
      i32.add
      i32.store offset=24
      local.get 2
      i32.const 1
      i32.store offset=52
      local.get 2
      i32.const 2
      i32.store offset=44
      local.get 2
      i32.const 1050540
      i32.store offset=40
      local.get 2
      i32.const 0
      i32.store offset=32
      local.get 2
      local.get 2
      i32.store offset=48
      local.get 2
      i32.const 16
      i32.add
      i32.const 1049824
      local.get 2
      i32.const 32
      i32.add
      call $_ZN4core3fmt5write17h56e014c9dc514616E
      local.set 0
      local.get 2
      i32.load8_u offset=16
      local.set 3
      block  ;; label = @2
        block  ;; label = @3
          local.get 0
          i32.eqz
          br_if 0 (;@3;)
          local.get 3
          i32.const 255
          i32.and
          i32.const 4
          i32.eq
          br_if 1 (;@2;)
          local.get 2
          i32.load8_u offset=16
          i32.const 3
          i32.ne
          br_if 1 (;@2;)
          local.get 2
          i32.load offset=20
          local.tee 0
          i32.load
          local.get 0
          i32.load offset=4
          i32.load
          call_indirect (type 1)
          block  ;; label = @4
            local.get 0
            i32.load offset=4
            local.tee 3
            i32.const 4
            i32.add
            i32.load
            local.tee 4
            i32.eqz
            br_if 0 (;@4;)
            local.get 0
            i32.load
            local.get 4
            local.get 3
            i32.const 8
            i32.add
            i32.load
            call $__rust_dealloc
          end
          local.get 0
          i32.const 12
          i32.const 4
          call $__rust_dealloc
          br 1 (;@2;)
        end
        local.get 3
        i32.const 255
        i32.and
        i32.const 3
        i32.ne
        br_if 0 (;@2;)
        local.get 2
        i32.load offset=20
        local.tee 0
        i32.load
        local.get 0
        i32.load offset=4
        i32.load
        call_indirect (type 1)
        block  ;; label = @3
          local.get 0
          i32.load offset=4
          local.tee 3
          i32.const 4
          i32.add
          i32.load
          local.tee 4
          i32.eqz
          br_if 0 (;@3;)
          local.get 0
          i32.load
          local.get 4
          local.get 3
          i32.const 8
          i32.add
          i32.load
          call $__rust_dealloc
        end
        local.get 2
        i32.load offset=20
        i32.const 12
        i32.const 4
        call $__rust_dealloc
      end
      local.get 2
      i32.const 64
      i32.add
      global.set $__stack_pointer
      return
    end
    local.get 2
    i32.const 44
    i32.add
    i32.const 2
    i32.store
    local.get 2
    i32.const 52
    i32.add
    i32.const 1
    i32.store
    local.get 2
    i32.const 1050468
    i32.store offset=40
    local.get 2
    i32.const 0
    i32.store offset=32
    local.get 2
    i32.const 11
    i32.store offset=20
    local.get 2
    local.get 0
    i32.store
    local.get 2
    local.get 2
    i32.const 16
    i32.add
    i32.store offset=48
    local.get 2
    local.get 2
    i32.store offset=16
    local.get 2
    i32.const 32
    i32.add
    i32.const 1050508
    call $_ZN4core9panicking9panic_fmt17h0724b0a601bd3679E
    unreachable)
  (func $__rdl_alloc (type 2) (param i32 i32) (result i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.const 8
        i32.gt_u
        br_if 0 (;@2;)
        local.get 1
        local.get 0
        i32.le_u
        br_if 1 (;@1;)
      end
      local.get 1
      local.get 0
      call $aligned_alloc
      return
    end
    local.get 0
    call $malloc)
  (func $__rdl_dealloc (type 5) (param i32 i32 i32)
    local.get 0
    call $free)
  (func $__rdl_realloc (type 8) (param i32 i32 i32 i32) (result i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        i32.const 8
        i32.gt_u
        br_if 0 (;@2;)
        local.get 2
        local.get 3
        i32.le_u
        br_if 1 (;@1;)
      end
      block  ;; label = @2
        local.get 2
        local.get 3
        call $aligned_alloc
        local.tee 2
        br_if 0 (;@2;)
        i32.const 0
        return
      end
      local.get 2
      local.get 0
      local.get 1
      local.get 3
      local.get 1
      local.get 3
      i32.lt_u
      select
      call $memcpy
      local.set 3
      local.get 0
      call $free
      local.get 3
      return
    end
    local.get 0
    local.get 3
    call $realloc)
  (func $_ZN3std9panicking12default_hook28_$u7b$$u7b$closure$u7d$$u7d$17hf6bb6a273094adb0E (type 5) (param i32 i32 i32)
    (local i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 64
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    i32.const 16
    i32.add
    i32.const 12
    i32.add
    i32.const 4
    i32.store
    local.get 3
    i32.const 16
    i32.add
    i32.const 20
    i32.add
    i32.const 3
    i32.store
    local.get 3
    i32.const 40
    i32.add
    i32.const 20
    i32.add
    i32.const 12
    i32.store
    local.get 3
    i32.const 40
    i32.add
    i32.const 12
    i32.add
    i32.const 13
    i32.store
    local.get 3
    i32.const 1050732
    i32.store offset=24
    local.get 3
    i32.const 0
    i32.store offset=16
    local.get 3
    i32.const 13
    i32.store offset=44
    local.get 3
    local.get 0
    i32.load offset=8
    i32.store offset=56
    local.get 3
    local.get 0
    i32.load offset=4
    i32.store offset=48
    local.get 3
    local.get 0
    i32.load
    i32.store offset=40
    local.get 3
    local.get 3
    i32.const 40
    i32.add
    i32.store offset=32
    local.get 3
    i32.const 8
    i32.add
    local.get 1
    local.get 3
    i32.const 16
    i32.add
    local.get 2
    i32.load offset=36
    local.tee 4
    call_indirect (type 5)
    block  ;; label = @1
      local.get 3
      i32.load8_u offset=8
      i32.const 3
      i32.ne
      br_if 0 (;@1;)
      local.get 3
      i32.load offset=12
      local.tee 5
      i32.load
      local.get 5
      i32.load offset=4
      i32.load
      call_indirect (type 1)
      block  ;; label = @2
        local.get 5
        i32.load offset=4
        local.tee 6
        i32.const 4
        i32.add
        i32.load
        local.tee 7
        i32.eqz
        br_if 0 (;@2;)
        local.get 5
        i32.load
        local.get 7
        local.get 6
        i32.const 8
        i32.add
        i32.load
        call $__rust_dealloc
      end
      local.get 5
      i32.const 12
      i32.const 4
      call $__rust_dealloc
    end
    block  ;; label = @1
      local.get 0
      i32.load offset=12
      i32.load8_u
      local.tee 0
      i32.const 3
      i32.eq
      br_if 0 (;@1;)
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 0
            br_table 0 (;@4;) 1 (;@3;) 2 (;@2;) 0 (;@4;)
          end
          local.get 3
          i32.const 40
          i32.add
          local.get 1
          local.get 2
          i32.const 0
          call $_ZN3std10sys_common9backtrace5print17ha2c0437b18512c3dE
          local.get 3
          i32.load8_u offset=40
          i32.const 3
          i32.ne
          br_if 2 (;@1;)
          local.get 3
          i32.load offset=44
          local.tee 0
          i32.load
          local.get 0
          i32.load offset=4
          i32.load
          call_indirect (type 1)
          block  ;; label = @4
            local.get 0
            i32.load offset=4
            local.tee 1
            i32.const 4
            i32.add
            i32.load
            local.tee 2
            i32.eqz
            br_if 0 (;@4;)
            local.get 0
            i32.load
            local.get 2
            local.get 1
            i32.const 8
            i32.add
            i32.load
            call $__rust_dealloc
          end
          local.get 0
          i32.const 12
          i32.const 4
          call $__rust_dealloc
          br 2 (;@1;)
        end
        local.get 3
        i32.const 40
        i32.add
        local.get 1
        local.get 2
        i32.const 1
        call $_ZN3std10sys_common9backtrace5print17ha2c0437b18512c3dE
        local.get 3
        i32.load8_u offset=40
        i32.const 3
        i32.ne
        br_if 1 (;@1;)
        local.get 3
        i32.load offset=44
        local.tee 0
        i32.load
        local.get 0
        i32.load offset=4
        i32.load
        call_indirect (type 1)
        block  ;; label = @3
          local.get 0
          i32.load offset=4
          local.tee 1
          i32.const 4
          i32.add
          i32.load
          local.tee 2
          i32.eqz
          br_if 0 (;@3;)
          local.get 0
          i32.load
          local.get 2
          local.get 1
          i32.const 8
          i32.add
          i32.load
          call $__rust_dealloc
        end
        local.get 0
        i32.const 12
        i32.const 4
        call $__rust_dealloc
        br 1 (;@1;)
      end
      i32.const 0
      i32.load8_u offset=1059032
      local.set 0
      i32.const 0
      i32.const 0
      i32.store8 offset=1059032
      local.get 0
      i32.eqz
      br_if 0 (;@1;)
      local.get 3
      i32.const 52
      i32.add
      i32.const 1
      i32.store
      local.get 3
      i32.const 60
      i32.add
      i32.const 0
      i32.store
      local.get 3
      i32.const 1050844
      i32.store offset=48
      local.get 3
      i32.const 1048832
      i32.store offset=56
      local.get 3
      i32.const 0
      i32.store offset=40
      local.get 3
      i32.const 16
      i32.add
      local.get 1
      local.get 3
      i32.const 40
      i32.add
      local.get 4
      call_indirect (type 5)
      local.get 3
      i32.load8_u offset=16
      i32.const 3
      i32.ne
      br_if 0 (;@1;)
      local.get 3
      i32.load offset=20
      local.tee 0
      i32.load
      local.get 0
      i32.load offset=4
      i32.load
      call_indirect (type 1)
      block  ;; label = @2
        local.get 0
        i32.load offset=4
        local.tee 1
        i32.const 4
        i32.add
        i32.load
        local.tee 2
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        i32.load
        local.get 2
        local.get 1
        i32.const 8
        i32.add
        i32.load
        call $__rust_dealloc
      end
      local.get 0
      i32.const 12
      i32.const 4
      call $__rust_dealloc
    end
    local.get 3
    i32.const 64
    i32.add
    global.set $__stack_pointer)
  (func $rust_begin_unwind (type 1) (param i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        call $_ZN4core5panic10panic_info9PanicInfo8location17hac97990c2b530673E
        local.tee 2
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        call $_ZN4core5panic10panic_info9PanicInfo7message17hf52e4c2cd1887affE
        local.tee 3
        i32.eqz
        br_if 1 (;@1;)
        local.get 1
        local.get 2
        i32.store offset=8
        local.get 1
        local.get 0
        i32.store offset=4
        local.get 1
        local.get 3
        i32.store
        local.get 1
        call $_ZN3std10sys_common9backtrace26__rust_end_short_backtrace17h0b174033ce8c7f4eE
        unreachable
      end
      i32.const 1048868
      i32.const 43
      i32.const 1050868
      call $_ZN4core9panicking5panic17h33aac59f7456cb9eE
      unreachable
    end
    i32.const 1048868
    i32.const 43
    i32.const 1050852
    call $_ZN4core9panicking5panic17h33aac59f7456cb9eE
    unreachable)
  (func $_ZN90_$LT$std..panicking..begin_panic_handler..PanicPayload$u20$as$u20$core..panic..BoxMeUp$GT$8take_box17h897bde01b250c956E (type 6) (param i32 i32)
    (local i32 i32 i32 i64)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block  ;; label = @1
      local.get 1
      i32.load offset=4
      br_if 0 (;@1;)
      local.get 1
      i32.load offset=12
      local.set 3
      local.get 2
      i32.const 16
      i32.add
      i32.const 8
      i32.add
      local.tee 4
      i32.const 0
      i32.store
      local.get 2
      i64.const 4294967296
      i64.store offset=16
      local.get 2
      local.get 2
      i32.const 16
      i32.add
      i32.store offset=28
      local.get 2
      i32.const 28
      i32.add
      i32.const 1048808
      local.get 3
      call $_ZN4core3fmt5write17h56e014c9dc514616E
      drop
      local.get 1
      i32.const 8
      i32.add
      local.get 4
      i32.load
      i32.store
      local.get 1
      local.get 2
      i64.load offset=16
      i64.store align=4
    end
    local.get 1
    i64.load align=4
    local.set 5
    local.get 1
    i64.const 4294967296
    i64.store align=4
    local.get 2
    i32.const 8
    i32.add
    local.tee 3
    local.get 1
    i32.const 8
    i32.add
    local.tee 1
    i32.load
    i32.store
    local.get 1
    i32.const 0
    i32.store
    local.get 2
    local.get 5
    i64.store
    block  ;; label = @1
      i32.const 12
      i32.const 4
      call $__rust_alloc
      local.tee 1
      br_if 0 (;@1;)
      i32.const 12
      i32.const 4
      call $_ZN5alloc5alloc18handle_alloc_error17h4888856d90202140E
      unreachable
    end
    local.get 1
    local.get 2
    i64.load
    i64.store align=4
    local.get 1
    i32.const 8
    i32.add
    local.get 3
    i32.load
    i32.store
    local.get 0
    i32.const 1050884
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer)
  (func $_ZN90_$LT$std..panicking..begin_panic_handler..PanicPayload$u20$as$u20$core..panic..BoxMeUp$GT$3get17h4fea0969750388b6E (type 6) (param i32 i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block  ;; label = @1
      local.get 1
      i32.load offset=4
      br_if 0 (;@1;)
      local.get 1
      i32.load offset=12
      local.set 3
      local.get 2
      i32.const 8
      i32.add
      local.tee 4
      i32.const 0
      i32.store
      local.get 2
      i64.const 4294967296
      i64.store
      local.get 2
      local.get 2
      i32.store offset=12
      local.get 2
      i32.const 12
      i32.add
      i32.const 1048808
      local.get 3
      call $_ZN4core3fmt5write17h56e014c9dc514616E
      drop
      local.get 1
      i32.const 8
      i32.add
      local.get 4
      i32.load
      i32.store
      local.get 1
      local.get 2
      i64.load
      i64.store align=4
    end
    local.get 0
    i32.const 1050884
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer)
  (func $_ZN93_$LT$std..panicking..begin_panic_handler..StrPanicPayload$u20$as$u20$core..panic..BoxMeUp$GT$8take_box17h8b0280949f0541bdE (type 6) (param i32 i32)
    (local i32 i32)
    local.get 1
    i32.load offset=4
    local.set 2
    local.get 1
    i32.load
    local.set 3
    block  ;; label = @1
      i32.const 8
      i32.const 4
      call $__rust_alloc
      local.tee 1
      br_if 0 (;@1;)
      i32.const 8
      i32.const 4
      call $_ZN5alloc5alloc18handle_alloc_error17h4888856d90202140E
      unreachable
    end
    local.get 1
    local.get 2
    i32.store offset=4
    local.get 1
    local.get 3
    i32.store
    local.get 0
    i32.const 1050900
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store)
  (func $_ZN93_$LT$std..panicking..begin_panic_handler..StrPanicPayload$u20$as$u20$core..panic..BoxMeUp$GT$3get17hdc699511fb6d5496E (type 6) (param i32 i32)
    local.get 0
    i32.const 1050900
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store)
  (func $_ZN3std9panicking20rust_panic_with_hook17h51603b60b23a0fb8E (type 13) (param i32 i32 i32 i32 i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 112
    i32.sub
    local.tee 5
    global.set $__stack_pointer
    i32.const 0
    i32.const 0
    i32.load offset=1059120
    local.tee 6
    i32.const 1
    i32.add
    i32.store offset=1059120
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 6
                    i32.const -1
                    i32.gt_s
                    br_if 0 (;@8;)
                    local.get 6
                    i32.const 2147483647
                    i32.and
                    i32.const 2
                    i32.gt_u
                    br_if 2 (;@6;)
                    local.get 5
                    local.get 4
                    i32.store8 offset=64
                    local.get 5
                    local.get 3
                    i32.store offset=60
                    local.get 5
                    local.get 2
                    i32.store offset=56
                    local.get 5
                    i32.const 1051016
                    i32.store offset=52
                    local.get 5
                    i32.const 1048832
                    i32.store offset=48
                    local.get 5
                    i32.const 14
                    i32.store offset=76
                    local.get 5
                    local.get 5
                    i32.const 48
                    i32.add
                    i32.store offset=72
                    local.get 5
                    i32.const 4
                    i32.store8 offset=16
                    local.get 5
                    local.get 5
                    i32.const 104
                    i32.add
                    i32.store offset=24
                    local.get 5
                    i32.const 1
                    i32.store offset=100
                    local.get 5
                    i32.const 2
                    i32.store offset=92
                    local.get 5
                    i32.const 1051084
                    i32.store offset=88
                    local.get 5
                    i32.const 0
                    i32.store offset=80
                    local.get 5
                    local.get 5
                    i32.const 72
                    i32.add
                    i32.store offset=96
                    local.get 5
                    i32.const 16
                    i32.add
                    i32.const 1049824
                    local.get 5
                    i32.const 80
                    i32.add
                    call $_ZN4core3fmt5write17h56e014c9dc514616E
                    local.set 6
                    local.get 5
                    i32.load8_u offset=16
                    local.set 4
                    local.get 6
                    i32.eqz
                    br_if 1 (;@7;)
                    local.get 4
                    i32.const 255
                    i32.and
                    i32.const 4
                    i32.eq
                    br_if 7 (;@1;)
                    local.get 5
                    i32.load8_u offset=16
                    i32.const 3
                    i32.ne
                    br_if 7 (;@1;)
                    local.get 5
                    i32.load offset=20
                    local.tee 5
                    i32.load
                    local.get 5
                    i32.load offset=4
                    i32.load
                    call_indirect (type 1)
                    block  ;; label = @9
                      local.get 5
                      i32.load offset=4
                      local.tee 6
                      i32.const 4
                      i32.add
                      i32.load
                      local.tee 4
                      i32.eqz
                      br_if 0 (;@9;)
                      local.get 5
                      i32.load
                      local.get 4
                      local.get 6
                      i32.const 8
                      i32.add
                      i32.load
                      call $__rust_dealloc
                    end
                    local.get 5
                    i32.const 12
                    i32.const 4
                    call $__rust_dealloc
                    call $_ZN3std3sys4wasi14abort_internal17h9975d127165b10b0E
                    unreachable
                  end
                  i32.const 0
                  i32.const 0
                  i32.load offset=1059156
                  i32.const 1
                  i32.add
                  local.tee 6
                  i32.store offset=1059156
                  local.get 6
                  i32.const 2
                  i32.gt_u
                  br_if 1 (;@6;)
                  local.get 5
                  local.get 4
                  i32.store8 offset=32
                  local.get 5
                  local.get 3
                  i32.store offset=28
                  local.get 5
                  local.get 2
                  i32.store offset=24
                  local.get 5
                  i32.const 1051016
                  i32.store offset=20
                  local.get 5
                  i32.const 1048832
                  i32.store offset=16
                  i32.const 0
                  i32.load offset=1059104
                  local.tee 3
                  i32.const -1
                  i32.le_s
                  br_if 4 (;@3;)
                  i32.const 0
                  local.get 3
                  i32.const 1
                  i32.add
                  i32.store offset=1059104
                  i32.const 0
                  i32.load offset=1059112
                  local.set 3
                  local.get 5
                  i32.const 8
                  i32.add
                  local.get 0
                  local.get 1
                  i32.load offset=16
                  call_indirect (type 6)
                  local.get 5
                  local.get 5
                  i64.load offset=8
                  i64.store offset=16
                  local.get 3
                  br_if 2 (;@5;)
                  local.get 5
                  i32.const 16
                  i32.add
                  call $_ZN3std9panicking12default_hook17hb9a783738c35b420E
                  br 3 (;@4;)
                end
                local.get 4
                i32.const 255
                i32.and
                i32.const 3
                i32.ne
                br_if 5 (;@1;)
                local.get 5
                i32.load offset=20
                local.tee 6
                i32.load
                local.get 6
                i32.load offset=4
                i32.load
                call_indirect (type 1)
                block  ;; label = @7
                  local.get 6
                  i32.load offset=4
                  local.tee 4
                  i32.const 4
                  i32.add
                  i32.load
                  local.tee 3
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 6
                  i32.load
                  local.get 3
                  local.get 4
                  i32.const 8
                  i32.add
                  i32.load
                  call $__rust_dealloc
                end
                local.get 5
                i32.load offset=20
                i32.const 12
                i32.const 4
                call $__rust_dealloc
                call $_ZN3std3sys4wasi14abort_internal17h9975d127165b10b0E
                unreachable
              end
              local.get 5
              i32.const 4
              i32.store8 offset=48
              local.get 5
              local.get 5
              i32.const 104
              i32.add
              i32.store offset=56
              local.get 5
              i32.const 0
              i32.store offset=100
              local.get 5
              i32.const 1048832
              i32.store offset=96
              local.get 5
              i32.const 1
              i32.store offset=92
              local.get 5
              i32.const 1051008
              i32.store offset=88
              local.get 5
              i32.const 0
              i32.store offset=80
              local.get 5
              i32.const 48
              i32.add
              i32.const 1049824
              local.get 5
              i32.const 80
              i32.add
              call $_ZN4core3fmt5write17h56e014c9dc514616E
              local.set 6
              local.get 5
              i32.load8_u offset=48
              local.set 4
              block  ;; label = @6
                local.get 6
                i32.eqz
                br_if 0 (;@6;)
                local.get 4
                i32.const 255
                i32.and
                i32.const 4
                i32.eq
                br_if 5 (;@1;)
                local.get 5
                i32.load8_u offset=48
                i32.const 3
                i32.ne
                br_if 5 (;@1;)
                local.get 5
                i32.load offset=52
                local.tee 5
                i32.load
                local.get 5
                i32.load offset=4
                i32.load
                call_indirect (type 1)
                block  ;; label = @7
                  local.get 5
                  i32.load offset=4
                  local.tee 6
                  i32.const 4
                  i32.add
                  i32.load
                  local.tee 4
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 5
                  i32.load
                  local.get 4
                  local.get 6
                  i32.const 8
                  i32.add
                  i32.load
                  call $__rust_dealloc
                end
                local.get 5
                i32.const 12
                i32.const 4
                call $__rust_dealloc
                call $_ZN3std3sys4wasi14abort_internal17h9975d127165b10b0E
                unreachable
              end
              local.get 4
              i32.const 255
              i32.and
              i32.const 3
              i32.ne
              br_if 4 (;@1;)
              local.get 5
              i32.load offset=52
              local.tee 6
              i32.load
              local.get 6
              i32.load offset=4
              i32.load
              call_indirect (type 1)
              block  ;; label = @6
                local.get 6
                i32.load offset=4
                local.tee 4
                i32.const 4
                i32.add
                i32.load
                local.tee 3
                i32.eqz
                br_if 0 (;@6;)
                local.get 6
                i32.load
                local.get 3
                local.get 4
                i32.const 8
                i32.add
                i32.load
                call $__rust_dealloc
              end
              local.get 5
              i32.load offset=52
              i32.const 12
              i32.const 4
              call $__rust_dealloc
              br 4 (;@1;)
            end
            i32.const 0
            i32.load offset=1059112
            local.get 5
            i32.const 16
            i32.add
            i32.const 0
            i32.load offset=1059116
            i32.load offset=20
            call_indirect (type 6)
          end
          i32.const 0
          i32.const 0
          i32.load offset=1059104
          i32.const -1
          i32.add
          i32.store offset=1059104
          block  ;; label = @4
            local.get 6
            i32.const 1
            i32.gt_u
            br_if 0 (;@4;)
            local.get 4
            br_if 2 (;@2;)
          end
          block  ;; label = @4
            local.get 4
            i32.eqz
            br_if 0 (;@4;)
            local.get 5
            i32.const 4
            i32.store8 offset=48
            local.get 5
            local.get 5
            i32.const 104
            i32.add
            i32.store offset=56
            local.get 5
            i32.const 0
            i32.store offset=100
            local.get 5
            i32.const 1048832
            i32.store offset=96
            local.get 5
            i32.const 1
            i32.store offset=92
            local.get 5
            i32.const 1051200
            i32.store offset=88
            local.get 5
            i32.const 0
            i32.store offset=80
            local.get 5
            i32.const 48
            i32.add
            i32.const 1049824
            local.get 5
            i32.const 80
            i32.add
            call $_ZN4core3fmt5write17h56e014c9dc514616E
            local.set 6
            local.get 5
            i32.load8_u offset=48
            local.set 4
            block  ;; label = @5
              local.get 6
              i32.eqz
              br_if 0 (;@5;)
              local.get 4
              i32.const 255
              i32.and
              i32.const 4
              i32.eq
              br_if 4 (;@1;)
              local.get 5
              i32.load8_u offset=48
              i32.const 3
              i32.ne
              br_if 4 (;@1;)
              local.get 5
              i32.load offset=52
              local.tee 5
              i32.load
              local.get 5
              i32.load offset=4
              i32.load
              call_indirect (type 1)
              block  ;; label = @6
                local.get 5
                i32.load offset=4
                local.tee 6
                i32.const 4
                i32.add
                i32.load
                local.tee 4
                i32.eqz
                br_if 0 (;@6;)
                local.get 5
                i32.load
                local.get 4
                local.get 6
                i32.const 8
                i32.add
                i32.load
                call $__rust_dealloc
              end
              local.get 5
              i32.const 12
              i32.const 4
              call $__rust_dealloc
              call $_ZN3std3sys4wasi14abort_internal17h9975d127165b10b0E
              unreachable
            end
            local.get 4
            i32.const 255
            i32.and
            i32.const 3
            i32.ne
            br_if 3 (;@1;)
            local.get 5
            i32.load offset=52
            local.tee 6
            i32.load
            local.get 6
            i32.load offset=4
            i32.load
            call_indirect (type 1)
            block  ;; label = @5
              local.get 6
              i32.load offset=4
              local.tee 4
              i32.const 4
              i32.add
              i32.load
              local.tee 3
              i32.eqz
              br_if 0 (;@5;)
              local.get 6
              i32.load
              local.get 3
              local.get 4
              i32.const 8
              i32.add
              i32.load
              call $__rust_dealloc
            end
            local.get 5
            i32.load offset=52
            i32.const 12
            i32.const 4
            call $__rust_dealloc
            call $_ZN3std3sys4wasi14abort_internal17h9975d127165b10b0E
            unreachable
          end
          local.get 5
          i32.const 4
          i32.store8 offset=48
          local.get 5
          local.get 5
          i32.const 104
          i32.add
          i32.store offset=56
          local.get 5
          i32.const 0
          i32.store offset=100
          local.get 5
          i32.const 1048832
          i32.store offset=96
          local.get 5
          i32.const 1
          i32.store offset=92
          local.get 5
          i32.const 1051148
          i32.store offset=88
          local.get 5
          i32.const 0
          i32.store offset=80
          local.get 5
          i32.const 48
          i32.add
          i32.const 1049824
          local.get 5
          i32.const 80
          i32.add
          call $_ZN4core3fmt5write17h56e014c9dc514616E
          local.set 6
          local.get 5
          i32.load8_u offset=48
          local.set 4
          block  ;; label = @4
            local.get 6
            i32.eqz
            br_if 0 (;@4;)
            local.get 4
            i32.const 255
            i32.and
            i32.const 4
            i32.eq
            br_if 3 (;@1;)
            local.get 5
            i32.load8_u offset=48
            i32.const 3
            i32.ne
            br_if 3 (;@1;)
            local.get 5
            i32.load offset=52
            local.tee 5
            i32.load
            local.get 5
            i32.load offset=4
            i32.load
            call_indirect (type 1)
            block  ;; label = @5
              local.get 5
              i32.load offset=4
              local.tee 6
              i32.const 4
              i32.add
              i32.load
              local.tee 4
              i32.eqz
              br_if 0 (;@5;)
              local.get 5
              i32.load
              local.get 4
              local.get 6
              i32.const 8
              i32.add
              i32.load
              call $__rust_dealloc
            end
            local.get 5
            i32.const 12
            i32.const 4
            call $__rust_dealloc
            call $_ZN3std3sys4wasi14abort_internal17h9975d127165b10b0E
            unreachable
          end
          local.get 4
          i32.const 255
          i32.and
          i32.const 3
          i32.ne
          br_if 2 (;@1;)
          local.get 5
          i32.load offset=52
          local.tee 6
          i32.load
          local.get 6
          i32.load offset=4
          i32.load
          call_indirect (type 1)
          block  ;; label = @4
            local.get 6
            i32.load offset=4
            local.tee 4
            i32.const 4
            i32.add
            i32.load
            local.tee 3
            i32.eqz
            br_if 0 (;@4;)
            local.get 6
            i32.load
            local.get 3
            local.get 4
            i32.const 8
            i32.add
            i32.load
            call $__rust_dealloc
          end
          local.get 5
          i32.load offset=52
          i32.const 12
          i32.const 4
          call $__rust_dealloc
          call $_ZN3std3sys4wasi14abort_internal17h9975d127165b10b0E
          unreachable
        end
        local.get 5
        i32.const 48
        i32.add
        i32.const 12
        i32.add
        i32.const 2
        i32.store
        local.get 5
        i32.const 48
        i32.add
        i32.const 20
        i32.add
        i32.const 1
        i32.store
        local.get 5
        i32.const 80
        i32.add
        i32.const 12
        i32.add
        i32.const 1
        i32.store
        local.get 5
        i32.const 80
        i32.add
        i32.const 20
        i32.add
        i32.const 0
        i32.store
        local.get 5
        i32.const 1049112
        i32.store offset=56
        local.get 5
        i32.const 0
        i32.store offset=48
        local.get 5
        i32.const 7
        i32.store offset=76
        local.get 5
        i32.const 1052164
        i32.store offset=88
        local.get 5
        i32.const 1048832
        i32.store offset=96
        local.get 5
        i32.const 0
        i32.store offset=80
        local.get 5
        local.get 5
        i32.const 72
        i32.add
        i32.store offset=64
        local.get 5
        local.get 5
        i32.const 80
        i32.add
        i32.store offset=72
        local.get 5
        i32.const 40
        i32.add
        local.get 5
        i32.const 104
        i32.add
        local.get 5
        i32.const 48
        i32.add
        call $_ZN3std2io5Write9write_fmt17h73c1d085b221682dE
        local.get 5
        i32.const 40
        i32.add
        call $_ZN4core3ptr81drop_in_place$LT$core..result..Result$LT$$LP$$RP$$C$std..io..error..Error$GT$$GT$17h578f989f6735c81dE
        call $_ZN3std3sys4wasi14abort_internal17h9975d127165b10b0E
        unreachable
      end
      local.get 0
      local.get 1
      call $rust_panic
      unreachable
    end
    call $_ZN3std3sys4wasi14abort_internal17h9975d127165b10b0E
    unreachable)
  (func $rust_panic (type 6) (param i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 96
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    local.get 0
    local.get 1
    call $__rust_start_panic
    i32.store offset=12
    local.get 2
    i32.const 24
    i32.add
    i32.const 12
    i32.add
    i32.const 2
    i32.store
    local.get 2
    i32.const 24
    i32.add
    i32.const 20
    i32.add
    i32.const 1
    i32.store
    local.get 2
    i32.const 56
    i32.add
    i32.const 12
    i32.add
    i32.const 1
    i32.store
    local.get 2
    i32.const 56
    i32.add
    i32.const 20
    i32.add
    i32.const 1
    i32.store
    local.get 2
    i32.const 1049112
    i32.store offset=32
    local.get 2
    i32.const 0
    i32.store offset=24
    local.get 2
    i32.const 7
    i32.store offset=52
    local.get 2
    i32.const 1051240
    i32.store offset=64
    local.get 2
    i32.const 0
    i32.store offset=56
    local.get 2
    i32.const 11
    i32.store offset=84
    local.get 2
    local.get 2
    i32.const 48
    i32.add
    i32.store offset=40
    local.get 2
    local.get 2
    i32.const 56
    i32.add
    i32.store offset=48
    local.get 2
    local.get 2
    i32.const 80
    i32.add
    i32.store offset=72
    local.get 2
    local.get 2
    i32.const 12
    i32.add
    i32.store offset=80
    local.get 2
    i32.const 16
    i32.add
    local.get 2
    i32.const 88
    i32.add
    local.get 2
    i32.const 24
    i32.add
    call $_ZN3std2io5Write9write_fmt17h73c1d085b221682dE
    local.get 2
    i32.const 16
    i32.add
    call $_ZN4core3ptr81drop_in_place$LT$core..result..Result$LT$$LP$$RP$$C$std..io..error..Error$GT$$GT$17h578f989f6735c81dE
    call $_ZN3std3sys4wasi14abort_internal17h9975d127165b10b0E
    unreachable)
  (func $_ZN62_$LT$std..io..error..ErrorKind$u20$as$u20$core..fmt..Debug$GT$3fmt17h5659196e58541555E (type 2) (param i32 i32) (result i32)
    local.get 1
    local.get 0
    i32.load8_u
    i32.const 2
    i32.shl
    local.tee 0
    i32.const 1052596
    i32.add
    i32.load
    local.get 0
    i32.const 1052432
    i32.add
    i32.load
    call $_ZN4core3fmt9Formatter9write_str17h6749db218b079d3bE)
  (func $_ZN3std3sys4wasi2fs7open_at17h6ed0c206cbba7569E (type 13) (param i32 i32 i32 i32 i32)
    (local i32 i32 i32 i64 i64)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 5
    global.set $__stack_pointer
    local.get 4
    i32.load offset=32
    local.set 6
    local.get 5
    i32.const 24
    i32.add
    local.get 2
    local.get 3
    call $_ZN4core3str8converts9from_utf817he9b4783fc054fc5dE
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 5
          i32.load offset=24
          br_if 0 (;@3;)
          local.get 5
          i32.load offset=28
          local.tee 3
          i32.eqz
          br_if 0 (;@3;)
          local.get 5
          i32.const 32
          i32.add
          i32.load
          local.set 2
          local.get 4
          i32.load16_u offset=38
          local.set 7
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 4
                i64.load
                i64.const 0
                i64.ne
                br_if 0 (;@6;)
                i64.const 16386
                i64.const 0
                local.get 4
                i32.load8_u offset=40
                select
                local.tee 8
                i64.const 4194625
                i64.or
                local.get 8
                local.get 4
                i32.load8_u offset=41
                local.get 4
                i32.load8_u offset=42
                i32.or
                select
                i64.const 262651580
                i64.or
                local.tee 9
                local.set 8
                local.get 4
                i64.load offset=16
                i64.eqz
                i32.eqz
                br_if 1 (;@5;)
                br 2 (;@4;)
              end
              local.get 4
              i64.load offset=8
              local.tee 9
              local.set 8
              local.get 4
              i64.load offset=16
              i64.eqz
              br_if 1 (;@4;)
            end
            local.get 4
            i32.const 24
            i32.add
            i64.load
            local.set 8
          end
          local.get 5
          i32.const 8
          i32.add
          local.get 1
          local.get 6
          local.get 3
          local.get 2
          local.get 7
          local.get 9
          local.get 8
          local.get 4
          i32.load16_u offset=36
          call $_ZN4wasi13lib_generated9path_open17h1460cb18ef94d26bE
          block  ;; label = @4
            local.get 5
            i32.load16_u offset=8
            br_if 0 (;@4;)
            local.get 5
            local.get 5
            i32.load offset=12
            local.tee 4
            i32.store offset=20
            local.get 4
            i32.const -1
            i32.eq
            br_if 3 (;@1;)
            local.get 0
            i32.const 4
            i32.store8
            local.get 0
            local.get 4
            i32.store offset=4
            br 2 (;@2;)
          end
          local.get 5
          local.get 5
          i32.load16_u offset=10
          i32.store16 offset=24
          local.get 5
          i32.const 24
          i32.add
          call $_ZN4wasi13lib_generated5Errno3raw17h169d0715fcc2f361E
          local.set 4
          local.get 0
          i32.const 3
          i32.add
          i32.const 0
          i32.store8
          local.get 0
          i32.const 0
          i32.store16 offset=1 align=1
          local.get 0
          local.get 4
          i32.const 65535
          i32.and
          i32.store offset=4
          local.get 0
          i32.const 0
          i32.store8
          br 1 (;@2;)
        end
        local.get 0
        i32.const 1049868
        i32.store offset=4
        local.get 0
        i32.const 2
        i32.store
      end
      local.get 5
      i32.const 48
      i32.add
      global.set $__stack_pointer
      return
    end
    local.get 5
    i32.const 0
    i32.store offset=32
    local.get 5
    i32.const 20
    i32.add
    local.get 5
    i32.const 24
    i32.add
    call $_ZN4core9panicking13assert_failed17h56c36f852b6e67b2E
    unreachable)
  (func $_ZN3std3sys4wasi7process8ExitCode6as_i3217hc69c91814d2ce79fE (type 4) (param i32) (result i32)
    local.get 0
    i32.load8_u)
  (func $_ZN64_$LT$std..sys..wasi..stdio..Stderr$u20$as$u20$std..io..Write$GT$5write17h56809a5b3fcaeebdE (type 11) (param i32 i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    local.get 4
    local.get 3
    i32.store offset=12
    local.get 4
    local.get 2
    i32.store offset=8
    local.get 4
    i32.const 16
    i32.add
    i32.const 2
    local.get 4
    i32.const 8
    i32.add
    i32.const 1
    call $_ZN4wasi13lib_generated8fd_write17h78b16dde3600cdd5E
    block  ;; label = @1
      block  ;; label = @2
        local.get 4
        i32.load16_u offset=16
        br_if 0 (;@2;)
        local.get 0
        local.get 4
        i32.load offset=20
        i32.store offset=4
        local.get 0
        i32.const 4
        i32.store8
        br 1 (;@1;)
      end
      local.get 4
      local.get 4
      i32.load16_u offset=18
      i32.store16 offset=30
      local.get 0
      local.get 4
      i32.const 30
      i32.add
      call $_ZN4wasi13lib_generated5Errno3raw17h169d0715fcc2f361E
      i64.extend_i32_u
      i64.const 65535
      i64.and
      i64.const 32
      i64.shl
      i64.store align=4
    end
    local.get 4
    i32.const 32
    i32.add
    global.set $__stack_pointer)
  (func $_ZN64_$LT$std..sys..wasi..stdio..Stderr$u20$as$u20$std..io..Write$GT$14write_vectored17h5575c3c26ed3d779E (type 11) (param i32 i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    local.get 4
    i32.const 2
    local.get 2
    local.get 3
    call $_ZN4wasi13lib_generated8fd_write17h78b16dde3600cdd5E
    block  ;; label = @1
      block  ;; label = @2
        local.get 4
        i32.load16_u
        br_if 0 (;@2;)
        local.get 0
        local.get 4
        i32.load offset=4
        i32.store offset=4
        local.get 0
        i32.const 4
        i32.store8
        br 1 (;@1;)
      end
      local.get 4
      local.get 4
      i32.load16_u offset=2
      i32.store16 offset=14
      local.get 0
      local.get 4
      i32.const 14
      i32.add
      call $_ZN4wasi13lib_generated5Errno3raw17h169d0715fcc2f361E
      i64.extend_i32_u
      i64.const 65535
      i64.and
      i64.const 32
      i64.shl
      i64.store align=4
    end
    local.get 4
    i32.const 16
    i32.add
    global.set $__stack_pointer)
  (func $_ZN64_$LT$std..sys..wasi..stdio..Stderr$u20$as$u20$std..io..Write$GT$17is_write_vectored17h8908d08cb5e31eddE (type 4) (param i32) (result i32)
    i32.const 1)
  (func $_ZN64_$LT$std..sys..wasi..stdio..Stderr$u20$as$u20$std..io..Write$GT$5flush17h2c1e73e04b726cdeE (type 6) (param i32 i32)
    local.get 0
    i32.const 4
    i32.store8)
  (func $_ZN3std5alloc8rust_oom17h430925c776560ef5E (type 6) (param i32 i32)
    (local i32)
    local.get 0
    local.get 1
    i32.const 0
    i32.load offset=1059100
    local.tee 2
    i32.const 15
    local.get 2
    select
    call_indirect (type 6)
    call $_ZN3std7process5abort17h9f8484c62a35da1fE
    unreachable)
  (func $__rg_oom (type 6) (param i32 i32)
    local.get 0
    local.get 1
    call $_ZN3std5alloc8rust_oom17h430925c776560ef5E
    unreachable)
  (func $__rust_start_panic (type 2) (param i32 i32) (result i32)
    unreachable
    unreachable)
  (func $_ZN4wasi13lib_generated5Errno3raw17h169d0715fcc2f361E (type 4) (param i32) (result i32)
    local.get 0
    i32.load16_u)
  (func $_ZN4wasi13lib_generated8fd_write17h78b16dde3600cdd5E (type 11) (param i32 i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        local.get 2
        local.get 3
        local.get 4
        i32.const 12
        i32.add
        call $_ZN4wasi13lib_generated22wasi_snapshot_preview18fd_write17h7ef83300274a0defE
        local.tee 3
        br_if 0 (;@2;)
        local.get 0
        local.get 4
        i32.load offset=12
        i32.store offset=4
        i32.const 0
        local.set 3
        br 1 (;@1;)
      end
      local.get 0
      local.get 3
      i32.store16 offset=2
      i32.const 1
      local.set 3
    end
    local.get 0
    local.get 3
    i32.store16
    local.get 4
    i32.const 16
    i32.add
    global.set $__stack_pointer)
  (func $_ZN4wasi13lib_generated9path_open17h1460cb18ef94d26bE (type 14) (param i32 i32 i32 i32 i32 i32 i64 i64 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 9
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        local.get 2
        local.get 3
        local.get 4
        local.get 5
        i32.const 65535
        i32.and
        local.get 6
        local.get 7
        local.get 8
        i32.const 65535
        i32.and
        local.get 9
        i32.const 12
        i32.add
        call $_ZN4wasi13lib_generated22wasi_snapshot_preview19path_open17h9cf26e64f10a7e40E
        local.tee 8
        br_if 0 (;@2;)
        local.get 0
        local.get 9
        i32.load offset=12
        i32.store offset=4
        i32.const 0
        local.set 8
        br 1 (;@1;)
      end
      local.get 0
      local.get 8
      i32.store16 offset=2
      i32.const 1
      local.set 8
    end
    local.get 0
    local.get 8
    i32.store16
    local.get 9
    i32.const 16
    i32.add
    global.set $__stack_pointer)
  (func $malloc (type 4) (param i32) (result i32)
    local.get 0
    call $dlmalloc)
  (func $dlmalloc (type 4) (param i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    block  ;; label = @1
      i32.const 0
      i32.load offset=1059184
      local.tee 2
      br_if 0 (;@1;)
      block  ;; label = @2
        block  ;; label = @3
          i32.const 0
          i32.load offset=1059632
          local.tee 3
          i32.eqz
          br_if 0 (;@3;)
          i32.const 0
          i32.load offset=1059636
          local.set 4
          br 1 (;@2;)
        end
        i32.const 0
        i64.const -1
        i64.store offset=1059644 align=4
        i32.const 0
        i64.const 281474976776192
        i64.store offset=1059636 align=4
        i32.const 0
        local.get 1
        i32.const 8
        i32.add
        i32.const -16
        i32.and
        i32.const 1431655768
        i32.xor
        local.tee 3
        i32.store offset=1059632
        i32.const 0
        i32.const 0
        i32.store offset=1059652
        i32.const 0
        i32.const 0
        i32.store offset=1059604
        i32.const 65536
        local.set 4
      end
      i32.const 0
      local.set 2
      i32.const 1114112
      i32.const 1059712
      local.get 4
      i32.add
      i32.const -1
      i32.add
      i32.const 0
      local.get 4
      i32.sub
      i32.and
      i32.const 1114112
      select
      i32.const 1059712
      i32.sub
      local.tee 5
      i32.const 89
      i32.lt_u
      br_if 0 (;@1;)
      i32.const 0
      local.set 4
      i32.const 0
      local.get 5
      i32.store offset=1059612
      i32.const 0
      i32.const 1059712
      i32.store offset=1059608
      i32.const 0
      i32.const 1059712
      i32.store offset=1059176
      i32.const 0
      local.get 3
      i32.store offset=1059196
      i32.const 0
      i32.const -1
      i32.store offset=1059192
      loop  ;; label = @2
        local.get 4
        i32.const 1059220
        i32.add
        local.get 4
        i32.const 1059208
        i32.add
        local.tee 3
        i32.store
        local.get 3
        local.get 4
        i32.const 1059200
        i32.add
        local.tee 6
        i32.store
        local.get 4
        i32.const 1059212
        i32.add
        local.get 6
        i32.store
        local.get 4
        i32.const 1059228
        i32.add
        local.get 4
        i32.const 1059216
        i32.add
        local.tee 6
        i32.store
        local.get 6
        local.get 3
        i32.store
        local.get 4
        i32.const 1059236
        i32.add
        local.get 4
        i32.const 1059224
        i32.add
        local.tee 3
        i32.store
        local.get 3
        local.get 6
        i32.store
        local.get 4
        i32.const 1059232
        i32.add
        local.get 3
        i32.store
        local.get 4
        i32.const 32
        i32.add
        local.tee 4
        i32.const 256
        i32.ne
        br_if 0 (;@2;)
      end
      i32.const 1059712
      i32.const -8
      i32.const 1059712
      i32.sub
      i32.const 15
      i32.and
      i32.const 0
      i32.const 1059712
      i32.const 8
      i32.add
      i32.const 15
      i32.and
      select
      local.tee 4
      i32.add
      local.tee 2
      i32.const 4
      i32.add
      local.get 5
      i32.const -56
      i32.add
      local.tee 3
      local.get 4
      i32.sub
      local.tee 4
      i32.const 1
      i32.or
      i32.store
      i32.const 0
      i32.const 0
      i32.load offset=1059648
      i32.store offset=1059188
      i32.const 0
      local.get 4
      i32.store offset=1059172
      i32.const 0
      local.get 2
      i32.store offset=1059184
      i32.const 1059712
      local.get 3
      i32.add
      i32.const 56
      i32.store offset=4
    end
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            local.get 0
                            i32.const 236
                            i32.gt_u
                            br_if 0 (;@12;)
                            block  ;; label = @13
                              i32.const 0
                              i32.load offset=1059160
                              local.tee 7
                              i32.const 16
                              local.get 0
                              i32.const 19
                              i32.add
                              i32.const -16
                              i32.and
                              local.get 0
                              i32.const 11
                              i32.lt_u
                              select
                              local.tee 5
                              i32.const 3
                              i32.shr_u
                              local.tee 3
                              i32.shr_u
                              local.tee 4
                              i32.const 3
                              i32.and
                              i32.eqz
                              br_if 0 (;@13;)
                              block  ;; label = @14
                                block  ;; label = @15
                                  local.get 4
                                  i32.const 1
                                  i32.and
                                  local.get 3
                                  i32.or
                                  i32.const 1
                                  i32.xor
                                  local.tee 6
                                  i32.const 3
                                  i32.shl
                                  local.tee 3
                                  i32.const 1059200
                                  i32.add
                                  local.tee 4
                                  local.get 3
                                  i32.const 1059208
                                  i32.add
                                  i32.load
                                  local.tee 3
                                  i32.load offset=8
                                  local.tee 5
                                  i32.ne
                                  br_if 0 (;@15;)
                                  i32.const 0
                                  local.get 7
                                  i32.const -2
                                  local.get 6
                                  i32.rotl
                                  i32.and
                                  i32.store offset=1059160
                                  br 1 (;@14;)
                                end
                                local.get 4
                                local.get 5
                                i32.store offset=8
                                local.get 5
                                local.get 4
                                i32.store offset=12
                              end
                              local.get 3
                              i32.const 8
                              i32.add
                              local.set 4
                              local.get 3
                              local.get 6
                              i32.const 3
                              i32.shl
                              local.tee 6
                              i32.const 3
                              i32.or
                              i32.store offset=4
                              local.get 3
                              local.get 6
                              i32.add
                              local.tee 3
                              local.get 3
                              i32.load offset=4
                              i32.const 1
                              i32.or
                              i32.store offset=4
                              br 12 (;@1;)
                            end
                            local.get 5
                            i32.const 0
                            i32.load offset=1059168
                            local.tee 8
                            i32.le_u
                            br_if 1 (;@11;)
                            block  ;; label = @13
                              local.get 4
                              i32.eqz
                              br_if 0 (;@13;)
                              block  ;; label = @14
                                block  ;; label = @15
                                  local.get 4
                                  local.get 3
                                  i32.shl
                                  i32.const 2
                                  local.get 3
                                  i32.shl
                                  local.tee 4
                                  i32.const 0
                                  local.get 4
                                  i32.sub
                                  i32.or
                                  i32.and
                                  local.tee 4
                                  i32.const 0
                                  local.get 4
                                  i32.sub
                                  i32.and
                                  i32.const -1
                                  i32.add
                                  local.tee 4
                                  local.get 4
                                  i32.const 12
                                  i32.shr_u
                                  i32.const 16
                                  i32.and
                                  local.tee 4
                                  i32.shr_u
                                  local.tee 3
                                  i32.const 5
                                  i32.shr_u
                                  i32.const 8
                                  i32.and
                                  local.tee 6
                                  local.get 4
                                  i32.or
                                  local.get 3
                                  local.get 6
                                  i32.shr_u
                                  local.tee 4
                                  i32.const 2
                                  i32.shr_u
                                  i32.const 4
                                  i32.and
                                  local.tee 3
                                  i32.or
                                  local.get 4
                                  local.get 3
                                  i32.shr_u
                                  local.tee 4
                                  i32.const 1
                                  i32.shr_u
                                  i32.const 2
                                  i32.and
                                  local.tee 3
                                  i32.or
                                  local.get 4
                                  local.get 3
                                  i32.shr_u
                                  local.tee 4
                                  i32.const 1
                                  i32.shr_u
                                  i32.const 1
                                  i32.and
                                  local.tee 3
                                  i32.or
                                  local.get 4
                                  local.get 3
                                  i32.shr_u
                                  i32.add
                                  local.tee 3
                                  i32.const 3
                                  i32.shl
                                  local.tee 4
                                  i32.const 1059200
                                  i32.add
                                  local.tee 6
                                  local.get 4
                                  i32.const 1059208
                                  i32.add
                                  i32.load
                                  local.tee 4
                                  i32.load offset=8
                                  local.tee 0
                                  i32.ne
                                  br_if 0 (;@15;)
                                  i32.const 0
                                  local.get 7
                                  i32.const -2
                                  local.get 3
                                  i32.rotl
                                  i32.and
                                  local.tee 7
                                  i32.store offset=1059160
                                  br 1 (;@14;)
                                end
                                local.get 6
                                local.get 0
                                i32.store offset=8
                                local.get 0
                                local.get 6
                                i32.store offset=12
                              end
                              local.get 4
                              local.get 5
                              i32.const 3
                              i32.or
                              i32.store offset=4
                              local.get 4
                              local.get 3
                              i32.const 3
                              i32.shl
                              local.tee 3
                              i32.add
                              local.get 3
                              local.get 5
                              i32.sub
                              local.tee 6
                              i32.store
                              local.get 4
                              local.get 5
                              i32.add
                              local.tee 0
                              local.get 6
                              i32.const 1
                              i32.or
                              i32.store offset=4
                              block  ;; label = @14
                                local.get 8
                                i32.eqz
                                br_if 0 (;@14;)
                                local.get 8
                                i32.const -8
                                i32.and
                                i32.const 1059200
                                i32.add
                                local.set 5
                                i32.const 0
                                i32.load offset=1059180
                                local.set 3
                                block  ;; label = @15
                                  block  ;; label = @16
                                    local.get 7
                                    i32.const 1
                                    local.get 8
                                    i32.const 3
                                    i32.shr_u
                                    i32.shl
                                    local.tee 9
                                    i32.and
                                    br_if 0 (;@16;)
                                    i32.const 0
                                    local.get 7
                                    local.get 9
                                    i32.or
                                    i32.store offset=1059160
                                    local.get 5
                                    local.set 9
                                    br 1 (;@15;)
                                  end
                                  local.get 5
                                  i32.load offset=8
                                  local.set 9
                                end
                                local.get 9
                                local.get 3
                                i32.store offset=12
                                local.get 5
                                local.get 3
                                i32.store offset=8
                                local.get 3
                                local.get 5
                                i32.store offset=12
                                local.get 3
                                local.get 9
                                i32.store offset=8
                              end
                              local.get 4
                              i32.const 8
                              i32.add
                              local.set 4
                              i32.const 0
                              local.get 0
                              i32.store offset=1059180
                              i32.const 0
                              local.get 6
                              i32.store offset=1059168
                              br 12 (;@1;)
                            end
                            i32.const 0
                            i32.load offset=1059164
                            local.tee 10
                            i32.eqz
                            br_if 1 (;@11;)
                            local.get 10
                            i32.const 0
                            local.get 10
                            i32.sub
                            i32.and
                            i32.const -1
                            i32.add
                            local.tee 4
                            local.get 4
                            i32.const 12
                            i32.shr_u
                            i32.const 16
                            i32.and
                            local.tee 4
                            i32.shr_u
                            local.tee 3
                            i32.const 5
                            i32.shr_u
                            i32.const 8
                            i32.and
                            local.tee 6
                            local.get 4
                            i32.or
                            local.get 3
                            local.get 6
                            i32.shr_u
                            local.tee 4
                            i32.const 2
                            i32.shr_u
                            i32.const 4
                            i32.and
                            local.tee 3
                            i32.or
                            local.get 4
                            local.get 3
                            i32.shr_u
                            local.tee 4
                            i32.const 1
                            i32.shr_u
                            i32.const 2
                            i32.and
                            local.tee 3
                            i32.or
                            local.get 4
                            local.get 3
                            i32.shr_u
                            local.tee 4
                            i32.const 1
                            i32.shr_u
                            i32.const 1
                            i32.and
                            local.tee 3
                            i32.or
                            local.get 4
                            local.get 3
                            i32.shr_u
                            i32.add
                            i32.const 2
                            i32.shl
                            i32.const 1059464
                            i32.add
                            i32.load
                            local.tee 0
                            i32.load offset=4
                            i32.const -8
                            i32.and
                            local.get 5
                            i32.sub
                            local.set 3
                            local.get 0
                            local.set 6
                            block  ;; label = @13
                              loop  ;; label = @14
                                block  ;; label = @15
                                  local.get 6
                                  i32.load offset=16
                                  local.tee 4
                                  br_if 0 (;@15;)
                                  local.get 6
                                  i32.const 20
                                  i32.add
                                  i32.load
                                  local.tee 4
                                  i32.eqz
                                  br_if 2 (;@13;)
                                end
                                local.get 4
                                i32.load offset=4
                                i32.const -8
                                i32.and
                                local.get 5
                                i32.sub
                                local.tee 6
                                local.get 3
                                local.get 6
                                local.get 3
                                i32.lt_u
                                local.tee 6
                                select
                                local.set 3
                                local.get 4
                                local.get 0
                                local.get 6
                                select
                                local.set 0
                                local.get 4
                                local.set 6
                                br 0 (;@14;)
                              end
                            end
                            local.get 0
                            i32.load offset=24
                            local.set 11
                            block  ;; label = @13
                              local.get 0
                              i32.load offset=12
                              local.tee 9
                              local.get 0
                              i32.eq
                              br_if 0 (;@13;)
                              local.get 0
                              i32.load offset=8
                              local.tee 4
                              i32.const 0
                              i32.load offset=1059176
                              i32.lt_u
                              drop
                              local.get 9
                              local.get 4
                              i32.store offset=8
                              local.get 4
                              local.get 9
                              i32.store offset=12
                              br 11 (;@2;)
                            end
                            block  ;; label = @13
                              local.get 0
                              i32.const 20
                              i32.add
                              local.tee 6
                              i32.load
                              local.tee 4
                              br_if 0 (;@13;)
                              local.get 0
                              i32.load offset=16
                              local.tee 4
                              i32.eqz
                              br_if 3 (;@10;)
                              local.get 0
                              i32.const 16
                              i32.add
                              local.set 6
                            end
                            loop  ;; label = @13
                              local.get 6
                              local.set 2
                              local.get 4
                              local.tee 9
                              i32.const 20
                              i32.add
                              local.tee 6
                              i32.load
                              local.tee 4
                              br_if 0 (;@13;)
                              local.get 9
                              i32.const 16
                              i32.add
                              local.set 6
                              local.get 9
                              i32.load offset=16
                              local.tee 4
                              br_if 0 (;@13;)
                            end
                            local.get 2
                            i32.const 0
                            i32.store
                            br 10 (;@2;)
                          end
                          i32.const -1
                          local.set 5
                          local.get 0
                          i32.const -65
                          i32.gt_u
                          br_if 0 (;@11;)
                          local.get 0
                          i32.const 19
                          i32.add
                          local.tee 4
                          i32.const -16
                          i32.and
                          local.set 5
                          i32.const 0
                          i32.load offset=1059164
                          local.tee 10
                          i32.eqz
                          br_if 0 (;@11;)
                          i32.const 0
                          local.set 8
                          block  ;; label = @12
                            local.get 5
                            i32.const 256
                            i32.lt_u
                            br_if 0 (;@12;)
                            i32.const 31
                            local.set 8
                            local.get 5
                            i32.const 16777215
                            i32.gt_u
                            br_if 0 (;@12;)
                            local.get 4
                            i32.const 8
                            i32.shr_u
                            local.tee 4
                            local.get 4
                            i32.const 1048320
                            i32.add
                            i32.const 16
                            i32.shr_u
                            i32.const 8
                            i32.and
                            local.tee 4
                            i32.shl
                            local.tee 3
                            local.get 3
                            i32.const 520192
                            i32.add
                            i32.const 16
                            i32.shr_u
                            i32.const 4
                            i32.and
                            local.tee 3
                            i32.shl
                            local.tee 6
                            local.get 6
                            i32.const 245760
                            i32.add
                            i32.const 16
                            i32.shr_u
                            i32.const 2
                            i32.and
                            local.tee 6
                            i32.shl
                            i32.const 15
                            i32.shr_u
                            local.get 4
                            local.get 3
                            i32.or
                            local.get 6
                            i32.or
                            i32.sub
                            local.tee 4
                            i32.const 1
                            i32.shl
                            local.get 5
                            local.get 4
                            i32.const 21
                            i32.add
                            i32.shr_u
                            i32.const 1
                            i32.and
                            i32.or
                            i32.const 28
                            i32.add
                            local.set 8
                          end
                          i32.const 0
                          local.get 5
                          i32.sub
                          local.set 3
                          block  ;; label = @12
                            block  ;; label = @13
                              block  ;; label = @14
                                block  ;; label = @15
                                  local.get 8
                                  i32.const 2
                                  i32.shl
                                  i32.const 1059464
                                  i32.add
                                  i32.load
                                  local.tee 6
                                  br_if 0 (;@15;)
                                  i32.const 0
                                  local.set 4
                                  i32.const 0
                                  local.set 9
                                  br 1 (;@14;)
                                end
                                i32.const 0
                                local.set 4
                                local.get 5
                                i32.const 0
                                i32.const 25
                                local.get 8
                                i32.const 1
                                i32.shr_u
                                i32.sub
                                local.get 8
                                i32.const 31
                                i32.eq
                                select
                                i32.shl
                                local.set 0
                                i32.const 0
                                local.set 9
                                loop  ;; label = @15
                                  block  ;; label = @16
                                    local.get 6
                                    i32.load offset=4
                                    i32.const -8
                                    i32.and
                                    local.get 5
                                    i32.sub
                                    local.tee 7
                                    local.get 3
                                    i32.ge_u
                                    br_if 0 (;@16;)
                                    local.get 7
                                    local.set 3
                                    local.get 6
                                    local.set 9
                                    local.get 7
                                    br_if 0 (;@16;)
                                    i32.const 0
                                    local.set 3
                                    local.get 6
                                    local.set 9
                                    local.get 6
                                    local.set 4
                                    br 3 (;@13;)
                                  end
                                  local.get 4
                                  local.get 6
                                  i32.const 20
                                  i32.add
                                  i32.load
                                  local.tee 7
                                  local.get 7
                                  local.get 6
                                  local.get 0
                                  i32.const 29
                                  i32.shr_u
                                  i32.const 4
                                  i32.and
                                  i32.add
                                  i32.const 16
                                  i32.add
                                  i32.load
                                  local.tee 6
                                  i32.eq
                                  select
                                  local.get 4
                                  local.get 7
                                  select
                                  local.set 4
                                  local.get 0
                                  i32.const 1
                                  i32.shl
                                  local.set 0
                                  local.get 6
                                  br_if 0 (;@15;)
                                end
                              end
                              block  ;; label = @14
                                local.get 4
                                local.get 9
                                i32.or
                                br_if 0 (;@14;)
                                i32.const 0
                                local.set 9
                                i32.const 2
                                local.get 8
                                i32.shl
                                local.tee 4
                                i32.const 0
                                local.get 4
                                i32.sub
                                i32.or
                                local.get 10
                                i32.and
                                local.tee 4
                                i32.eqz
                                br_if 3 (;@11;)
                                local.get 4
                                i32.const 0
                                local.get 4
                                i32.sub
                                i32.and
                                i32.const -1
                                i32.add
                                local.tee 4
                                local.get 4
                                i32.const 12
                                i32.shr_u
                                i32.const 16
                                i32.and
                                local.tee 4
                                i32.shr_u
                                local.tee 6
                                i32.const 5
                                i32.shr_u
                                i32.const 8
                                i32.and
                                local.tee 0
                                local.get 4
                                i32.or
                                local.get 6
                                local.get 0
                                i32.shr_u
                                local.tee 4
                                i32.const 2
                                i32.shr_u
                                i32.const 4
                                i32.and
                                local.tee 6
                                i32.or
                                local.get 4
                                local.get 6
                                i32.shr_u
                                local.tee 4
                                i32.const 1
                                i32.shr_u
                                i32.const 2
                                i32.and
                                local.tee 6
                                i32.or
                                local.get 4
                                local.get 6
                                i32.shr_u
                                local.tee 4
                                i32.const 1
                                i32.shr_u
                                i32.const 1
                                i32.and
                                local.tee 6
                                i32.or
                                local.get 4
                                local.get 6
                                i32.shr_u
                                i32.add
                                i32.const 2
                                i32.shl
                                i32.const 1059464
                                i32.add
                                i32.load
                                local.set 4
                              end
                              local.get 4
                              i32.eqz
                              br_if 1 (;@12;)
                            end
                            loop  ;; label = @13
                              local.get 4
                              i32.load offset=4
                              i32.const -8
                              i32.and
                              local.get 5
                              i32.sub
                              local.tee 7
                              local.get 3
                              i32.lt_u
                              local.set 0
                              block  ;; label = @14
                                local.get 4
                                i32.load offset=16
                                local.tee 6
                                br_if 0 (;@14;)
                                local.get 4
                                i32.const 20
                                i32.add
                                i32.load
                                local.set 6
                              end
                              local.get 7
                              local.get 3
                              local.get 0
                              select
                              local.set 3
                              local.get 4
                              local.get 9
                              local.get 0
                              select
                              local.set 9
                              local.get 6
                              local.set 4
                              local.get 6
                              br_if 0 (;@13;)
                            end
                          end
                          local.get 9
                          i32.eqz
                          br_if 0 (;@11;)
                          local.get 3
                          i32.const 0
                          i32.load offset=1059168
                          local.get 5
                          i32.sub
                          i32.ge_u
                          br_if 0 (;@11;)
                          local.get 9
                          i32.load offset=24
                          local.set 2
                          block  ;; label = @12
                            local.get 9
                            i32.load offset=12
                            local.tee 0
                            local.get 9
                            i32.eq
                            br_if 0 (;@12;)
                            local.get 9
                            i32.load offset=8
                            local.tee 4
                            i32.const 0
                            i32.load offset=1059176
                            i32.lt_u
                            drop
                            local.get 0
                            local.get 4
                            i32.store offset=8
                            local.get 4
                            local.get 0
                            i32.store offset=12
                            br 9 (;@3;)
                          end
                          block  ;; label = @12
                            local.get 9
                            i32.const 20
                            i32.add
                            local.tee 6
                            i32.load
                            local.tee 4
                            br_if 0 (;@12;)
                            local.get 9
                            i32.load offset=16
                            local.tee 4
                            i32.eqz
                            br_if 3 (;@9;)
                            local.get 9
                            i32.const 16
                            i32.add
                            local.set 6
                          end
                          loop  ;; label = @12
                            local.get 6
                            local.set 7
                            local.get 4
                            local.tee 0
                            i32.const 20
                            i32.add
                            local.tee 6
                            i32.load
                            local.tee 4
                            br_if 0 (;@12;)
                            local.get 0
                            i32.const 16
                            i32.add
                            local.set 6
                            local.get 0
                            i32.load offset=16
                            local.tee 4
                            br_if 0 (;@12;)
                          end
                          local.get 7
                          i32.const 0
                          i32.store
                          br 8 (;@3;)
                        end
                        block  ;; label = @11
                          i32.const 0
                          i32.load offset=1059168
                          local.tee 4
                          local.get 5
                          i32.lt_u
                          br_if 0 (;@11;)
                          i32.const 0
                          i32.load offset=1059180
                          local.set 3
                          block  ;; label = @12
                            block  ;; label = @13
                              local.get 4
                              local.get 5
                              i32.sub
                              local.tee 6
                              i32.const 16
                              i32.lt_u
                              br_if 0 (;@13;)
                              local.get 3
                              local.get 5
                              i32.add
                              local.tee 0
                              local.get 6
                              i32.const 1
                              i32.or
                              i32.store offset=4
                              i32.const 0
                              local.get 6
                              i32.store offset=1059168
                              i32.const 0
                              local.get 0
                              i32.store offset=1059180
                              local.get 3
                              local.get 4
                              i32.add
                              local.get 6
                              i32.store
                              local.get 3
                              local.get 5
                              i32.const 3
                              i32.or
                              i32.store offset=4
                              br 1 (;@12;)
                            end
                            local.get 3
                            local.get 4
                            i32.const 3
                            i32.or
                            i32.store offset=4
                            local.get 3
                            local.get 4
                            i32.add
                            local.tee 4
                            local.get 4
                            i32.load offset=4
                            i32.const 1
                            i32.or
                            i32.store offset=4
                            i32.const 0
                            i32.const 0
                            i32.store offset=1059180
                            i32.const 0
                            i32.const 0
                            i32.store offset=1059168
                          end
                          local.get 3
                          i32.const 8
                          i32.add
                          local.set 4
                          br 10 (;@1;)
                        end
                        block  ;; label = @11
                          i32.const 0
                          i32.load offset=1059172
                          local.tee 6
                          local.get 5
                          i32.le_u
                          br_if 0 (;@11;)
                          local.get 2
                          local.get 5
                          i32.add
                          local.tee 4
                          local.get 6
                          local.get 5
                          i32.sub
                          local.tee 3
                          i32.const 1
                          i32.or
                          i32.store offset=4
                          i32.const 0
                          local.get 4
                          i32.store offset=1059184
                          i32.const 0
                          local.get 3
                          i32.store offset=1059172
                          local.get 2
                          local.get 5
                          i32.const 3
                          i32.or
                          i32.store offset=4
                          local.get 2
                          i32.const 8
                          i32.add
                          local.set 4
                          br 10 (;@1;)
                        end
                        block  ;; label = @11
                          block  ;; label = @12
                            i32.const 0
                            i32.load offset=1059632
                            i32.eqz
                            br_if 0 (;@12;)
                            i32.const 0
                            i32.load offset=1059640
                            local.set 3
                            br 1 (;@11;)
                          end
                          i32.const 0
                          i64.const -1
                          i64.store offset=1059644 align=4
                          i32.const 0
                          i64.const 281474976776192
                          i64.store offset=1059636 align=4
                          i32.const 0
                          local.get 1
                          i32.const 12
                          i32.add
                          i32.const -16
                          i32.and
                          i32.const 1431655768
                          i32.xor
                          i32.store offset=1059632
                          i32.const 0
                          i32.const 0
                          i32.store offset=1059652
                          i32.const 0
                          i32.const 0
                          i32.store offset=1059604
                          i32.const 65536
                          local.set 3
                        end
                        i32.const 0
                        local.set 4
                        block  ;; label = @11
                          local.get 3
                          local.get 5
                          i32.const 71
                          i32.add
                          local.tee 8
                          i32.add
                          local.tee 0
                          i32.const 0
                          local.get 3
                          i32.sub
                          local.tee 7
                          i32.and
                          local.tee 9
                          local.get 5
                          i32.gt_u
                          br_if 0 (;@11;)
                          i32.const 0
                          i32.const 48
                          i32.store offset=1059656
                          br 10 (;@1;)
                        end
                        block  ;; label = @11
                          i32.const 0
                          i32.load offset=1059600
                          local.tee 4
                          i32.eqz
                          br_if 0 (;@11;)
                          block  ;; label = @12
                            i32.const 0
                            i32.load offset=1059592
                            local.tee 3
                            local.get 9
                            i32.add
                            local.tee 10
                            local.get 3
                            i32.le_u
                            br_if 0 (;@12;)
                            local.get 10
                            local.get 4
                            i32.le_u
                            br_if 1 (;@11;)
                          end
                          i32.const 0
                          local.set 4
                          i32.const 0
                          i32.const 48
                          i32.store offset=1059656
                          br 10 (;@1;)
                        end
                        i32.const 0
                        i32.load8_u offset=1059604
                        i32.const 4
                        i32.and
                        br_if 4 (;@6;)
                        block  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              local.get 2
                              i32.eqz
                              br_if 0 (;@13;)
                              i32.const 1059608
                              local.set 4
                              loop  ;; label = @14
                                block  ;; label = @15
                                  local.get 4
                                  i32.load
                                  local.tee 3
                                  local.get 2
                                  i32.gt_u
                                  br_if 0 (;@15;)
                                  local.get 3
                                  local.get 4
                                  i32.load offset=4
                                  i32.add
                                  local.get 2
                                  i32.gt_u
                                  br_if 3 (;@12;)
                                end
                                local.get 4
                                i32.load offset=8
                                local.tee 4
                                br_if 0 (;@14;)
                              end
                            end
                            i32.const 0
                            call $sbrk
                            local.tee 0
                            i32.const -1
                            i32.eq
                            br_if 5 (;@7;)
                            local.get 9
                            local.set 7
                            block  ;; label = @13
                              i32.const 0
                              i32.load offset=1059636
                              local.tee 4
                              i32.const -1
                              i32.add
                              local.tee 3
                              local.get 0
                              i32.and
                              i32.eqz
                              br_if 0 (;@13;)
                              local.get 9
                              local.get 0
                              i32.sub
                              local.get 3
                              local.get 0
                              i32.add
                              i32.const 0
                              local.get 4
                              i32.sub
                              i32.and
                              i32.add
                              local.set 7
                            end
                            local.get 7
                            local.get 5
                            i32.le_u
                            br_if 5 (;@7;)
                            local.get 7
                            i32.const 2147483646
                            i32.gt_u
                            br_if 5 (;@7;)
                            block  ;; label = @13
                              i32.const 0
                              i32.load offset=1059600
                              local.tee 4
                              i32.eqz
                              br_if 0 (;@13;)
                              i32.const 0
                              i32.load offset=1059592
                              local.tee 3
                              local.get 7
                              i32.add
                              local.tee 6
                              local.get 3
                              i32.le_u
                              br_if 6 (;@7;)
                              local.get 6
                              local.get 4
                              i32.gt_u
                              br_if 6 (;@7;)
                            end
                            local.get 7
                            call $sbrk
                            local.tee 4
                            local.get 0
                            i32.ne
                            br_if 1 (;@11;)
                            br 7 (;@5;)
                          end
                          local.get 0
                          local.get 6
                          i32.sub
                          local.get 7
                          i32.and
                          local.tee 7
                          i32.const 2147483646
                          i32.gt_u
                          br_if 4 (;@7;)
                          local.get 7
                          call $sbrk
                          local.tee 0
                          local.get 4
                          i32.load
                          local.get 4
                          i32.load offset=4
                          i32.add
                          i32.eq
                          br_if 3 (;@8;)
                          local.get 0
                          local.set 4
                        end
                        block  ;; label = @11
                          local.get 4
                          i32.const -1
                          i32.eq
                          br_if 0 (;@11;)
                          local.get 5
                          i32.const 72
                          i32.add
                          local.get 7
                          i32.le_u
                          br_if 0 (;@11;)
                          block  ;; label = @12
                            local.get 8
                            local.get 7
                            i32.sub
                            i32.const 0
                            i32.load offset=1059640
                            local.tee 3
                            i32.add
                            i32.const 0
                            local.get 3
                            i32.sub
                            i32.and
                            local.tee 3
                            i32.const 2147483646
                            i32.le_u
                            br_if 0 (;@12;)
                            local.get 4
                            local.set 0
                            br 7 (;@5;)
                          end
                          block  ;; label = @12
                            local.get 3
                            call $sbrk
                            i32.const -1
                            i32.eq
                            br_if 0 (;@12;)
                            local.get 3
                            local.get 7
                            i32.add
                            local.set 7
                            local.get 4
                            local.set 0
                            br 7 (;@5;)
                          end
                          i32.const 0
                          local.get 7
                          i32.sub
                          call $sbrk
                          drop
                          br 4 (;@7;)
                        end
                        local.get 4
                        local.set 0
                        local.get 4
                        i32.const -1
                        i32.ne
                        br_if 5 (;@5;)
                        br 3 (;@7;)
                      end
                      i32.const 0
                      local.set 9
                      br 7 (;@2;)
                    end
                    i32.const 0
                    local.set 0
                    br 5 (;@3;)
                  end
                  local.get 0
                  i32.const -1
                  i32.ne
                  br_if 2 (;@5;)
                end
                i32.const 0
                i32.const 0
                i32.load offset=1059604
                i32.const 4
                i32.or
                i32.store offset=1059604
              end
              local.get 9
              i32.const 2147483646
              i32.gt_u
              br_if 1 (;@4;)
              local.get 9
              call $sbrk
              local.set 0
              i32.const 0
              call $sbrk
              local.set 4
              local.get 0
              i32.const -1
              i32.eq
              br_if 1 (;@4;)
              local.get 4
              i32.const -1
              i32.eq
              br_if 1 (;@4;)
              local.get 0
              local.get 4
              i32.ge_u
              br_if 1 (;@4;)
              local.get 4
              local.get 0
              i32.sub
              local.tee 7
              local.get 5
              i32.const 56
              i32.add
              i32.le_u
              br_if 1 (;@4;)
            end
            i32.const 0
            i32.const 0
            i32.load offset=1059592
            local.get 7
            i32.add
            local.tee 4
            i32.store offset=1059592
            block  ;; label = @5
              local.get 4
              i32.const 0
              i32.load offset=1059596
              i32.le_u
              br_if 0 (;@5;)
              i32.const 0
              local.get 4
              i32.store offset=1059596
            end
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    i32.const 0
                    i32.load offset=1059184
                    local.tee 3
                    i32.eqz
                    br_if 0 (;@8;)
                    i32.const 1059608
                    local.set 4
                    loop  ;; label = @9
                      local.get 0
                      local.get 4
                      i32.load
                      local.tee 6
                      local.get 4
                      i32.load offset=4
                      local.tee 9
                      i32.add
                      i32.eq
                      br_if 2 (;@7;)
                      local.get 4
                      i32.load offset=8
                      local.tee 4
                      br_if 0 (;@9;)
                      br 3 (;@6;)
                    end
                  end
                  block  ;; label = @8
                    block  ;; label = @9
                      i32.const 0
                      i32.load offset=1059176
                      local.tee 4
                      i32.eqz
                      br_if 0 (;@9;)
                      local.get 0
                      local.get 4
                      i32.ge_u
                      br_if 1 (;@8;)
                    end
                    i32.const 0
                    local.get 0
                    i32.store offset=1059176
                  end
                  i32.const 0
                  local.set 4
                  i32.const 0
                  local.get 7
                  i32.store offset=1059612
                  i32.const 0
                  local.get 0
                  i32.store offset=1059608
                  i32.const 0
                  i32.const -1
                  i32.store offset=1059192
                  i32.const 0
                  i32.const 0
                  i32.load offset=1059632
                  i32.store offset=1059196
                  i32.const 0
                  i32.const 0
                  i32.store offset=1059620
                  loop  ;; label = @8
                    local.get 4
                    i32.const 1059220
                    i32.add
                    local.get 4
                    i32.const 1059208
                    i32.add
                    local.tee 3
                    i32.store
                    local.get 3
                    local.get 4
                    i32.const 1059200
                    i32.add
                    local.tee 6
                    i32.store
                    local.get 4
                    i32.const 1059212
                    i32.add
                    local.get 6
                    i32.store
                    local.get 4
                    i32.const 1059228
                    i32.add
                    local.get 4
                    i32.const 1059216
                    i32.add
                    local.tee 6
                    i32.store
                    local.get 6
                    local.get 3
                    i32.store
                    local.get 4
                    i32.const 1059236
                    i32.add
                    local.get 4
                    i32.const 1059224
                    i32.add
                    local.tee 3
                    i32.store
                    local.get 3
                    local.get 6
                    i32.store
                    local.get 4
                    i32.const 1059232
                    i32.add
                    local.get 3
                    i32.store
                    local.get 4
                    i32.const 32
                    i32.add
                    local.tee 4
                    i32.const 256
                    i32.ne
                    br_if 0 (;@8;)
                  end
                  local.get 0
                  i32.const -8
                  local.get 0
                  i32.sub
                  i32.const 15
                  i32.and
                  i32.const 0
                  local.get 0
                  i32.const 8
                  i32.add
                  i32.const 15
                  i32.and
                  select
                  local.tee 4
                  i32.add
                  local.tee 3
                  local.get 7
                  i32.const -56
                  i32.add
                  local.tee 6
                  local.get 4
                  i32.sub
                  local.tee 4
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  i32.const 0
                  i32.const 0
                  i32.load offset=1059648
                  i32.store offset=1059188
                  i32.const 0
                  local.get 4
                  i32.store offset=1059172
                  i32.const 0
                  local.get 3
                  i32.store offset=1059184
                  local.get 0
                  local.get 6
                  i32.add
                  i32.const 56
                  i32.store offset=4
                  br 2 (;@5;)
                end
                local.get 4
                i32.load8_u offset=12
                i32.const 8
                i32.and
                br_if 0 (;@6;)
                local.get 3
                local.get 6
                i32.lt_u
                br_if 0 (;@6;)
                local.get 3
                local.get 0
                i32.ge_u
                br_if 0 (;@6;)
                local.get 3
                i32.const -8
                local.get 3
                i32.sub
                i32.const 15
                i32.and
                i32.const 0
                local.get 3
                i32.const 8
                i32.add
                i32.const 15
                i32.and
                select
                local.tee 6
                i32.add
                local.tee 0
                i32.const 0
                i32.load offset=1059172
                local.get 7
                i32.add
                local.tee 2
                local.get 6
                i32.sub
                local.tee 6
                i32.const 1
                i32.or
                i32.store offset=4
                local.get 4
                local.get 9
                local.get 7
                i32.add
                i32.store offset=4
                i32.const 0
                i32.const 0
                i32.load offset=1059648
                i32.store offset=1059188
                i32.const 0
                local.get 6
                i32.store offset=1059172
                i32.const 0
                local.get 0
                i32.store offset=1059184
                local.get 3
                local.get 2
                i32.add
                i32.const 56
                i32.store offset=4
                br 1 (;@5;)
              end
              block  ;; label = @6
                local.get 0
                i32.const 0
                i32.load offset=1059176
                local.tee 9
                i32.ge_u
                br_if 0 (;@6;)
                i32.const 0
                local.get 0
                i32.store offset=1059176
                local.get 0
                local.set 9
              end
              local.get 0
              local.get 7
              i32.add
              local.set 6
              i32.const 1059608
              local.set 4
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            loop  ;; label = @13
                              local.get 4
                              i32.load
                              local.get 6
                              i32.eq
                              br_if 1 (;@12;)
                              local.get 4
                              i32.load offset=8
                              local.tee 4
                              br_if 0 (;@13;)
                              br 2 (;@11;)
                            end
                          end
                          local.get 4
                          i32.load8_u offset=12
                          i32.const 8
                          i32.and
                          i32.eqz
                          br_if 1 (;@10;)
                        end
                        i32.const 1059608
                        local.set 4
                        loop  ;; label = @11
                          block  ;; label = @12
                            local.get 4
                            i32.load
                            local.tee 6
                            local.get 3
                            i32.gt_u
                            br_if 0 (;@12;)
                            local.get 6
                            local.get 4
                            i32.load offset=4
                            i32.add
                            local.tee 6
                            local.get 3
                            i32.gt_u
                            br_if 3 (;@9;)
                          end
                          local.get 4
                          i32.load offset=8
                          local.set 4
                          br 0 (;@11;)
                        end
                      end
                      local.get 4
                      local.get 0
                      i32.store
                      local.get 4
                      local.get 4
                      i32.load offset=4
                      local.get 7
                      i32.add
                      i32.store offset=4
                      local.get 0
                      i32.const -8
                      local.get 0
                      i32.sub
                      i32.const 15
                      i32.and
                      i32.const 0
                      local.get 0
                      i32.const 8
                      i32.add
                      i32.const 15
                      i32.and
                      select
                      i32.add
                      local.tee 2
                      local.get 5
                      i32.const 3
                      i32.or
                      i32.store offset=4
                      local.get 6
                      i32.const -8
                      local.get 6
                      i32.sub
                      i32.const 15
                      i32.and
                      i32.const 0
                      local.get 6
                      i32.const 8
                      i32.add
                      i32.const 15
                      i32.and
                      select
                      i32.add
                      local.tee 7
                      local.get 2
                      local.get 5
                      i32.add
                      local.tee 5
                      i32.sub
                      local.set 4
                      block  ;; label = @10
                        local.get 7
                        local.get 3
                        i32.ne
                        br_if 0 (;@10;)
                        i32.const 0
                        local.get 5
                        i32.store offset=1059184
                        i32.const 0
                        i32.const 0
                        i32.load offset=1059172
                        local.get 4
                        i32.add
                        local.tee 4
                        i32.store offset=1059172
                        local.get 5
                        local.get 4
                        i32.const 1
                        i32.or
                        i32.store offset=4
                        br 3 (;@7;)
                      end
                      block  ;; label = @10
                        local.get 7
                        i32.const 0
                        i32.load offset=1059180
                        i32.ne
                        br_if 0 (;@10;)
                        i32.const 0
                        local.get 5
                        i32.store offset=1059180
                        i32.const 0
                        i32.const 0
                        i32.load offset=1059168
                        local.get 4
                        i32.add
                        local.tee 4
                        i32.store offset=1059168
                        local.get 5
                        local.get 4
                        i32.const 1
                        i32.or
                        i32.store offset=4
                        local.get 5
                        local.get 4
                        i32.add
                        local.get 4
                        i32.store
                        br 3 (;@7;)
                      end
                      block  ;; label = @10
                        local.get 7
                        i32.load offset=4
                        local.tee 3
                        i32.const 3
                        i32.and
                        i32.const 1
                        i32.ne
                        br_if 0 (;@10;)
                        local.get 3
                        i32.const -8
                        i32.and
                        local.set 8
                        block  ;; label = @11
                          block  ;; label = @12
                            local.get 3
                            i32.const 255
                            i32.gt_u
                            br_if 0 (;@12;)
                            local.get 7
                            i32.load offset=8
                            local.tee 6
                            local.get 3
                            i32.const 3
                            i32.shr_u
                            local.tee 9
                            i32.const 3
                            i32.shl
                            i32.const 1059200
                            i32.add
                            local.tee 0
                            i32.eq
                            drop
                            block  ;; label = @13
                              local.get 7
                              i32.load offset=12
                              local.tee 3
                              local.get 6
                              i32.ne
                              br_if 0 (;@13;)
                              i32.const 0
                              i32.const 0
                              i32.load offset=1059160
                              i32.const -2
                              local.get 9
                              i32.rotl
                              i32.and
                              i32.store offset=1059160
                              br 2 (;@11;)
                            end
                            local.get 3
                            local.get 0
                            i32.eq
                            drop
                            local.get 3
                            local.get 6
                            i32.store offset=8
                            local.get 6
                            local.get 3
                            i32.store offset=12
                            br 1 (;@11;)
                          end
                          local.get 7
                          i32.load offset=24
                          local.set 10
                          block  ;; label = @12
                            block  ;; label = @13
                              local.get 7
                              i32.load offset=12
                              local.tee 0
                              local.get 7
                              i32.eq
                              br_if 0 (;@13;)
                              local.get 7
                              i32.load offset=8
                              local.tee 3
                              local.get 9
                              i32.lt_u
                              drop
                              local.get 0
                              local.get 3
                              i32.store offset=8
                              local.get 3
                              local.get 0
                              i32.store offset=12
                              br 1 (;@12;)
                            end
                            block  ;; label = @13
                              local.get 7
                              i32.const 20
                              i32.add
                              local.tee 3
                              i32.load
                              local.tee 6
                              br_if 0 (;@13;)
                              local.get 7
                              i32.const 16
                              i32.add
                              local.tee 3
                              i32.load
                              local.tee 6
                              br_if 0 (;@13;)
                              i32.const 0
                              local.set 0
                              br 1 (;@12;)
                            end
                            loop  ;; label = @13
                              local.get 3
                              local.set 9
                              local.get 6
                              local.tee 0
                              i32.const 20
                              i32.add
                              local.tee 3
                              i32.load
                              local.tee 6
                              br_if 0 (;@13;)
                              local.get 0
                              i32.const 16
                              i32.add
                              local.set 3
                              local.get 0
                              i32.load offset=16
                              local.tee 6
                              br_if 0 (;@13;)
                            end
                            local.get 9
                            i32.const 0
                            i32.store
                          end
                          local.get 10
                          i32.eqz
                          br_if 0 (;@11;)
                          block  ;; label = @12
                            block  ;; label = @13
                              local.get 7
                              local.get 7
                              i32.load offset=28
                              local.tee 6
                              i32.const 2
                              i32.shl
                              i32.const 1059464
                              i32.add
                              local.tee 3
                              i32.load
                              i32.ne
                              br_if 0 (;@13;)
                              local.get 3
                              local.get 0
                              i32.store
                              local.get 0
                              br_if 1 (;@12;)
                              i32.const 0
                              i32.const 0
                              i32.load offset=1059164
                              i32.const -2
                              local.get 6
                              i32.rotl
                              i32.and
                              i32.store offset=1059164
                              br 2 (;@11;)
                            end
                            local.get 10
                            i32.const 16
                            i32.const 20
                            local.get 10
                            i32.load offset=16
                            local.get 7
                            i32.eq
                            select
                            i32.add
                            local.get 0
                            i32.store
                            local.get 0
                            i32.eqz
                            br_if 1 (;@11;)
                          end
                          local.get 0
                          local.get 10
                          i32.store offset=24
                          block  ;; label = @12
                            local.get 7
                            i32.load offset=16
                            local.tee 3
                            i32.eqz
                            br_if 0 (;@12;)
                            local.get 0
                            local.get 3
                            i32.store offset=16
                            local.get 3
                            local.get 0
                            i32.store offset=24
                          end
                          local.get 7
                          i32.load offset=20
                          local.tee 3
                          i32.eqz
                          br_if 0 (;@11;)
                          local.get 0
                          i32.const 20
                          i32.add
                          local.get 3
                          i32.store
                          local.get 3
                          local.get 0
                          i32.store offset=24
                        end
                        local.get 8
                        local.get 4
                        i32.add
                        local.set 4
                        local.get 7
                        local.get 8
                        i32.add
                        local.tee 7
                        i32.load offset=4
                        local.set 3
                      end
                      local.get 7
                      local.get 3
                      i32.const -2
                      i32.and
                      i32.store offset=4
                      local.get 5
                      local.get 4
                      i32.add
                      local.get 4
                      i32.store
                      local.get 5
                      local.get 4
                      i32.const 1
                      i32.or
                      i32.store offset=4
                      block  ;; label = @10
                        local.get 4
                        i32.const 255
                        i32.gt_u
                        br_if 0 (;@10;)
                        local.get 4
                        i32.const -8
                        i32.and
                        i32.const 1059200
                        i32.add
                        local.set 3
                        block  ;; label = @11
                          block  ;; label = @12
                            i32.const 0
                            i32.load offset=1059160
                            local.tee 6
                            i32.const 1
                            local.get 4
                            i32.const 3
                            i32.shr_u
                            i32.shl
                            local.tee 4
                            i32.and
                            br_if 0 (;@12;)
                            i32.const 0
                            local.get 6
                            local.get 4
                            i32.or
                            i32.store offset=1059160
                            local.get 3
                            local.set 4
                            br 1 (;@11;)
                          end
                          local.get 3
                          i32.load offset=8
                          local.set 4
                        end
                        local.get 4
                        local.get 5
                        i32.store offset=12
                        local.get 3
                        local.get 5
                        i32.store offset=8
                        local.get 5
                        local.get 3
                        i32.store offset=12
                        local.get 5
                        local.get 4
                        i32.store offset=8
                        br 3 (;@7;)
                      end
                      i32.const 31
                      local.set 3
                      block  ;; label = @10
                        local.get 4
                        i32.const 16777215
                        i32.gt_u
                        br_if 0 (;@10;)
                        local.get 4
                        i32.const 8
                        i32.shr_u
                        local.tee 3
                        local.get 3
                        i32.const 1048320
                        i32.add
                        i32.const 16
                        i32.shr_u
                        i32.const 8
                        i32.and
                        local.tee 3
                        i32.shl
                        local.tee 6
                        local.get 6
                        i32.const 520192
                        i32.add
                        i32.const 16
                        i32.shr_u
                        i32.const 4
                        i32.and
                        local.tee 6
                        i32.shl
                        local.tee 0
                        local.get 0
                        i32.const 245760
                        i32.add
                        i32.const 16
                        i32.shr_u
                        i32.const 2
                        i32.and
                        local.tee 0
                        i32.shl
                        i32.const 15
                        i32.shr_u
                        local.get 3
                        local.get 6
                        i32.or
                        local.get 0
                        i32.or
                        i32.sub
                        local.tee 3
                        i32.const 1
                        i32.shl
                        local.get 4
                        local.get 3
                        i32.const 21
                        i32.add
                        i32.shr_u
                        i32.const 1
                        i32.and
                        i32.or
                        i32.const 28
                        i32.add
                        local.set 3
                      end
                      local.get 5
                      local.get 3
                      i32.store offset=28
                      local.get 5
                      i64.const 0
                      i64.store offset=16 align=4
                      local.get 3
                      i32.const 2
                      i32.shl
                      i32.const 1059464
                      i32.add
                      local.set 6
                      block  ;; label = @10
                        i32.const 0
                        i32.load offset=1059164
                        local.tee 0
                        i32.const 1
                        local.get 3
                        i32.shl
                        local.tee 9
                        i32.and
                        br_if 0 (;@10;)
                        local.get 6
                        local.get 5
                        i32.store
                        i32.const 0
                        local.get 0
                        local.get 9
                        i32.or
                        i32.store offset=1059164
                        local.get 5
                        local.get 6
                        i32.store offset=24
                        local.get 5
                        local.get 5
                        i32.store offset=8
                        local.get 5
                        local.get 5
                        i32.store offset=12
                        br 3 (;@7;)
                      end
                      local.get 4
                      i32.const 0
                      i32.const 25
                      local.get 3
                      i32.const 1
                      i32.shr_u
                      i32.sub
                      local.get 3
                      i32.const 31
                      i32.eq
                      select
                      i32.shl
                      local.set 3
                      local.get 6
                      i32.load
                      local.set 0
                      loop  ;; label = @10
                        local.get 0
                        local.tee 6
                        i32.load offset=4
                        i32.const -8
                        i32.and
                        local.get 4
                        i32.eq
                        br_if 2 (;@8;)
                        local.get 3
                        i32.const 29
                        i32.shr_u
                        local.set 0
                        local.get 3
                        i32.const 1
                        i32.shl
                        local.set 3
                        local.get 6
                        local.get 0
                        i32.const 4
                        i32.and
                        i32.add
                        i32.const 16
                        i32.add
                        local.tee 9
                        i32.load
                        local.tee 0
                        br_if 0 (;@10;)
                      end
                      local.get 9
                      local.get 5
                      i32.store
                      local.get 5
                      local.get 6
                      i32.store offset=24
                      local.get 5
                      local.get 5
                      i32.store offset=12
                      local.get 5
                      local.get 5
                      i32.store offset=8
                      br 2 (;@7;)
                    end
                    local.get 0
                    i32.const -8
                    local.get 0
                    i32.sub
                    i32.const 15
                    i32.and
                    i32.const 0
                    local.get 0
                    i32.const 8
                    i32.add
                    i32.const 15
                    i32.and
                    select
                    local.tee 4
                    i32.add
                    local.tee 2
                    local.get 7
                    i32.const -56
                    i32.add
                    local.tee 9
                    local.get 4
                    i32.sub
                    local.tee 4
                    i32.const 1
                    i32.or
                    i32.store offset=4
                    local.get 0
                    local.get 9
                    i32.add
                    i32.const 56
                    i32.store offset=4
                    local.get 3
                    local.get 6
                    i32.const 55
                    local.get 6
                    i32.sub
                    i32.const 15
                    i32.and
                    i32.const 0
                    local.get 6
                    i32.const -55
                    i32.add
                    i32.const 15
                    i32.and
                    select
                    i32.add
                    i32.const -63
                    i32.add
                    local.tee 9
                    local.get 9
                    local.get 3
                    i32.const 16
                    i32.add
                    i32.lt_u
                    select
                    local.tee 9
                    i32.const 35
                    i32.store offset=4
                    i32.const 0
                    i32.const 0
                    i32.load offset=1059648
                    i32.store offset=1059188
                    i32.const 0
                    local.get 4
                    i32.store offset=1059172
                    i32.const 0
                    local.get 2
                    i32.store offset=1059184
                    local.get 9
                    i32.const 16
                    i32.add
                    i32.const 0
                    i64.load offset=1059616 align=4
                    i64.store align=4
                    local.get 9
                    i32.const 0
                    i64.load offset=1059608 align=4
                    i64.store offset=8 align=4
                    i32.const 0
                    local.get 9
                    i32.const 8
                    i32.add
                    i32.store offset=1059616
                    i32.const 0
                    local.get 7
                    i32.store offset=1059612
                    i32.const 0
                    local.get 0
                    i32.store offset=1059608
                    i32.const 0
                    i32.const 0
                    i32.store offset=1059620
                    local.get 9
                    i32.const 36
                    i32.add
                    local.set 4
                    loop  ;; label = @9
                      local.get 4
                      i32.const 7
                      i32.store
                      local.get 4
                      i32.const 4
                      i32.add
                      local.tee 4
                      local.get 6
                      i32.lt_u
                      br_if 0 (;@9;)
                    end
                    local.get 9
                    local.get 3
                    i32.eq
                    br_if 3 (;@5;)
                    local.get 9
                    local.get 9
                    i32.load offset=4
                    i32.const -2
                    i32.and
                    i32.store offset=4
                    local.get 9
                    local.get 9
                    local.get 3
                    i32.sub
                    local.tee 0
                    i32.store
                    local.get 3
                    local.get 0
                    i32.const 1
                    i32.or
                    i32.store offset=4
                    block  ;; label = @9
                      local.get 0
                      i32.const 255
                      i32.gt_u
                      br_if 0 (;@9;)
                      local.get 0
                      i32.const -8
                      i32.and
                      i32.const 1059200
                      i32.add
                      local.set 4
                      block  ;; label = @10
                        block  ;; label = @11
                          i32.const 0
                          i32.load offset=1059160
                          local.tee 6
                          i32.const 1
                          local.get 0
                          i32.const 3
                          i32.shr_u
                          i32.shl
                          local.tee 0
                          i32.and
                          br_if 0 (;@11;)
                          i32.const 0
                          local.get 6
                          local.get 0
                          i32.or
                          i32.store offset=1059160
                          local.get 4
                          local.set 6
                          br 1 (;@10;)
                        end
                        local.get 4
                        i32.load offset=8
                        local.set 6
                      end
                      local.get 6
                      local.get 3
                      i32.store offset=12
                      local.get 4
                      local.get 3
                      i32.store offset=8
                      local.get 3
                      local.get 4
                      i32.store offset=12
                      local.get 3
                      local.get 6
                      i32.store offset=8
                      br 4 (;@5;)
                    end
                    i32.const 31
                    local.set 4
                    block  ;; label = @9
                      local.get 0
                      i32.const 16777215
                      i32.gt_u
                      br_if 0 (;@9;)
                      local.get 0
                      i32.const 8
                      i32.shr_u
                      local.tee 4
                      local.get 4
                      i32.const 1048320
                      i32.add
                      i32.const 16
                      i32.shr_u
                      i32.const 8
                      i32.and
                      local.tee 4
                      i32.shl
                      local.tee 6
                      local.get 6
                      i32.const 520192
                      i32.add
                      i32.const 16
                      i32.shr_u
                      i32.const 4
                      i32.and
                      local.tee 6
                      i32.shl
                      local.tee 9
                      local.get 9
                      i32.const 245760
                      i32.add
                      i32.const 16
                      i32.shr_u
                      i32.const 2
                      i32.and
                      local.tee 9
                      i32.shl
                      i32.const 15
                      i32.shr_u
                      local.get 4
                      local.get 6
                      i32.or
                      local.get 9
                      i32.or
                      i32.sub
                      local.tee 4
                      i32.const 1
                      i32.shl
                      local.get 0
                      local.get 4
                      i32.const 21
                      i32.add
                      i32.shr_u
                      i32.const 1
                      i32.and
                      i32.or
                      i32.const 28
                      i32.add
                      local.set 4
                    end
                    local.get 3
                    local.get 4
                    i32.store offset=28
                    local.get 3
                    i64.const 0
                    i64.store offset=16 align=4
                    local.get 4
                    i32.const 2
                    i32.shl
                    i32.const 1059464
                    i32.add
                    local.set 6
                    block  ;; label = @9
                      i32.const 0
                      i32.load offset=1059164
                      local.tee 9
                      i32.const 1
                      local.get 4
                      i32.shl
                      local.tee 7
                      i32.and
                      br_if 0 (;@9;)
                      local.get 6
                      local.get 3
                      i32.store
                      i32.const 0
                      local.get 9
                      local.get 7
                      i32.or
                      i32.store offset=1059164
                      local.get 3
                      local.get 6
                      i32.store offset=24
                      local.get 3
                      local.get 3
                      i32.store offset=8
                      local.get 3
                      local.get 3
                      i32.store offset=12
                      br 4 (;@5;)
                    end
                    local.get 0
                    i32.const 0
                    i32.const 25
                    local.get 4
                    i32.const 1
                    i32.shr_u
                    i32.sub
                    local.get 4
                    i32.const 31
                    i32.eq
                    select
                    i32.shl
                    local.set 4
                    local.get 6
                    i32.load
                    local.set 9
                    loop  ;; label = @9
                      local.get 9
                      local.tee 6
                      i32.load offset=4
                      i32.const -8
                      i32.and
                      local.get 0
                      i32.eq
                      br_if 3 (;@6;)
                      local.get 4
                      i32.const 29
                      i32.shr_u
                      local.set 9
                      local.get 4
                      i32.const 1
                      i32.shl
                      local.set 4
                      local.get 6
                      local.get 9
                      i32.const 4
                      i32.and
                      i32.add
                      i32.const 16
                      i32.add
                      local.tee 7
                      i32.load
                      local.tee 9
                      br_if 0 (;@9;)
                    end
                    local.get 7
                    local.get 3
                    i32.store
                    local.get 3
                    local.get 6
                    i32.store offset=24
                    local.get 3
                    local.get 3
                    i32.store offset=12
                    local.get 3
                    local.get 3
                    i32.store offset=8
                    br 3 (;@5;)
                  end
                  local.get 6
                  i32.load offset=8
                  local.tee 4
                  local.get 5
                  i32.store offset=12
                  local.get 6
                  local.get 5
                  i32.store offset=8
                  local.get 5
                  i32.const 0
                  i32.store offset=24
                  local.get 5
                  local.get 6
                  i32.store offset=12
                  local.get 5
                  local.get 4
                  i32.store offset=8
                end
                local.get 2
                i32.const 8
                i32.add
                local.set 4
                br 5 (;@1;)
              end
              local.get 6
              i32.load offset=8
              local.tee 4
              local.get 3
              i32.store offset=12
              local.get 6
              local.get 3
              i32.store offset=8
              local.get 3
              i32.const 0
              i32.store offset=24
              local.get 3
              local.get 6
              i32.store offset=12
              local.get 3
              local.get 4
              i32.store offset=8
            end
            i32.const 0
            i32.load offset=1059172
            local.tee 4
            local.get 5
            i32.le_u
            br_if 0 (;@4;)
            i32.const 0
            i32.load offset=1059184
            local.tee 3
            local.get 5
            i32.add
            local.tee 6
            local.get 4
            local.get 5
            i32.sub
            local.tee 4
            i32.const 1
            i32.or
            i32.store offset=4
            i32.const 0
            local.get 4
            i32.store offset=1059172
            i32.const 0
            local.get 6
            i32.store offset=1059184
            local.get 3
            local.get 5
            i32.const 3
            i32.or
            i32.store offset=4
            local.get 3
            i32.const 8
            i32.add
            local.set 4
            br 3 (;@1;)
          end
          i32.const 0
          local.set 4
          i32.const 0
          i32.const 48
          i32.store offset=1059656
          br 2 (;@1;)
        end
        block  ;; label = @3
          local.get 2
          i32.eqz
          br_if 0 (;@3;)
          block  ;; label = @4
            block  ;; label = @5
              local.get 9
              local.get 9
              i32.load offset=28
              local.tee 6
              i32.const 2
              i32.shl
              i32.const 1059464
              i32.add
              local.tee 4
              i32.load
              i32.ne
              br_if 0 (;@5;)
              local.get 4
              local.get 0
              i32.store
              local.get 0
              br_if 1 (;@4;)
              i32.const 0
              local.get 10
              i32.const -2
              local.get 6
              i32.rotl
              i32.and
              local.tee 10
              i32.store offset=1059164
              br 2 (;@3;)
            end
            local.get 2
            i32.const 16
            i32.const 20
            local.get 2
            i32.load offset=16
            local.get 9
            i32.eq
            select
            i32.add
            local.get 0
            i32.store
            local.get 0
            i32.eqz
            br_if 1 (;@3;)
          end
          local.get 0
          local.get 2
          i32.store offset=24
          block  ;; label = @4
            local.get 9
            i32.load offset=16
            local.tee 4
            i32.eqz
            br_if 0 (;@4;)
            local.get 0
            local.get 4
            i32.store offset=16
            local.get 4
            local.get 0
            i32.store offset=24
          end
          local.get 9
          i32.const 20
          i32.add
          i32.load
          local.tee 4
          i32.eqz
          br_if 0 (;@3;)
          local.get 0
          i32.const 20
          i32.add
          local.get 4
          i32.store
          local.get 4
          local.get 0
          i32.store offset=24
        end
        block  ;; label = @3
          block  ;; label = @4
            local.get 3
            i32.const 15
            i32.gt_u
            br_if 0 (;@4;)
            local.get 9
            local.get 3
            local.get 5
            i32.add
            local.tee 4
            i32.const 3
            i32.or
            i32.store offset=4
            local.get 9
            local.get 4
            i32.add
            local.tee 4
            local.get 4
            i32.load offset=4
            i32.const 1
            i32.or
            i32.store offset=4
            br 1 (;@3;)
          end
          local.get 9
          local.get 5
          i32.add
          local.tee 0
          local.get 3
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 9
          local.get 5
          i32.const 3
          i32.or
          i32.store offset=4
          local.get 0
          local.get 3
          i32.add
          local.get 3
          i32.store
          block  ;; label = @4
            local.get 3
            i32.const 255
            i32.gt_u
            br_if 0 (;@4;)
            local.get 3
            i32.const -8
            i32.and
            i32.const 1059200
            i32.add
            local.set 4
            block  ;; label = @5
              block  ;; label = @6
                i32.const 0
                i32.load offset=1059160
                local.tee 6
                i32.const 1
                local.get 3
                i32.const 3
                i32.shr_u
                i32.shl
                local.tee 3
                i32.and
                br_if 0 (;@6;)
                i32.const 0
                local.get 6
                local.get 3
                i32.or
                i32.store offset=1059160
                local.get 4
                local.set 3
                br 1 (;@5;)
              end
              local.get 4
              i32.load offset=8
              local.set 3
            end
            local.get 3
            local.get 0
            i32.store offset=12
            local.get 4
            local.get 0
            i32.store offset=8
            local.get 0
            local.get 4
            i32.store offset=12
            local.get 0
            local.get 3
            i32.store offset=8
            br 1 (;@3;)
          end
          i32.const 31
          local.set 4
          block  ;; label = @4
            local.get 3
            i32.const 16777215
            i32.gt_u
            br_if 0 (;@4;)
            local.get 3
            i32.const 8
            i32.shr_u
            local.tee 4
            local.get 4
            i32.const 1048320
            i32.add
            i32.const 16
            i32.shr_u
            i32.const 8
            i32.and
            local.tee 4
            i32.shl
            local.tee 6
            local.get 6
            i32.const 520192
            i32.add
            i32.const 16
            i32.shr_u
            i32.const 4
            i32.and
            local.tee 6
            i32.shl
            local.tee 5
            local.get 5
            i32.const 245760
            i32.add
            i32.const 16
            i32.shr_u
            i32.const 2
            i32.and
            local.tee 5
            i32.shl
            i32.const 15
            i32.shr_u
            local.get 4
            local.get 6
            i32.or
            local.get 5
            i32.or
            i32.sub
            local.tee 4
            i32.const 1
            i32.shl
            local.get 3
            local.get 4
            i32.const 21
            i32.add
            i32.shr_u
            i32.const 1
            i32.and
            i32.or
            i32.const 28
            i32.add
            local.set 4
          end
          local.get 0
          local.get 4
          i32.store offset=28
          local.get 0
          i64.const 0
          i64.store offset=16 align=4
          local.get 4
          i32.const 2
          i32.shl
          i32.const 1059464
          i32.add
          local.set 6
          block  ;; label = @4
            local.get 10
            i32.const 1
            local.get 4
            i32.shl
            local.tee 5
            i32.and
            br_if 0 (;@4;)
            local.get 6
            local.get 0
            i32.store
            i32.const 0
            local.get 10
            local.get 5
            i32.or
            i32.store offset=1059164
            local.get 0
            local.get 6
            i32.store offset=24
            local.get 0
            local.get 0
            i32.store offset=8
            local.get 0
            local.get 0
            i32.store offset=12
            br 1 (;@3;)
          end
          local.get 3
          i32.const 0
          i32.const 25
          local.get 4
          i32.const 1
          i32.shr_u
          i32.sub
          local.get 4
          i32.const 31
          i32.eq
          select
          i32.shl
          local.set 4
          local.get 6
          i32.load
          local.set 5
          block  ;; label = @4
            loop  ;; label = @5
              local.get 5
              local.tee 6
              i32.load offset=4
              i32.const -8
              i32.and
              local.get 3
              i32.eq
              br_if 1 (;@4;)
              local.get 4
              i32.const 29
              i32.shr_u
              local.set 5
              local.get 4
              i32.const 1
              i32.shl
              local.set 4
              local.get 6
              local.get 5
              i32.const 4
              i32.and
              i32.add
              i32.const 16
              i32.add
              local.tee 7
              i32.load
              local.tee 5
              br_if 0 (;@5;)
            end
            local.get 7
            local.get 0
            i32.store
            local.get 0
            local.get 6
            i32.store offset=24
            local.get 0
            local.get 0
            i32.store offset=12
            local.get 0
            local.get 0
            i32.store offset=8
            br 1 (;@3;)
          end
          local.get 6
          i32.load offset=8
          local.tee 4
          local.get 0
          i32.store offset=12
          local.get 6
          local.get 0
          i32.store offset=8
          local.get 0
          i32.const 0
          i32.store offset=24
          local.get 0
          local.get 6
          i32.store offset=12
          local.get 0
          local.get 4
          i32.store offset=8
        end
        local.get 9
        i32.const 8
        i32.add
        local.set 4
        br 1 (;@1;)
      end
      block  ;; label = @2
        local.get 11
        i32.eqz
        br_if 0 (;@2;)
        block  ;; label = @3
          block  ;; label = @4
            local.get 0
            local.get 0
            i32.load offset=28
            local.tee 6
            i32.const 2
            i32.shl
            i32.const 1059464
            i32.add
            local.tee 4
            i32.load
            i32.ne
            br_if 0 (;@4;)
            local.get 4
            local.get 9
            i32.store
            local.get 9
            br_if 1 (;@3;)
            i32.const 0
            local.get 10
            i32.const -2
            local.get 6
            i32.rotl
            i32.and
            i32.store offset=1059164
            br 2 (;@2;)
          end
          local.get 11
          i32.const 16
          i32.const 20
          local.get 11
          i32.load offset=16
          local.get 0
          i32.eq
          select
          i32.add
          local.get 9
          i32.store
          local.get 9
          i32.eqz
          br_if 1 (;@2;)
        end
        local.get 9
        local.get 11
        i32.store offset=24
        block  ;; label = @3
          local.get 0
          i32.load offset=16
          local.tee 4
          i32.eqz
          br_if 0 (;@3;)
          local.get 9
          local.get 4
          i32.store offset=16
          local.get 4
          local.get 9
          i32.store offset=24
        end
        local.get 0
        i32.const 20
        i32.add
        i32.load
        local.tee 4
        i32.eqz
        br_if 0 (;@2;)
        local.get 9
        i32.const 20
        i32.add
        local.get 4
        i32.store
        local.get 4
        local.get 9
        i32.store offset=24
      end
      block  ;; label = @2
        block  ;; label = @3
          local.get 3
          i32.const 15
          i32.gt_u
          br_if 0 (;@3;)
          local.get 0
          local.get 3
          local.get 5
          i32.add
          local.tee 4
          i32.const 3
          i32.or
          i32.store offset=4
          local.get 0
          local.get 4
          i32.add
          local.tee 4
          local.get 4
          i32.load offset=4
          i32.const 1
          i32.or
          i32.store offset=4
          br 1 (;@2;)
        end
        local.get 0
        local.get 5
        i32.add
        local.tee 6
        local.get 3
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 0
        local.get 5
        i32.const 3
        i32.or
        i32.store offset=4
        local.get 6
        local.get 3
        i32.add
        local.get 3
        i32.store
        block  ;; label = @3
          local.get 8
          i32.eqz
          br_if 0 (;@3;)
          local.get 8
          i32.const -8
          i32.and
          i32.const 1059200
          i32.add
          local.set 5
          i32.const 0
          i32.load offset=1059180
          local.set 4
          block  ;; label = @4
            block  ;; label = @5
              i32.const 1
              local.get 8
              i32.const 3
              i32.shr_u
              i32.shl
              local.tee 9
              local.get 7
              i32.and
              br_if 0 (;@5;)
              i32.const 0
              local.get 9
              local.get 7
              i32.or
              i32.store offset=1059160
              local.get 5
              local.set 9
              br 1 (;@4;)
            end
            local.get 5
            i32.load offset=8
            local.set 9
          end
          local.get 9
          local.get 4
          i32.store offset=12
          local.get 5
          local.get 4
          i32.store offset=8
          local.get 4
          local.get 5
          i32.store offset=12
          local.get 4
          local.get 9
          i32.store offset=8
        end
        i32.const 0
        local.get 6
        i32.store offset=1059180
        i32.const 0
        local.get 3
        i32.store offset=1059168
      end
      local.get 0
      i32.const 8
      i32.add
      local.set 4
    end
    local.get 1
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 4)
  (func $free (type 1) (param i32)
    local.get 0
    call $dlfree)
  (func $dlfree (type 1) (param i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    block  ;; label = @1
      local.get 0
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.const -8
      i32.add
      local.tee 1
      local.get 0
      i32.const -4
      i32.add
      i32.load
      local.tee 2
      i32.const -8
      i32.and
      local.tee 0
      i32.add
      local.set 3
      block  ;; label = @2
        local.get 2
        i32.const 1
        i32.and
        br_if 0 (;@2;)
        local.get 2
        i32.const 3
        i32.and
        i32.eqz
        br_if 1 (;@1;)
        local.get 1
        local.get 1
        i32.load
        local.tee 2
        i32.sub
        local.tee 1
        i32.const 0
        i32.load offset=1059176
        local.tee 4
        i32.lt_u
        br_if 1 (;@1;)
        local.get 2
        local.get 0
        i32.add
        local.set 0
        block  ;; label = @3
          local.get 1
          i32.const 0
          i32.load offset=1059180
          i32.eq
          br_if 0 (;@3;)
          block  ;; label = @4
            local.get 2
            i32.const 255
            i32.gt_u
            br_if 0 (;@4;)
            local.get 1
            i32.load offset=8
            local.tee 4
            local.get 2
            i32.const 3
            i32.shr_u
            local.tee 5
            i32.const 3
            i32.shl
            i32.const 1059200
            i32.add
            local.tee 6
            i32.eq
            drop
            block  ;; label = @5
              local.get 1
              i32.load offset=12
              local.tee 2
              local.get 4
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1059160
              i32.const -2
              local.get 5
              i32.rotl
              i32.and
              i32.store offset=1059160
              br 3 (;@2;)
            end
            local.get 2
            local.get 6
            i32.eq
            drop
            local.get 2
            local.get 4
            i32.store offset=8
            local.get 4
            local.get 2
            i32.store offset=12
            br 2 (;@2;)
          end
          local.get 1
          i32.load offset=24
          local.set 7
          block  ;; label = @4
            block  ;; label = @5
              local.get 1
              i32.load offset=12
              local.tee 6
              local.get 1
              i32.eq
              br_if 0 (;@5;)
              local.get 1
              i32.load offset=8
              local.tee 2
              local.get 4
              i32.lt_u
              drop
              local.get 6
              local.get 2
              i32.store offset=8
              local.get 2
              local.get 6
              i32.store offset=12
              br 1 (;@4;)
            end
            block  ;; label = @5
              local.get 1
              i32.const 20
              i32.add
              local.tee 2
              i32.load
              local.tee 4
              br_if 0 (;@5;)
              local.get 1
              i32.const 16
              i32.add
              local.tee 2
              i32.load
              local.tee 4
              br_if 0 (;@5;)
              i32.const 0
              local.set 6
              br 1 (;@4;)
            end
            loop  ;; label = @5
              local.get 2
              local.set 5
              local.get 4
              local.tee 6
              i32.const 20
              i32.add
              local.tee 2
              i32.load
              local.tee 4
              br_if 0 (;@5;)
              local.get 6
              i32.const 16
              i32.add
              local.set 2
              local.get 6
              i32.load offset=16
              local.tee 4
              br_if 0 (;@5;)
            end
            local.get 5
            i32.const 0
            i32.store
          end
          local.get 7
          i32.eqz
          br_if 1 (;@2;)
          block  ;; label = @4
            block  ;; label = @5
              local.get 1
              local.get 1
              i32.load offset=28
              local.tee 4
              i32.const 2
              i32.shl
              i32.const 1059464
              i32.add
              local.tee 2
              i32.load
              i32.ne
              br_if 0 (;@5;)
              local.get 2
              local.get 6
              i32.store
              local.get 6
              br_if 1 (;@4;)
              i32.const 0
              i32.const 0
              i32.load offset=1059164
              i32.const -2
              local.get 4
              i32.rotl
              i32.and
              i32.store offset=1059164
              br 3 (;@2;)
            end
            local.get 7
            i32.const 16
            i32.const 20
            local.get 7
            i32.load offset=16
            local.get 1
            i32.eq
            select
            i32.add
            local.get 6
            i32.store
            local.get 6
            i32.eqz
            br_if 2 (;@2;)
          end
          local.get 6
          local.get 7
          i32.store offset=24
          block  ;; label = @4
            local.get 1
            i32.load offset=16
            local.tee 2
            i32.eqz
            br_if 0 (;@4;)
            local.get 6
            local.get 2
            i32.store offset=16
            local.get 2
            local.get 6
            i32.store offset=24
          end
          local.get 1
          i32.load offset=20
          local.tee 2
          i32.eqz
          br_if 1 (;@2;)
          local.get 6
          i32.const 20
          i32.add
          local.get 2
          i32.store
          local.get 2
          local.get 6
          i32.store offset=24
          br 1 (;@2;)
        end
        local.get 3
        i32.load offset=4
        local.tee 2
        i32.const 3
        i32.and
        i32.const 3
        i32.ne
        br_if 0 (;@2;)
        local.get 3
        local.get 2
        i32.const -2
        i32.and
        i32.store offset=4
        i32.const 0
        local.get 0
        i32.store offset=1059168
        local.get 1
        local.get 0
        i32.add
        local.get 0
        i32.store
        local.get 1
        local.get 0
        i32.const 1
        i32.or
        i32.store offset=4
        return
      end
      local.get 1
      local.get 3
      i32.ge_u
      br_if 0 (;@1;)
      local.get 3
      i32.load offset=4
      local.tee 2
      i32.const 1
      i32.and
      i32.eqz
      br_if 0 (;@1;)
      block  ;; label = @2
        block  ;; label = @3
          local.get 2
          i32.const 2
          i32.and
          br_if 0 (;@3;)
          block  ;; label = @4
            local.get 3
            i32.const 0
            i32.load offset=1059184
            i32.ne
            br_if 0 (;@4;)
            i32.const 0
            local.get 1
            i32.store offset=1059184
            i32.const 0
            i32.const 0
            i32.load offset=1059172
            local.get 0
            i32.add
            local.tee 0
            i32.store offset=1059172
            local.get 1
            local.get 0
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 1
            i32.const 0
            i32.load offset=1059180
            i32.ne
            br_if 3 (;@1;)
            i32.const 0
            i32.const 0
            i32.store offset=1059168
            i32.const 0
            i32.const 0
            i32.store offset=1059180
            return
          end
          block  ;; label = @4
            local.get 3
            i32.const 0
            i32.load offset=1059180
            i32.ne
            br_if 0 (;@4;)
            i32.const 0
            local.get 1
            i32.store offset=1059180
            i32.const 0
            i32.const 0
            i32.load offset=1059168
            local.get 0
            i32.add
            local.tee 0
            i32.store offset=1059168
            local.get 1
            local.get 0
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 1
            local.get 0
            i32.add
            local.get 0
            i32.store
            return
          end
          local.get 2
          i32.const -8
          i32.and
          local.get 0
          i32.add
          local.set 0
          block  ;; label = @4
            block  ;; label = @5
              local.get 2
              i32.const 255
              i32.gt_u
              br_if 0 (;@5;)
              local.get 3
              i32.load offset=8
              local.tee 4
              local.get 2
              i32.const 3
              i32.shr_u
              local.tee 5
              i32.const 3
              i32.shl
              i32.const 1059200
              i32.add
              local.tee 6
              i32.eq
              drop
              block  ;; label = @6
                local.get 3
                i32.load offset=12
                local.tee 2
                local.get 4
                i32.ne
                br_if 0 (;@6;)
                i32.const 0
                i32.const 0
                i32.load offset=1059160
                i32.const -2
                local.get 5
                i32.rotl
                i32.and
                i32.store offset=1059160
                br 2 (;@4;)
              end
              local.get 2
              local.get 6
              i32.eq
              drop
              local.get 2
              local.get 4
              i32.store offset=8
              local.get 4
              local.get 2
              i32.store offset=12
              br 1 (;@4;)
            end
            local.get 3
            i32.load offset=24
            local.set 7
            block  ;; label = @5
              block  ;; label = @6
                local.get 3
                i32.load offset=12
                local.tee 6
                local.get 3
                i32.eq
                br_if 0 (;@6;)
                local.get 3
                i32.load offset=8
                local.tee 2
                i32.const 0
                i32.load offset=1059176
                i32.lt_u
                drop
                local.get 6
                local.get 2
                i32.store offset=8
                local.get 2
                local.get 6
                i32.store offset=12
                br 1 (;@5;)
              end
              block  ;; label = @6
                local.get 3
                i32.const 20
                i32.add
                local.tee 2
                i32.load
                local.tee 4
                br_if 0 (;@6;)
                local.get 3
                i32.const 16
                i32.add
                local.tee 2
                i32.load
                local.tee 4
                br_if 0 (;@6;)
                i32.const 0
                local.set 6
                br 1 (;@5;)
              end
              loop  ;; label = @6
                local.get 2
                local.set 5
                local.get 4
                local.tee 6
                i32.const 20
                i32.add
                local.tee 2
                i32.load
                local.tee 4
                br_if 0 (;@6;)
                local.get 6
                i32.const 16
                i32.add
                local.set 2
                local.get 6
                i32.load offset=16
                local.tee 4
                br_if 0 (;@6;)
              end
              local.get 5
              i32.const 0
              i32.store
            end
            local.get 7
            i32.eqz
            br_if 0 (;@4;)
            block  ;; label = @5
              block  ;; label = @6
                local.get 3
                local.get 3
                i32.load offset=28
                local.tee 4
                i32.const 2
                i32.shl
                i32.const 1059464
                i32.add
                local.tee 2
                i32.load
                i32.ne
                br_if 0 (;@6;)
                local.get 2
                local.get 6
                i32.store
                local.get 6
                br_if 1 (;@5;)
                i32.const 0
                i32.const 0
                i32.load offset=1059164
                i32.const -2
                local.get 4
                i32.rotl
                i32.and
                i32.store offset=1059164
                br 2 (;@4;)
              end
              local.get 7
              i32.const 16
              i32.const 20
              local.get 7
              i32.load offset=16
              local.get 3
              i32.eq
              select
              i32.add
              local.get 6
              i32.store
              local.get 6
              i32.eqz
              br_if 1 (;@4;)
            end
            local.get 6
            local.get 7
            i32.store offset=24
            block  ;; label = @5
              local.get 3
              i32.load offset=16
              local.tee 2
              i32.eqz
              br_if 0 (;@5;)
              local.get 6
              local.get 2
              i32.store offset=16
              local.get 2
              local.get 6
              i32.store offset=24
            end
            local.get 3
            i32.load offset=20
            local.tee 2
            i32.eqz
            br_if 0 (;@4;)
            local.get 6
            i32.const 20
            i32.add
            local.get 2
            i32.store
            local.get 2
            local.get 6
            i32.store offset=24
          end
          local.get 1
          local.get 0
          i32.add
          local.get 0
          i32.store
          local.get 1
          local.get 0
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 1
          i32.const 0
          i32.load offset=1059180
          i32.ne
          br_if 1 (;@2;)
          i32.const 0
          local.get 0
          i32.store offset=1059168
          return
        end
        local.get 3
        local.get 2
        i32.const -2
        i32.and
        i32.store offset=4
        local.get 1
        local.get 0
        i32.add
        local.get 0
        i32.store
        local.get 1
        local.get 0
        i32.const 1
        i32.or
        i32.store offset=4
      end
      block  ;; label = @2
        local.get 0
        i32.const 255
        i32.gt_u
        br_if 0 (;@2;)
        local.get 0
        i32.const -8
        i32.and
        i32.const 1059200
        i32.add
        local.set 2
        block  ;; label = @3
          block  ;; label = @4
            i32.const 0
            i32.load offset=1059160
            local.tee 4
            i32.const 1
            local.get 0
            i32.const 3
            i32.shr_u
            i32.shl
            local.tee 0
            i32.and
            br_if 0 (;@4;)
            i32.const 0
            local.get 4
            local.get 0
            i32.or
            i32.store offset=1059160
            local.get 2
            local.set 0
            br 1 (;@3;)
          end
          local.get 2
          i32.load offset=8
          local.set 0
        end
        local.get 0
        local.get 1
        i32.store offset=12
        local.get 2
        local.get 1
        i32.store offset=8
        local.get 1
        local.get 2
        i32.store offset=12
        local.get 1
        local.get 0
        i32.store offset=8
        return
      end
      i32.const 31
      local.set 2
      block  ;; label = @2
        local.get 0
        i32.const 16777215
        i32.gt_u
        br_if 0 (;@2;)
        local.get 0
        i32.const 8
        i32.shr_u
        local.tee 2
        local.get 2
        i32.const 1048320
        i32.add
        i32.const 16
        i32.shr_u
        i32.const 8
        i32.and
        local.tee 2
        i32.shl
        local.tee 4
        local.get 4
        i32.const 520192
        i32.add
        i32.const 16
        i32.shr_u
        i32.const 4
        i32.and
        local.tee 4
        i32.shl
        local.tee 6
        local.get 6
        i32.const 245760
        i32.add
        i32.const 16
        i32.shr_u
        i32.const 2
        i32.and
        local.tee 6
        i32.shl
        i32.const 15
        i32.shr_u
        local.get 2
        local.get 4
        i32.or
        local.get 6
        i32.or
        i32.sub
        local.tee 2
        i32.const 1
        i32.shl
        local.get 0
        local.get 2
        i32.const 21
        i32.add
        i32.shr_u
        i32.const 1
        i32.and
        i32.or
        i32.const 28
        i32.add
        local.set 2
      end
      local.get 1
      local.get 2
      i32.store offset=28
      local.get 1
      i64.const 0
      i64.store offset=16 align=4
      local.get 2
      i32.const 2
      i32.shl
      i32.const 1059464
      i32.add
      local.set 4
      block  ;; label = @2
        block  ;; label = @3
          i32.const 0
          i32.load offset=1059164
          local.tee 6
          i32.const 1
          local.get 2
          i32.shl
          local.tee 3
          i32.and
          br_if 0 (;@3;)
          local.get 4
          local.get 1
          i32.store
          i32.const 0
          local.get 6
          local.get 3
          i32.or
          i32.store offset=1059164
          local.get 1
          local.get 4
          i32.store offset=24
          local.get 1
          local.get 1
          i32.store offset=8
          local.get 1
          local.get 1
          i32.store offset=12
          br 1 (;@2;)
        end
        local.get 0
        i32.const 0
        i32.const 25
        local.get 2
        i32.const 1
        i32.shr_u
        i32.sub
        local.get 2
        i32.const 31
        i32.eq
        select
        i32.shl
        local.set 2
        local.get 4
        i32.load
        local.set 6
        block  ;; label = @3
          loop  ;; label = @4
            local.get 6
            local.tee 4
            i32.load offset=4
            i32.const -8
            i32.and
            local.get 0
            i32.eq
            br_if 1 (;@3;)
            local.get 2
            i32.const 29
            i32.shr_u
            local.set 6
            local.get 2
            i32.const 1
            i32.shl
            local.set 2
            local.get 4
            local.get 6
            i32.const 4
            i32.and
            i32.add
            i32.const 16
            i32.add
            local.tee 3
            i32.load
            local.tee 6
            br_if 0 (;@4;)
          end
          local.get 3
          local.get 1
          i32.store
          local.get 1
          local.get 4
          i32.store offset=24
          local.get 1
          local.get 1
          i32.store offset=12
          local.get 1
          local.get 1
          i32.store offset=8
          br 1 (;@2;)
        end
        local.get 4
        i32.load offset=8
        local.tee 0
        local.get 1
        i32.store offset=12
        local.get 4
        local.get 1
        i32.store offset=8
        local.get 1
        i32.const 0
        i32.store offset=24
        local.get 1
        local.get 4
        i32.store offset=12
        local.get 1
        local.get 0
        i32.store offset=8
      end
      i32.const 0
      i32.const 0
      i32.load offset=1059192
      i32.const -1
      i32.add
      local.tee 1
      i32.const -1
      local.get 1
      select
      i32.store offset=1059192
    end)
  (func $calloc (type 2) (param i32 i32) (result i32)
    (local i32 i64)
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        br_if 0 (;@2;)
        i32.const 0
        local.set 2
        br 1 (;@1;)
      end
      local.get 0
      i64.extend_i32_u
      local.get 1
      i64.extend_i32_u
      i64.mul
      local.tee 3
      i32.wrap_i64
      local.set 2
      local.get 1
      local.get 0
      i32.or
      i32.const 65536
      i32.lt_u
      br_if 0 (;@1;)
      i32.const -1
      local.get 2
      local.get 3
      i64.const 32
      i64.shr_u
      i32.wrap_i64
      i32.const 0
      i32.ne
      select
      local.set 2
    end
    block  ;; label = @1
      local.get 2
      call $dlmalloc
      local.tee 0
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.const -4
      i32.add
      i32.load8_u
      i32.const 3
      i32.and
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.const 0
      local.get 2
      call $memset
      drop
    end
    local.get 0)
  (func $realloc (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    block  ;; label = @1
      local.get 0
      br_if 0 (;@1;)
      local.get 1
      call $dlmalloc
      return
    end
    block  ;; label = @1
      local.get 1
      i32.const -64
      i32.lt_u
      br_if 0 (;@1;)
      i32.const 0
      i32.const 48
      i32.store offset=1059656
      i32.const 0
      return
    end
    i32.const 16
    local.get 1
    i32.const 19
    i32.add
    i32.const -16
    i32.and
    local.get 1
    i32.const 11
    i32.lt_u
    select
    local.set 2
    local.get 0
    i32.const -4
    i32.add
    local.tee 3
    i32.load
    local.tee 4
    i32.const -8
    i32.and
    local.set 5
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 4
          i32.const 3
          i32.and
          br_if 0 (;@3;)
          local.get 2
          i32.const 256
          i32.lt_u
          br_if 1 (;@2;)
          local.get 5
          local.get 2
          i32.const 4
          i32.or
          i32.lt_u
          br_if 1 (;@2;)
          local.get 5
          local.get 2
          i32.sub
          i32.const 0
          i32.load offset=1059640
          i32.const 1
          i32.shl
          i32.le_u
          br_if 2 (;@1;)
          br 1 (;@2;)
        end
        local.get 0
        i32.const -8
        i32.add
        local.tee 6
        local.get 5
        i32.add
        local.set 7
        block  ;; label = @3
          local.get 5
          local.get 2
          i32.lt_u
          br_if 0 (;@3;)
          local.get 5
          local.get 2
          i32.sub
          local.tee 1
          i32.const 16
          i32.lt_u
          br_if 2 (;@1;)
          local.get 3
          local.get 2
          local.get 4
          i32.const 1
          i32.and
          i32.or
          i32.const 2
          i32.or
          i32.store
          local.get 6
          local.get 2
          i32.add
          local.tee 2
          local.get 1
          i32.const 3
          i32.or
          i32.store offset=4
          local.get 7
          local.get 7
          i32.load offset=4
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 2
          local.get 1
          call $dispose_chunk
          local.get 0
          return
        end
        block  ;; label = @3
          local.get 7
          i32.const 0
          i32.load offset=1059184
          i32.ne
          br_if 0 (;@3;)
          i32.const 0
          i32.load offset=1059172
          local.get 5
          i32.add
          local.tee 5
          local.get 2
          i32.le_u
          br_if 1 (;@2;)
          local.get 3
          local.get 2
          local.get 4
          i32.const 1
          i32.and
          i32.or
          i32.const 2
          i32.or
          i32.store
          i32.const 0
          local.get 6
          local.get 2
          i32.add
          local.tee 1
          i32.store offset=1059184
          i32.const 0
          local.get 5
          local.get 2
          i32.sub
          local.tee 2
          i32.store offset=1059172
          local.get 1
          local.get 2
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 0
          return
        end
        block  ;; label = @3
          local.get 7
          i32.const 0
          i32.load offset=1059180
          i32.ne
          br_if 0 (;@3;)
          i32.const 0
          i32.load offset=1059168
          local.get 5
          i32.add
          local.tee 5
          local.get 2
          i32.lt_u
          br_if 1 (;@2;)
          block  ;; label = @4
            block  ;; label = @5
              local.get 5
              local.get 2
              i32.sub
              local.tee 1
              i32.const 16
              i32.lt_u
              br_if 0 (;@5;)
              local.get 3
              local.get 2
              local.get 4
              i32.const 1
              i32.and
              i32.or
              i32.const 2
              i32.or
              i32.store
              local.get 6
              local.get 2
              i32.add
              local.tee 2
              local.get 1
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 6
              local.get 5
              i32.add
              local.tee 5
              local.get 1
              i32.store
              local.get 5
              local.get 5
              i32.load offset=4
              i32.const -2
              i32.and
              i32.store offset=4
              br 1 (;@4;)
            end
            local.get 3
            local.get 4
            i32.const 1
            i32.and
            local.get 5
            i32.or
            i32.const 2
            i32.or
            i32.store
            local.get 6
            local.get 5
            i32.add
            local.tee 1
            local.get 1
            i32.load offset=4
            i32.const 1
            i32.or
            i32.store offset=4
            i32.const 0
            local.set 1
            i32.const 0
            local.set 2
          end
          i32.const 0
          local.get 2
          i32.store offset=1059180
          i32.const 0
          local.get 1
          i32.store offset=1059168
          local.get 0
          return
        end
        local.get 7
        i32.load offset=4
        local.tee 8
        i32.const 2
        i32.and
        br_if 0 (;@2;)
        local.get 8
        i32.const -8
        i32.and
        local.get 5
        i32.add
        local.tee 9
        local.get 2
        i32.lt_u
        br_if 0 (;@2;)
        local.get 9
        local.get 2
        i32.sub
        local.set 10
        block  ;; label = @3
          block  ;; label = @4
            local.get 8
            i32.const 255
            i32.gt_u
            br_if 0 (;@4;)
            local.get 7
            i32.load offset=8
            local.tee 1
            local.get 8
            i32.const 3
            i32.shr_u
            local.tee 11
            i32.const 3
            i32.shl
            i32.const 1059200
            i32.add
            local.tee 8
            i32.eq
            drop
            block  ;; label = @5
              local.get 7
              i32.load offset=12
              local.tee 5
              local.get 1
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1059160
              i32.const -2
              local.get 11
              i32.rotl
              i32.and
              i32.store offset=1059160
              br 2 (;@3;)
            end
            local.get 5
            local.get 8
            i32.eq
            drop
            local.get 5
            local.get 1
            i32.store offset=8
            local.get 1
            local.get 5
            i32.store offset=12
            br 1 (;@3;)
          end
          local.get 7
          i32.load offset=24
          local.set 12
          block  ;; label = @4
            block  ;; label = @5
              local.get 7
              i32.load offset=12
              local.tee 8
              local.get 7
              i32.eq
              br_if 0 (;@5;)
              local.get 7
              i32.load offset=8
              local.tee 1
              i32.const 0
              i32.load offset=1059176
              i32.lt_u
              drop
              local.get 8
              local.get 1
              i32.store offset=8
              local.get 1
              local.get 8
              i32.store offset=12
              br 1 (;@4;)
            end
            block  ;; label = @5
              local.get 7
              i32.const 20
              i32.add
              local.tee 1
              i32.load
              local.tee 5
              br_if 0 (;@5;)
              local.get 7
              i32.const 16
              i32.add
              local.tee 1
              i32.load
              local.tee 5
              br_if 0 (;@5;)
              i32.const 0
              local.set 8
              br 1 (;@4;)
            end
            loop  ;; label = @5
              local.get 1
              local.set 11
              local.get 5
              local.tee 8
              i32.const 20
              i32.add
              local.tee 1
              i32.load
              local.tee 5
              br_if 0 (;@5;)
              local.get 8
              i32.const 16
              i32.add
              local.set 1
              local.get 8
              i32.load offset=16
              local.tee 5
              br_if 0 (;@5;)
            end
            local.get 11
            i32.const 0
            i32.store
          end
          local.get 12
          i32.eqz
          br_if 0 (;@3;)
          block  ;; label = @4
            block  ;; label = @5
              local.get 7
              local.get 7
              i32.load offset=28
              local.tee 5
              i32.const 2
              i32.shl
              i32.const 1059464
              i32.add
              local.tee 1
              i32.load
              i32.ne
              br_if 0 (;@5;)
              local.get 1
              local.get 8
              i32.store
              local.get 8
              br_if 1 (;@4;)
              i32.const 0
              i32.const 0
              i32.load offset=1059164
              i32.const -2
              local.get 5
              i32.rotl
              i32.and
              i32.store offset=1059164
              br 2 (;@3;)
            end
            local.get 12
            i32.const 16
            i32.const 20
            local.get 12
            i32.load offset=16
            local.get 7
            i32.eq
            select
            i32.add
            local.get 8
            i32.store
            local.get 8
            i32.eqz
            br_if 1 (;@3;)
          end
          local.get 8
          local.get 12
          i32.store offset=24
          block  ;; label = @4
            local.get 7
            i32.load offset=16
            local.tee 1
            i32.eqz
            br_if 0 (;@4;)
            local.get 8
            local.get 1
            i32.store offset=16
            local.get 1
            local.get 8
            i32.store offset=24
          end
          local.get 7
          i32.load offset=20
          local.tee 1
          i32.eqz
          br_if 0 (;@3;)
          local.get 8
          i32.const 20
          i32.add
          local.get 1
          i32.store
          local.get 1
          local.get 8
          i32.store offset=24
        end
        block  ;; label = @3
          local.get 10
          i32.const 15
          i32.gt_u
          br_if 0 (;@3;)
          local.get 3
          local.get 4
          i32.const 1
          i32.and
          local.get 9
          i32.or
          i32.const 2
          i32.or
          i32.store
          local.get 6
          local.get 9
          i32.add
          local.tee 1
          local.get 1
          i32.load offset=4
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 0
          return
        end
        local.get 3
        local.get 2
        local.get 4
        i32.const 1
        i32.and
        i32.or
        i32.const 2
        i32.or
        i32.store
        local.get 6
        local.get 2
        i32.add
        local.tee 1
        local.get 10
        i32.const 3
        i32.or
        i32.store offset=4
        local.get 6
        local.get 9
        i32.add
        local.tee 2
        local.get 2
        i32.load offset=4
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 1
        local.get 10
        call $dispose_chunk
        local.get 0
        return
      end
      block  ;; label = @2
        local.get 1
        call $dlmalloc
        local.tee 2
        br_if 0 (;@2;)
        i32.const 0
        return
      end
      local.get 2
      local.get 0
      i32.const -4
      i32.const -8
      local.get 3
      i32.load
      local.tee 5
      i32.const 3
      i32.and
      select
      local.get 5
      i32.const -8
      i32.and
      i32.add
      local.tee 5
      local.get 1
      local.get 5
      local.get 1
      i32.lt_u
      select
      call $memcpy
      local.set 1
      local.get 0
      call $dlfree
      local.get 1
      local.set 0
    end
    local.get 0)
  (func $dispose_chunk (type 6) (param i32 i32)
    (local i32 i32 i32 i32 i32 i32)
    local.get 0
    local.get 1
    i32.add
    local.set 2
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load offset=4
        local.tee 3
        i32.const 1
        i32.and
        br_if 0 (;@2;)
        local.get 3
        i32.const 3
        i32.and
        i32.eqz
        br_if 1 (;@1;)
        local.get 0
        i32.load
        local.tee 3
        local.get 1
        i32.add
        local.set 1
        block  ;; label = @3
          block  ;; label = @4
            local.get 0
            local.get 3
            i32.sub
            local.tee 0
            i32.const 0
            i32.load offset=1059180
            i32.eq
            br_if 0 (;@4;)
            block  ;; label = @5
              local.get 3
              i32.const 255
              i32.gt_u
              br_if 0 (;@5;)
              local.get 0
              i32.load offset=8
              local.tee 4
              local.get 3
              i32.const 3
              i32.shr_u
              local.tee 5
              i32.const 3
              i32.shl
              i32.const 1059200
              i32.add
              local.tee 6
              i32.eq
              drop
              local.get 0
              i32.load offset=12
              local.tee 3
              local.get 4
              i32.ne
              br_if 2 (;@3;)
              i32.const 0
              i32.const 0
              i32.load offset=1059160
              i32.const -2
              local.get 5
              i32.rotl
              i32.and
              i32.store offset=1059160
              br 3 (;@2;)
            end
            local.get 0
            i32.load offset=24
            local.set 7
            block  ;; label = @5
              block  ;; label = @6
                local.get 0
                i32.load offset=12
                local.tee 6
                local.get 0
                i32.eq
                br_if 0 (;@6;)
                local.get 0
                i32.load offset=8
                local.tee 3
                i32.const 0
                i32.load offset=1059176
                i32.lt_u
                drop
                local.get 6
                local.get 3
                i32.store offset=8
                local.get 3
                local.get 6
                i32.store offset=12
                br 1 (;@5;)
              end
              block  ;; label = @6
                local.get 0
                i32.const 20
                i32.add
                local.tee 3
                i32.load
                local.tee 4
                br_if 0 (;@6;)
                local.get 0
                i32.const 16
                i32.add
                local.tee 3
                i32.load
                local.tee 4
                br_if 0 (;@6;)
                i32.const 0
                local.set 6
                br 1 (;@5;)
              end
              loop  ;; label = @6
                local.get 3
                local.set 5
                local.get 4
                local.tee 6
                i32.const 20
                i32.add
                local.tee 3
                i32.load
                local.tee 4
                br_if 0 (;@6;)
                local.get 6
                i32.const 16
                i32.add
                local.set 3
                local.get 6
                i32.load offset=16
                local.tee 4
                br_if 0 (;@6;)
              end
              local.get 5
              i32.const 0
              i32.store
            end
            local.get 7
            i32.eqz
            br_if 2 (;@2;)
            block  ;; label = @5
              block  ;; label = @6
                local.get 0
                local.get 0
                i32.load offset=28
                local.tee 4
                i32.const 2
                i32.shl
                i32.const 1059464
                i32.add
                local.tee 3
                i32.load
                i32.ne
                br_if 0 (;@6;)
                local.get 3
                local.get 6
                i32.store
                local.get 6
                br_if 1 (;@5;)
                i32.const 0
                i32.const 0
                i32.load offset=1059164
                i32.const -2
                local.get 4
                i32.rotl
                i32.and
                i32.store offset=1059164
                br 4 (;@2;)
              end
              local.get 7
              i32.const 16
              i32.const 20
              local.get 7
              i32.load offset=16
              local.get 0
              i32.eq
              select
              i32.add
              local.get 6
              i32.store
              local.get 6
              i32.eqz
              br_if 3 (;@2;)
            end
            local.get 6
            local.get 7
            i32.store offset=24
            block  ;; label = @5
              local.get 0
              i32.load offset=16
              local.tee 3
              i32.eqz
              br_if 0 (;@5;)
              local.get 6
              local.get 3
              i32.store offset=16
              local.get 3
              local.get 6
              i32.store offset=24
            end
            local.get 0
            i32.load offset=20
            local.tee 3
            i32.eqz
            br_if 2 (;@2;)
            local.get 6
            i32.const 20
            i32.add
            local.get 3
            i32.store
            local.get 3
            local.get 6
            i32.store offset=24
            br 2 (;@2;)
          end
          local.get 2
          i32.load offset=4
          local.tee 3
          i32.const 3
          i32.and
          i32.const 3
          i32.ne
          br_if 1 (;@2;)
          local.get 2
          local.get 3
          i32.const -2
          i32.and
          i32.store offset=4
          i32.const 0
          local.get 1
          i32.store offset=1059168
          local.get 2
          local.get 1
          i32.store
          local.get 0
          local.get 1
          i32.const 1
          i32.or
          i32.store offset=4
          return
        end
        local.get 3
        local.get 6
        i32.eq
        drop
        local.get 3
        local.get 4
        i32.store offset=8
        local.get 4
        local.get 3
        i32.store offset=12
      end
      block  ;; label = @2
        block  ;; label = @3
          local.get 2
          i32.load offset=4
          local.tee 3
          i32.const 2
          i32.and
          br_if 0 (;@3;)
          block  ;; label = @4
            local.get 2
            i32.const 0
            i32.load offset=1059184
            i32.ne
            br_if 0 (;@4;)
            i32.const 0
            local.get 0
            i32.store offset=1059184
            i32.const 0
            i32.const 0
            i32.load offset=1059172
            local.get 1
            i32.add
            local.tee 1
            i32.store offset=1059172
            local.get 0
            local.get 1
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 0
            i32.const 0
            i32.load offset=1059180
            i32.ne
            br_if 3 (;@1;)
            i32.const 0
            i32.const 0
            i32.store offset=1059168
            i32.const 0
            i32.const 0
            i32.store offset=1059180
            return
          end
          block  ;; label = @4
            local.get 2
            i32.const 0
            i32.load offset=1059180
            i32.ne
            br_if 0 (;@4;)
            i32.const 0
            local.get 0
            i32.store offset=1059180
            i32.const 0
            i32.const 0
            i32.load offset=1059168
            local.get 1
            i32.add
            local.tee 1
            i32.store offset=1059168
            local.get 0
            local.get 1
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 0
            local.get 1
            i32.add
            local.get 1
            i32.store
            return
          end
          local.get 3
          i32.const -8
          i32.and
          local.get 1
          i32.add
          local.set 1
          block  ;; label = @4
            block  ;; label = @5
              local.get 3
              i32.const 255
              i32.gt_u
              br_if 0 (;@5;)
              local.get 2
              i32.load offset=8
              local.tee 4
              local.get 3
              i32.const 3
              i32.shr_u
              local.tee 5
              i32.const 3
              i32.shl
              i32.const 1059200
              i32.add
              local.tee 6
              i32.eq
              drop
              block  ;; label = @6
                local.get 2
                i32.load offset=12
                local.tee 3
                local.get 4
                i32.ne
                br_if 0 (;@6;)
                i32.const 0
                i32.const 0
                i32.load offset=1059160
                i32.const -2
                local.get 5
                i32.rotl
                i32.and
                i32.store offset=1059160
                br 2 (;@4;)
              end
              local.get 3
              local.get 6
              i32.eq
              drop
              local.get 3
              local.get 4
              i32.store offset=8
              local.get 4
              local.get 3
              i32.store offset=12
              br 1 (;@4;)
            end
            local.get 2
            i32.load offset=24
            local.set 7
            block  ;; label = @5
              block  ;; label = @6
                local.get 2
                i32.load offset=12
                local.tee 6
                local.get 2
                i32.eq
                br_if 0 (;@6;)
                local.get 2
                i32.load offset=8
                local.tee 3
                i32.const 0
                i32.load offset=1059176
                i32.lt_u
                drop
                local.get 6
                local.get 3
                i32.store offset=8
                local.get 3
                local.get 6
                i32.store offset=12
                br 1 (;@5;)
              end
              block  ;; label = @6
                local.get 2
                i32.const 20
                i32.add
                local.tee 4
                i32.load
                local.tee 3
                br_if 0 (;@6;)
                local.get 2
                i32.const 16
                i32.add
                local.tee 4
                i32.load
                local.tee 3
                br_if 0 (;@6;)
                i32.const 0
                local.set 6
                br 1 (;@5;)
              end
              loop  ;; label = @6
                local.get 4
                local.set 5
                local.get 3
                local.tee 6
                i32.const 20
                i32.add
                local.tee 4
                i32.load
                local.tee 3
                br_if 0 (;@6;)
                local.get 6
                i32.const 16
                i32.add
                local.set 4
                local.get 6
                i32.load offset=16
                local.tee 3
                br_if 0 (;@6;)
              end
              local.get 5
              i32.const 0
              i32.store
            end
            local.get 7
            i32.eqz
            br_if 0 (;@4;)
            block  ;; label = @5
              block  ;; label = @6
                local.get 2
                local.get 2
                i32.load offset=28
                local.tee 4
                i32.const 2
                i32.shl
                i32.const 1059464
                i32.add
                local.tee 3
                i32.load
                i32.ne
                br_if 0 (;@6;)
                local.get 3
                local.get 6
                i32.store
                local.get 6
                br_if 1 (;@5;)
                i32.const 0
                i32.const 0
                i32.load offset=1059164
                i32.const -2
                local.get 4
                i32.rotl
                i32.and
                i32.store offset=1059164
                br 2 (;@4;)
              end
              local.get 7
              i32.const 16
              i32.const 20
              local.get 7
              i32.load offset=16
              local.get 2
              i32.eq
              select
              i32.add
              local.get 6
              i32.store
              local.get 6
              i32.eqz
              br_if 1 (;@4;)
            end
            local.get 6
            local.get 7
            i32.store offset=24
            block  ;; label = @5
              local.get 2
              i32.load offset=16
              local.tee 3
              i32.eqz
              br_if 0 (;@5;)
              local.get 6
              local.get 3
              i32.store offset=16
              local.get 3
              local.get 6
              i32.store offset=24
            end
            local.get 2
            i32.load offset=20
            local.tee 3
            i32.eqz
            br_if 0 (;@4;)
            local.get 6
            i32.const 20
            i32.add
            local.get 3
            i32.store
            local.get 3
            local.get 6
            i32.store offset=24
          end
          local.get 0
          local.get 1
          i32.add
          local.get 1
          i32.store
          local.get 0
          local.get 1
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 0
          i32.const 0
          i32.load offset=1059180
          i32.ne
          br_if 1 (;@2;)
          i32.const 0
          local.get 1
          i32.store offset=1059168
          return
        end
        local.get 2
        local.get 3
        i32.const -2
        i32.and
        i32.store offset=4
        local.get 0
        local.get 1
        i32.add
        local.get 1
        i32.store
        local.get 0
        local.get 1
        i32.const 1
        i32.or
        i32.store offset=4
      end
      block  ;; label = @2
        local.get 1
        i32.const 255
        i32.gt_u
        br_if 0 (;@2;)
        local.get 1
        i32.const -8
        i32.and
        i32.const 1059200
        i32.add
        local.set 3
        block  ;; label = @3
          block  ;; label = @4
            i32.const 0
            i32.load offset=1059160
            local.tee 4
            i32.const 1
            local.get 1
            i32.const 3
            i32.shr_u
            i32.shl
            local.tee 1
            i32.and
            br_if 0 (;@4;)
            i32.const 0
            local.get 4
            local.get 1
            i32.or
            i32.store offset=1059160
            local.get 3
            local.set 1
            br 1 (;@3;)
          end
          local.get 3
          i32.load offset=8
          local.set 1
        end
        local.get 1
        local.get 0
        i32.store offset=12
        local.get 3
        local.get 0
        i32.store offset=8
        local.get 0
        local.get 3
        i32.store offset=12
        local.get 0
        local.get 1
        i32.store offset=8
        return
      end
      i32.const 31
      local.set 3
      block  ;; label = @2
        local.get 1
        i32.const 16777215
        i32.gt_u
        br_if 0 (;@2;)
        local.get 1
        i32.const 8
        i32.shr_u
        local.tee 3
        local.get 3
        i32.const 1048320
        i32.add
        i32.const 16
        i32.shr_u
        i32.const 8
        i32.and
        local.tee 3
        i32.shl
        local.tee 4
        local.get 4
        i32.const 520192
        i32.add
        i32.const 16
        i32.shr_u
        i32.const 4
        i32.and
        local.tee 4
        i32.shl
        local.tee 6
        local.get 6
        i32.const 245760
        i32.add
        i32.const 16
        i32.shr_u
        i32.const 2
        i32.and
        local.tee 6
        i32.shl
        i32.const 15
        i32.shr_u
        local.get 3
        local.get 4
        i32.or
        local.get 6
        i32.or
        i32.sub
        local.tee 3
        i32.const 1
        i32.shl
        local.get 1
        local.get 3
        i32.const 21
        i32.add
        i32.shr_u
        i32.const 1
        i32.and
        i32.or
        i32.const 28
        i32.add
        local.set 3
      end
      local.get 0
      local.get 3
      i32.store offset=28
      local.get 0
      i64.const 0
      i64.store offset=16 align=4
      local.get 3
      i32.const 2
      i32.shl
      i32.const 1059464
      i32.add
      local.set 4
      block  ;; label = @2
        i32.const 0
        i32.load offset=1059164
        local.tee 6
        i32.const 1
        local.get 3
        i32.shl
        local.tee 2
        i32.and
        br_if 0 (;@2;)
        local.get 4
        local.get 0
        i32.store
        i32.const 0
        local.get 6
        local.get 2
        i32.or
        i32.store offset=1059164
        local.get 0
        local.get 4
        i32.store offset=24
        local.get 0
        local.get 0
        i32.store offset=8
        local.get 0
        local.get 0
        i32.store offset=12
        return
      end
      local.get 1
      i32.const 0
      i32.const 25
      local.get 3
      i32.const 1
      i32.shr_u
      i32.sub
      local.get 3
      i32.const 31
      i32.eq
      select
      i32.shl
      local.set 3
      local.get 4
      i32.load
      local.set 6
      block  ;; label = @2
        loop  ;; label = @3
          local.get 6
          local.tee 4
          i32.load offset=4
          i32.const -8
          i32.and
          local.get 1
          i32.eq
          br_if 1 (;@2;)
          local.get 3
          i32.const 29
          i32.shr_u
          local.set 6
          local.get 3
          i32.const 1
          i32.shl
          local.set 3
          local.get 4
          local.get 6
          i32.const 4
          i32.and
          i32.add
          i32.const 16
          i32.add
          local.tee 2
          i32.load
          local.tee 6
          br_if 0 (;@3;)
        end
        local.get 2
        local.get 0
        i32.store
        local.get 0
        local.get 4
        i32.store offset=24
        local.get 0
        local.get 0
        i32.store offset=12
        local.get 0
        local.get 0
        i32.store offset=8
        return
      end
      local.get 4
      i32.load offset=8
      local.tee 1
      local.get 0
      i32.store offset=12
      local.get 4
      local.get 0
      i32.store offset=8
      local.get 0
      i32.const 0
      i32.store offset=24
      local.get 0
      local.get 4
      i32.store offset=12
      local.get 0
      local.get 1
      i32.store offset=8
    end)
  (func $internal_memalign (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.const 16
        local.get 0
        i32.const 16
        i32.gt_u
        select
        local.tee 2
        local.get 2
        i32.const -1
        i32.add
        i32.and
        br_if 0 (;@2;)
        local.get 2
        local.set 0
        br 1 (;@1;)
      end
      i32.const 32
      local.set 3
      loop  ;; label = @2
        local.get 3
        local.tee 0
        i32.const 1
        i32.shl
        local.set 3
        local.get 0
        local.get 2
        i32.lt_u
        br_if 0 (;@2;)
      end
    end
    block  ;; label = @1
      i32.const -64
      local.get 0
      i32.sub
      local.get 1
      i32.gt_u
      br_if 0 (;@1;)
      i32.const 0
      i32.const 48
      i32.store offset=1059656
      i32.const 0
      return
    end
    block  ;; label = @1
      local.get 0
      i32.const 16
      local.get 1
      i32.const 19
      i32.add
      i32.const -16
      i32.and
      local.get 1
      i32.const 11
      i32.lt_u
      select
      local.tee 1
      i32.add
      i32.const 12
      i32.add
      call $dlmalloc
      local.tee 3
      br_if 0 (;@1;)
      i32.const 0
      return
    end
    local.get 3
    i32.const -8
    i32.add
    local.set 2
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.const -1
        i32.add
        local.get 3
        i32.and
        br_if 0 (;@2;)
        local.get 2
        local.set 0
        br 1 (;@1;)
      end
      local.get 3
      i32.const -4
      i32.add
      local.tee 4
      i32.load
      local.tee 5
      i32.const -8
      i32.and
      local.get 3
      local.get 0
      i32.add
      i32.const -1
      i32.add
      i32.const 0
      local.get 0
      i32.sub
      i32.and
      i32.const -8
      i32.add
      local.tee 3
      i32.const 0
      local.get 0
      local.get 3
      local.get 2
      i32.sub
      i32.const 15
      i32.gt_u
      select
      i32.add
      local.tee 0
      local.get 2
      i32.sub
      local.tee 3
      i32.sub
      local.set 6
      block  ;; label = @2
        local.get 5
        i32.const 3
        i32.and
        br_if 0 (;@2;)
        local.get 0
        local.get 6
        i32.store offset=4
        local.get 0
        local.get 2
        i32.load
        local.get 3
        i32.add
        i32.store
        br 1 (;@1;)
      end
      local.get 0
      local.get 6
      local.get 0
      i32.load offset=4
      i32.const 1
      i32.and
      i32.or
      i32.const 2
      i32.or
      i32.store offset=4
      local.get 0
      local.get 6
      i32.add
      local.tee 6
      local.get 6
      i32.load offset=4
      i32.const 1
      i32.or
      i32.store offset=4
      local.get 4
      local.get 3
      local.get 4
      i32.load
      i32.const 1
      i32.and
      i32.or
      i32.const 2
      i32.or
      i32.store
      local.get 2
      local.get 3
      i32.add
      local.tee 6
      local.get 6
      i32.load offset=4
      i32.const 1
      i32.or
      i32.store offset=4
      local.get 2
      local.get 3
      call $dispose_chunk
    end
    block  ;; label = @1
      local.get 0
      i32.load offset=4
      local.tee 3
      i32.const 3
      i32.and
      i32.eqz
      br_if 0 (;@1;)
      local.get 3
      i32.const -8
      i32.and
      local.tee 2
      local.get 1
      i32.const 16
      i32.add
      i32.le_u
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      local.get 3
      i32.const 1
      i32.and
      i32.or
      i32.const 2
      i32.or
      i32.store offset=4
      local.get 0
      local.get 1
      i32.add
      local.tee 3
      local.get 2
      local.get 1
      i32.sub
      local.tee 1
      i32.const 3
      i32.or
      i32.store offset=4
      local.get 0
      local.get 2
      i32.add
      local.tee 2
      local.get 2
      i32.load offset=4
      i32.const 1
      i32.or
      i32.store offset=4
      local.get 3
      local.get 1
      call $dispose_chunk
    end
    local.get 0
    i32.const 8
    i32.add)
  (func $aligned_alloc (type 2) (param i32 i32) (result i32)
    block  ;; label = @1
      local.get 0
      i32.const 16
      i32.gt_u
      br_if 0 (;@1;)
      local.get 1
      call $dlmalloc
      return
    end
    local.get 0
    local.get 1
    call $internal_memalign)
  (func $close (type 4) (param i32) (result i32)
    block  ;; label = @1
      local.get 0
      call $__wasi_fd_close
      local.tee 0
      br_if 0 (;@1;)
      i32.const 0
      return
    end
    i32.const 0
    local.get 0
    i32.store offset=1059656
    i32.const -1)
  (func $_Exit (type 1) (param i32)
    local.get 0
    call $__wasi_proc_exit
    unreachable)
  (func $__wasilibc_ensure_environ (type 0)
    block  ;; label = @1
      i32.const 0
      i32.load offset=1059036
      i32.const -1
      i32.ne
      br_if 0 (;@1;)
      call $__wasilibc_initialize_environ
    end)
  (func $__wasilibc_initialize_environ (type 0)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 0
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.const 12
        i32.add
        local.get 0
        i32.const 8
        i32.add
        call $__wasi_environ_sizes_get
        br_if 0 (;@2;)
        block  ;; label = @3
          local.get 0
          i32.load offset=12
          local.tee 1
          br_if 0 (;@3;)
          i32.const 1059660
          local.set 1
          br 2 (;@1;)
        end
        block  ;; label = @3
          block  ;; label = @4
            local.get 1
            i32.const 1
            i32.add
            local.tee 1
            i32.eqz
            br_if 0 (;@4;)
            local.get 0
            i32.load offset=8
            call $malloc
            local.tee 2
            i32.eqz
            br_if 0 (;@4;)
            local.get 1
            i32.const 4
            call $calloc
            local.tee 1
            br_if 1 (;@3;)
            local.get 2
            call $free
          end
          i32.const 70
          call $_Exit
          unreachable
        end
        local.get 1
        local.get 2
        call $__wasi_environ_get
        i32.eqz
        br_if 1 (;@1;)
        local.get 2
        call $free
        local.get 1
        call $free
      end
      i32.const 71
      call $_Exit
      unreachable
    end
    i32.const 0
    local.get 1
    i32.store offset=1059036
    local.get 0
    i32.const 16
    i32.add
    global.set $__stack_pointer)
  (func $__wasi_environ_get (type 2) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call $__imported_wasi_snapshot_preview1_environ_get
    i32.const 65535
    i32.and)
  (func $__wasi_environ_sizes_get (type 2) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call $__imported_wasi_snapshot_preview1_environ_sizes_get
    i32.const 65535
    i32.and)
  (func $__wasi_fd_close (type 4) (param i32) (result i32)
    local.get 0
    call $__imported_wasi_snapshot_preview1_fd_close
    i32.const 65535
    i32.and)
  (func $__wasi_fd_prestat_get (type 2) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call $__imported_wasi_snapshot_preview1_fd_prestat_get
    i32.const 65535
    i32.and)
  (func $__wasi_fd_prestat_dir_name (type 7) (param i32 i32 i32) (result i32)
    local.get 0
    local.get 1
    local.get 2
    call $__imported_wasi_snapshot_preview1_fd_prestat_dir_name
    i32.const 65535
    i32.and)
  (func $__wasi_proc_exit (type 1) (param i32)
    local.get 0
    call $__imported_wasi_snapshot_preview1_proc_exit
    unreachable)
  (func $abort (type 0)
    unreachable
    unreachable)
  (func $getcwd (type 2) (param i32 i32) (result i32)
    (local i32)
    i32.const 0
    i32.load offset=1059040
    local.set 2
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        br_if 0 (;@2;)
        local.get 2
        call $strdup
        local.tee 0
        br_if 1 (;@1;)
        i32.const 0
        i32.const 48
        i32.store offset=1059656
        i32.const 0
        return
      end
      block  ;; label = @2
        local.get 2
        call $strlen
        i32.const 1
        i32.add
        local.get 1
        i32.gt_u
        br_if 0 (;@2;)
        local.get 0
        local.get 2
        call $strcpy
        return
      end
      i32.const 0
      local.set 0
      i32.const 0
      i32.const 68
      i32.store offset=1059656
    end
    local.get 0)
  (func $internal_register_preopened_fd (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.const 2
        i32.add
        br_table 1 (;@1;) 1 (;@1;) 0 (;@2;)
      end
      local.get 1
      i32.eqz
      br_if 0 (;@1;)
      block  ;; label = @2
        i32.const 0
        i32.load offset=1059664
        local.tee 2
        i32.const 0
        i32.load offset=1059672
        i32.ne
        br_if 0 (;@2;)
        i32.const 0
        i32.load offset=1059668
        local.set 3
        block  ;; label = @3
          i32.const 8
          local.get 2
          i32.const 1
          i32.shl
          i32.const 4
          local.get 2
          select
          local.tee 4
          call $calloc
          local.tee 5
          br_if 0 (;@3;)
          i32.const -1
          return
        end
        local.get 5
        local.get 3
        local.get 2
        i32.const 3
        i32.shl
        call $memcpy
        local.set 5
        i32.const 0
        local.get 4
        i32.store offset=1059672
        i32.const 0
        local.get 5
        i32.store offset=1059668
        local.get 3
        call $free
      end
      block  ;; label = @2
        loop  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 1
              local.tee 3
              i32.load8_u
              i32.const -46
              i32.add
              br_table 1 (;@4;) 0 (;@5;) 3 (;@2;)
            end
            local.get 3
            i32.const 1
            i32.add
            local.set 1
            br 1 (;@3;)
          end
          local.get 3
          i32.const 1
          i32.add
          local.set 1
          local.get 3
          i32.load8_u offset=1
          local.tee 4
          i32.eqz
          br_if 0 (;@3;)
          local.get 4
          i32.const 47
          i32.ne
          br_if 1 (;@2;)
          local.get 3
          i32.const 2
          i32.add
          local.set 1
          br 0 (;@3;)
        end
      end
      block  ;; label = @2
        local.get 3
        call $strdup
        local.tee 3
        br_if 0 (;@2;)
        i32.const -1
        return
      end
      i32.const 0
      local.get 2
      i32.const 1
      i32.add
      i32.store offset=1059664
      i32.const 0
      i32.load offset=1059668
      local.get 2
      i32.const 3
      i32.shl
      i32.add
      local.tee 1
      local.get 0
      i32.store offset=4
      local.get 1
      local.get 3
      i32.store
      i32.const 0
      return
    end
    call $abort
    unreachable)
  (func $__wasilibc_find_relpath (type 8) (param i32 i32 i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    local.get 4
    local.get 3
    i32.store offset=12
    block  ;; label = @1
      block  ;; label = @2
        i32.const 73
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        local.get 1
        local.get 2
        local.get 4
        i32.const 12
        i32.add
        i32.const 0
        call $__wasilibc_find_relpath_alloc
        local.set 3
        br 1 (;@1;)
      end
      local.get 0
      local.get 1
      local.get 2
      call $__wasilibc_find_abspath
      local.set 3
    end
    local.get 4
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 3)
  (func $__wasilibc_find_abspath (type 7) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    local.get 0
    i32.const -1
    i32.add
    local.set 3
    loop  ;; label = @1
      local.get 3
      i32.const 1
      i32.add
      local.tee 3
      i32.load8_u
      i32.const 47
      i32.eq
      br_if 0 (;@1;)
    end
    i32.const 0
    local.set 4
    block  ;; label = @1
      block  ;; label = @2
        i32.const 0
        i32.load offset=1059664
        local.tee 5
        i32.eqz
        br_if 0 (;@2;)
        i32.const 0
        i32.load offset=1059668
        local.set 6
        i32.const -1
        local.set 7
        loop  ;; label = @3
          local.get 6
          local.get 5
          i32.const -1
          i32.add
          local.tee 5
          i32.const 3
          i32.shl
          i32.add
          local.tee 8
          i32.load
          local.tee 9
          call $strlen
          local.set 10
          block  ;; label = @4
            block  ;; label = @5
              local.get 7
              i32.const -1
              i32.eq
              br_if 0 (;@5;)
              local.get 10
              local.get 4
              i32.le_u
              br_if 1 (;@4;)
            end
            block  ;; label = @5
              block  ;; label = @6
                local.get 10
                br_if 0 (;@6;)
                local.get 3
                i32.load8_u
                i32.const 255
                i32.and
                i32.const 47
                i32.ne
                br_if 1 (;@5;)
              end
              local.get 3
              local.get 9
              local.get 10
              call $memcmp
              br_if 1 (;@4;)
              local.get 9
              i32.const -1
              i32.add
              local.set 11
              local.get 10
              local.set 12
              block  ;; label = @6
                loop  ;; label = @7
                  local.get 12
                  local.tee 0
                  i32.eqz
                  br_if 1 (;@6;)
                  local.get 0
                  i32.const -1
                  i32.add
                  local.set 12
                  local.get 11
                  local.get 0
                  i32.add
                  i32.load8_u
                  i32.const 47
                  i32.eq
                  br_if 0 (;@7;)
                end
              end
              local.get 3
              local.get 0
              i32.add
              i32.load8_u
              local.tee 0
              i32.const 47
              i32.eq
              br_if 0 (;@5;)
              local.get 0
              br_if 1 (;@4;)
            end
            local.get 1
            local.get 9
            i32.store
            local.get 8
            i32.load offset=4
            local.set 7
            local.get 10
            local.set 4
          end
          local.get 5
          br_if 0 (;@3;)
        end
        local.get 7
        i32.const -1
        i32.ne
        br_if 1 (;@1;)
      end
      i32.const 0
      i32.const 44
      i32.store offset=1059656
      i32.const -1
      return
    end
    local.get 3
    local.get 4
    i32.add
    i32.const -1
    i32.add
    local.set 0
    loop  ;; label = @1
      local.get 0
      i32.const 1
      i32.add
      local.tee 0
      i32.load8_u
      local.tee 3
      i32.const 47
      i32.eq
      br_if 0 (;@1;)
    end
    local.get 2
    local.get 0
    i32.const 1052762
    local.get 3
    select
    i32.store
    local.get 7)
  (func $__wasilibc_populate_preopens (type 0)
    (local i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 0
    global.set $__stack_pointer
    i32.const 3
    local.set 1
    block  ;; label = @1
      block  ;; label = @2
        loop  ;; label = @3
          block  ;; label = @4
            local.get 1
            local.get 0
            i32.const 8
            i32.add
            call $__wasi_fd_prestat_get
            local.tee 2
            i32.eqz
            br_if 0 (;@4;)
            local.get 2
            i32.const 8
            i32.ne
            br_if 2 (;@2;)
            local.get 0
            i32.const 16
            i32.add
            global.set $__stack_pointer
            return
          end
          block  ;; label = @4
            local.get 0
            i32.load8_u offset=8
            br_if 0 (;@4;)
            local.get 0
            i32.load offset=12
            local.tee 3
            i32.const 1
            i32.add
            call $malloc
            local.tee 2
            i32.eqz
            br_if 3 (;@1;)
            local.get 1
            local.get 2
            local.get 3
            call $__wasi_fd_prestat_dir_name
            br_if 2 (;@2;)
            local.get 2
            local.get 0
            i32.load offset=12
            i32.add
            i32.const 0
            i32.store8
            local.get 1
            local.get 2
            call $internal_register_preopened_fd
            br_if 3 (;@1;)
            local.get 2
            call $free
          end
          local.get 1
          i32.const 1
          i32.add
          local.set 1
          br 0 (;@3;)
        end
      end
      i32.const 71
      call $_Exit
      unreachable
    end
    i32.const 70
    call $_Exit
    unreachable)
  (func $sbrk (type 4) (param i32) (result i32)
    block  ;; label = @1
      local.get 0
      br_if 0 (;@1;)
      memory.size
      i32.const 16
      i32.shl
      return
    end
    block  ;; label = @1
      local.get 0
      i32.const 65535
      i32.and
      br_if 0 (;@1;)
      local.get 0
      i32.const -1
      i32.le_s
      br_if 0 (;@1;)
      block  ;; label = @2
        local.get 0
        i32.const 16
        i32.shr_u
        memory.grow
        local.tee 0
        i32.const -1
        i32.ne
        br_if 0 (;@2;)
        i32.const 0
        i32.const 48
        i32.store offset=1059656
        i32.const -1
        return
      end
      local.get 0
      i32.const 16
      i32.shl
      return
    end
    call $abort
    unreachable)
  (func $__wasilibc_find_relpath_alloc (type 12) (param i32 i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 5
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 0
                i32.load8_u
                local.tee 6
                i32.eqz
                br_if 0 (;@6;)
                block  ;; label = @7
                  local.get 6
                  i32.const 47
                  i32.ne
                  br_if 0 (;@7;)
                  local.get 0
                  local.set 6
                  br 5 (;@2;)
                end
                local.get 0
                i32.const 1052764
                call $strcmp
                i32.eqz
                br_if 0 (;@6;)
                local.get 0
                i32.const 1052766
                call $strcmp
                br_if 1 (;@5;)
              end
              i32.const 0
              i32.load offset=1059040
              local.set 6
              br 1 (;@4;)
            end
            block  ;; label = @5
              local.get 6
              i32.const 46
              i32.ne
              br_if 0 (;@5;)
              local.get 0
              local.get 0
              i32.load8_u offset=1
              i32.const 47
              i32.eq
              i32.const 1
              i32.shl
              i32.add
              local.set 0
            end
            i32.const 0
            i32.load offset=1059676
            local.set 7
            block  ;; label = @5
              i32.const 0
              i32.load offset=1059040
              local.tee 6
              call $strlen
              local.tee 8
              local.get 0
              call $strlen
              i32.add
              local.get 8
              local.get 6
              i32.add
              i32.const -1
              i32.add
              i32.load8_u
              local.tee 9
              i32.const 47
              i32.ne
              local.tee 10
              i32.add
              i32.const 1
              i32.add
              local.tee 11
              i32.const 0
              i32.load offset=1059680
              i32.le_u
              br_if 0 (;@5;)
              local.get 7
              local.get 11
              call $realloc
              local.tee 7
              i32.eqz
              br_if 2 (;@3;)
              i32.const 0
              local.get 11
              i32.store offset=1059680
              i32.const 0
              local.get 7
              i32.store offset=1059676
              i32.const 0
              i32.load offset=1059040
              local.set 6
            end
            local.get 7
            local.get 6
            call $strcpy
            local.set 6
            block  ;; label = @5
              local.get 9
              i32.const 47
              i32.eq
              br_if 0 (;@5;)
              local.get 6
              local.get 8
              i32.add
              i32.const 47
              i32.store16 align=1
            end
            local.get 6
            local.get 8
            i32.add
            local.get 10
            i32.add
            local.get 0
            call $strcpy
            drop
          end
          local.get 6
          br_if 1 (;@2;)
        end
        i32.const 0
        i32.const 48
        i32.store offset=1059656
        i32.const -1
        local.set 0
        br 1 (;@1;)
      end
      block  ;; label = @2
        local.get 6
        local.get 1
        local.get 5
        i32.const 12
        i32.add
        call $__wasilibc_find_abspath
        local.tee 0
        i32.const -1
        i32.ne
        br_if 0 (;@2;)
        i32.const -1
        local.set 0
        br 1 (;@1;)
      end
      block  ;; label = @2
        block  ;; label = @3
          local.get 3
          i32.load
          local.get 5
          i32.load offset=12
          local.tee 6
          call $strlen
          i32.const 1
          i32.add
          local.tee 8
          i32.lt_u
          br_if 0 (;@3;)
          local.get 2
          i32.load
          local.set 1
          br 1 (;@2;)
        end
        block  ;; label = @3
          local.get 4
          br_if 0 (;@3;)
          i32.const 0
          i32.const 68
          i32.store offset=1059656
          i32.const -1
          local.set 0
          br 2 (;@1;)
        end
        block  ;; label = @3
          local.get 2
          i32.load
          local.get 8
          call $realloc
          local.tee 1
          br_if 0 (;@3;)
          i32.const 0
          i32.const 48
          i32.store offset=1059656
          i32.const -1
          local.set 0
          br 2 (;@1;)
        end
        local.get 3
        local.get 8
        i32.store
        local.get 2
        local.get 1
        i32.store
        local.get 5
        i32.load offset=12
        local.set 6
      end
      local.get 1
      local.get 6
      call $strcpy
      drop
    end
    local.get 5
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 0)
  (func $dummy (type 0))
  (func $__wasm_call_dtors (type 0)
    call $dummy
    call $dummy)
  (func $getenv (type 4) (param i32) (result i32)
    (local i32 i32 i32 i32)
    call $__wasilibc_ensure_environ
    block  ;; label = @1
      local.get 0
      i32.const 61
      call $__strchrnul
      local.tee 1
      local.get 0
      i32.ne
      br_if 0 (;@1;)
      i32.const 0
      return
    end
    i32.const 0
    local.set 2
    block  ;; label = @1
      local.get 0
      local.get 1
      local.get 0
      i32.sub
      local.tee 3
      i32.add
      i32.load8_u
      br_if 0 (;@1;)
      i32.const 0
      i32.load offset=1059036
      local.tee 4
      i32.eqz
      br_if 0 (;@1;)
      local.get 4
      i32.load
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 4
      i32.const 4
      i32.add
      local.set 4
      block  ;; label = @2
        loop  ;; label = @3
          block  ;; label = @4
            local.get 0
            local.get 1
            local.get 3
            call $strncmp
            br_if 0 (;@4;)
            local.get 1
            local.get 3
            i32.add
            local.tee 1
            i32.load8_u
            i32.const 61
            i32.eq
            br_if 2 (;@2;)
          end
          local.get 4
          i32.load
          local.set 1
          local.get 4
          i32.const 4
          i32.add
          local.set 4
          local.get 1
          br_if 0 (;@3;)
          br 2 (;@1;)
        end
      end
      local.get 1
      i32.const 1
      i32.add
      local.set 2
    end
    local.get 2)
  (func $strdup (type 4) (param i32) (result i32)
    (local i32 i32)
    block  ;; label = @1
      local.get 0
      call $strlen
      i32.const 1
      i32.add
      local.tee 1
      call $malloc
      local.tee 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      local.get 0
      local.get 1
      call $memcpy
      drop
    end
    local.get 2)
  (func $memmove (type 7) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 2
          i32.const 33
          i32.ge_u
          br_if 0 (;@3;)
          local.get 0
          local.get 1
          i32.eq
          br_if 2 (;@1;)
          local.get 1
          local.get 0
          local.get 2
          i32.add
          local.tee 3
          i32.sub
          i32.const 0
          local.get 2
          i32.const 1
          i32.shl
          i32.sub
          i32.gt_u
          br_if 1 (;@2;)
        end
        local.get 0
        local.get 1
        local.get 2
        memory.copy
        br 1 (;@1;)
      end
      local.get 1
      local.get 0
      i32.xor
      i32.const 3
      i32.and
      local.set 4
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 0
            local.get 1
            i32.ge_u
            br_if 0 (;@4;)
            block  ;; label = @5
              local.get 4
              i32.eqz
              br_if 0 (;@5;)
              local.get 2
              local.set 4
              local.get 0
              local.set 3
              br 3 (;@2;)
            end
            block  ;; label = @5
              local.get 0
              i32.const 3
              i32.and
              br_if 0 (;@5;)
              local.get 2
              local.set 4
              local.get 0
              local.set 3
              br 2 (;@3;)
            end
            local.get 2
            i32.eqz
            br_if 3 (;@1;)
            local.get 0
            local.get 1
            i32.load8_u
            i32.store8
            local.get 2
            i32.const -1
            i32.add
            local.set 4
            block  ;; label = @5
              local.get 0
              i32.const 1
              i32.add
              local.tee 3
              i32.const 3
              i32.and
              br_if 0 (;@5;)
              local.get 1
              i32.const 1
              i32.add
              local.set 1
              br 2 (;@3;)
            end
            local.get 4
            i32.eqz
            br_if 3 (;@1;)
            local.get 0
            local.get 1
            i32.load8_u offset=1
            i32.store8 offset=1
            local.get 2
            i32.const -2
            i32.add
            local.set 4
            block  ;; label = @5
              local.get 0
              i32.const 2
              i32.add
              local.tee 3
              i32.const 3
              i32.and
              br_if 0 (;@5;)
              local.get 1
              i32.const 2
              i32.add
              local.set 1
              br 2 (;@3;)
            end
            local.get 4
            i32.eqz
            br_if 3 (;@1;)
            local.get 0
            local.get 1
            i32.load8_u offset=2
            i32.store8 offset=2
            local.get 2
            i32.const -3
            i32.add
            local.set 4
            block  ;; label = @5
              local.get 0
              i32.const 3
              i32.add
              local.tee 3
              i32.const 3
              i32.and
              br_if 0 (;@5;)
              local.get 1
              i32.const 3
              i32.add
              local.set 1
              br 2 (;@3;)
            end
            local.get 4
            i32.eqz
            br_if 3 (;@1;)
            local.get 0
            local.get 1
            i32.load8_u offset=3
            i32.store8 offset=3
            local.get 0
            i32.const 4
            i32.add
            local.set 3
            local.get 1
            i32.const 4
            i32.add
            local.set 1
            local.get 2
            i32.const -4
            i32.add
            local.set 4
            br 1 (;@3;)
          end
          block  ;; label = @4
            local.get 4
            br_if 0 (;@4;)
            block  ;; label = @5
              local.get 3
              i32.const 3
              i32.and
              i32.eqz
              br_if 0 (;@5;)
              local.get 2
              i32.eqz
              br_if 4 (;@1;)
              local.get 0
              local.get 2
              i32.const -1
              i32.add
              local.tee 3
              i32.add
              local.tee 4
              local.get 1
              local.get 3
              i32.add
              i32.load8_u
              i32.store8
              block  ;; label = @6
                local.get 4
                i32.const 3
                i32.and
                br_if 0 (;@6;)
                local.get 3
                local.set 2
                br 1 (;@5;)
              end
              local.get 3
              i32.eqz
              br_if 4 (;@1;)
              local.get 0
              local.get 2
              i32.const -2
              i32.add
              local.tee 3
              i32.add
              local.tee 4
              local.get 1
              local.get 3
              i32.add
              i32.load8_u
              i32.store8
              block  ;; label = @6
                local.get 4
                i32.const 3
                i32.and
                br_if 0 (;@6;)
                local.get 3
                local.set 2
                br 1 (;@5;)
              end
              local.get 3
              i32.eqz
              br_if 4 (;@1;)
              local.get 0
              local.get 2
              i32.const -3
              i32.add
              local.tee 3
              i32.add
              local.tee 4
              local.get 1
              local.get 3
              i32.add
              i32.load8_u
              i32.store8
              block  ;; label = @6
                local.get 4
                i32.const 3
                i32.and
                br_if 0 (;@6;)
                local.get 3
                local.set 2
                br 1 (;@5;)
              end
              local.get 3
              i32.eqz
              br_if 4 (;@1;)
              local.get 0
              local.get 2
              i32.const -4
              i32.add
              local.tee 2
              i32.add
              local.get 1
              local.get 2
              i32.add
              i32.load8_u
              i32.store8
            end
            local.get 2
            i32.const 4
            i32.lt_u
            br_if 0 (;@4;)
            block  ;; label = @5
              local.get 2
              i32.const -4
              i32.add
              local.tee 5
              i32.const 2
              i32.shr_u
              i32.const 1
              i32.add
              i32.const 3
              i32.and
              local.tee 3
              i32.eqz
              br_if 0 (;@5;)
              local.get 1
              i32.const -4
              i32.add
              local.set 4
              local.get 0
              i32.const -4
              i32.add
              local.set 6
              loop  ;; label = @6
                local.get 6
                local.get 2
                i32.add
                local.get 4
                local.get 2
                i32.add
                i32.load
                i32.store
                local.get 2
                i32.const -4
                i32.add
                local.set 2
                local.get 3
                i32.const -1
                i32.add
                local.tee 3
                br_if 0 (;@6;)
              end
            end
            local.get 5
            i32.const 12
            i32.lt_u
            br_if 0 (;@4;)
            local.get 1
            i32.const -16
            i32.add
            local.set 6
            local.get 0
            i32.const -16
            i32.add
            local.set 5
            loop  ;; label = @5
              local.get 5
              local.get 2
              i32.add
              local.tee 3
              i32.const 12
              i32.add
              local.get 6
              local.get 2
              i32.add
              local.tee 4
              i32.const 12
              i32.add
              i32.load
              i32.store
              local.get 3
              i32.const 8
              i32.add
              local.get 4
              i32.const 8
              i32.add
              i32.load
              i32.store
              local.get 3
              i32.const 4
              i32.add
              local.get 4
              i32.const 4
              i32.add
              i32.load
              i32.store
              local.get 3
              local.get 4
              i32.load
              i32.store
              local.get 2
              i32.const -16
              i32.add
              local.tee 2
              i32.const 3
              i32.gt_u
              br_if 0 (;@5;)
            end
          end
          local.get 2
          i32.eqz
          br_if 2 (;@1;)
          local.get 2
          i32.const -1
          i32.add
          local.set 5
          block  ;; label = @4
            local.get 2
            i32.const 3
            i32.and
            local.tee 3
            i32.eqz
            br_if 0 (;@4;)
            local.get 1
            i32.const -1
            i32.add
            local.set 4
            local.get 0
            i32.const -1
            i32.add
            local.set 6
            loop  ;; label = @5
              local.get 6
              local.get 2
              i32.add
              local.get 4
              local.get 2
              i32.add
              i32.load8_u
              i32.store8
              local.get 2
              i32.const -1
              i32.add
              local.set 2
              local.get 3
              i32.const -1
              i32.add
              local.tee 3
              br_if 0 (;@5;)
            end
          end
          local.get 5
          i32.const 3
          i32.lt_u
          br_if 2 (;@1;)
          local.get 1
          i32.const -4
          i32.add
          local.set 4
          local.get 0
          i32.const -4
          i32.add
          local.set 6
          loop  ;; label = @4
            local.get 6
            local.get 2
            i32.add
            local.tee 1
            i32.const 3
            i32.add
            local.get 4
            local.get 2
            i32.add
            local.tee 3
            i32.const 3
            i32.add
            i32.load8_u
            i32.store8
            local.get 1
            i32.const 2
            i32.add
            local.get 3
            i32.const 2
            i32.add
            i32.load8_u
            i32.store8
            local.get 1
            i32.const 1
            i32.add
            local.get 3
            i32.const 1
            i32.add
            i32.load8_u
            i32.store8
            local.get 1
            local.get 3
            i32.load8_u
            i32.store8
            local.get 2
            i32.const -4
            i32.add
            local.tee 2
            br_if 0 (;@4;)
            br 3 (;@1;)
          end
        end
        local.get 4
        i32.const 4
        i32.lt_u
        br_if 0 (;@2;)
        block  ;; label = @3
          local.get 4
          i32.const -4
          i32.add
          local.tee 6
          i32.const 2
          i32.shr_u
          i32.const 1
          i32.add
          i32.const 7
          i32.and
          local.tee 2
          i32.eqz
          br_if 0 (;@3;)
          loop  ;; label = @4
            local.get 3
            local.get 1
            i32.load
            i32.store
            local.get 1
            i32.const 4
            i32.add
            local.set 1
            local.get 3
            i32.const 4
            i32.add
            local.set 3
            local.get 4
            i32.const -4
            i32.add
            local.set 4
            local.get 2
            i32.const -1
            i32.add
            local.tee 2
            br_if 0 (;@4;)
          end
        end
        local.get 6
        i32.const 28
        i32.lt_u
        br_if 0 (;@2;)
        loop  ;; label = @3
          local.get 3
          local.get 1
          i32.load
          i32.store
          local.get 3
          local.get 1
          i32.load offset=4
          i32.store offset=4
          local.get 3
          local.get 1
          i32.load offset=8
          i32.store offset=8
          local.get 3
          local.get 1
          i32.load offset=12
          i32.store offset=12
          local.get 3
          local.get 1
          i32.load offset=16
          i32.store offset=16
          local.get 3
          local.get 1
          i32.load offset=20
          i32.store offset=20
          local.get 3
          local.get 1
          i32.load offset=24
          i32.store offset=24
          local.get 3
          local.get 1
          i32.load offset=28
          i32.store offset=28
          local.get 1
          i32.const 32
          i32.add
          local.set 1
          local.get 3
          i32.const 32
          i32.add
          local.set 3
          local.get 4
          i32.const -32
          i32.add
          local.tee 4
          i32.const 3
          i32.gt_u
          br_if 0 (;@3;)
        end
      end
      local.get 4
      i32.eqz
      br_if 0 (;@1;)
      local.get 4
      i32.const -1
      i32.add
      local.set 6
      block  ;; label = @2
        local.get 4
        i32.const 7
        i32.and
        local.tee 2
        i32.eqz
        br_if 0 (;@2;)
        loop  ;; label = @3
          local.get 3
          local.get 1
          i32.load8_u
          i32.store8
          local.get 4
          i32.const -1
          i32.add
          local.set 4
          local.get 3
          i32.const 1
          i32.add
          local.set 3
          local.get 1
          i32.const 1
          i32.add
          local.set 1
          local.get 2
          i32.const -1
          i32.add
          local.tee 2
          br_if 0 (;@3;)
        end
      end
      local.get 6
      i32.const 7
      i32.lt_u
      br_if 0 (;@1;)
      loop  ;; label = @2
        local.get 3
        local.get 1
        i32.load8_u
        i32.store8
        local.get 3
        local.get 1
        i32.load8_u offset=1
        i32.store8 offset=1
        local.get 3
        local.get 1
        i32.load8_u offset=2
        i32.store8 offset=2
        local.get 3
        local.get 1
        i32.load8_u offset=3
        i32.store8 offset=3
        local.get 3
        local.get 1
        i32.load8_u offset=4
        i32.store8 offset=4
        local.get 3
        local.get 1
        i32.load8_u offset=5
        i32.store8 offset=5
        local.get 3
        local.get 1
        i32.load8_u offset=6
        i32.store8 offset=6
        local.get 3
        local.get 1
        i32.load8_u offset=7
        i32.store8 offset=7
        local.get 3
        i32.const 8
        i32.add
        local.set 3
        local.get 1
        i32.const 8
        i32.add
        local.set 1
        local.get 4
        i32.const -8
        i32.add
        local.tee 4
        br_if 0 (;@2;)
      end
    end
    local.get 0)
  (func $memcmp (type 7) (param i32 i32 i32) (result i32)
    (local i32 i32 i32)
    i32.const 0
    local.set 3
    block  ;; label = @1
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      block  ;; label = @2
        loop  ;; label = @3
          local.get 0
          i32.load8_u
          local.tee 4
          local.get 1
          i32.load8_u
          local.tee 5
          i32.ne
          br_if 1 (;@2;)
          local.get 1
          i32.const 1
          i32.add
          local.set 1
          local.get 0
          i32.const 1
          i32.add
          local.set 0
          local.get 2
          i32.const -1
          i32.add
          local.tee 2
          br_if 0 (;@3;)
          br 2 (;@1;)
        end
      end
      local.get 4
      local.get 5
      i32.sub
      local.set 3
    end
    local.get 3)
  (func $__stpcpy (type 2) (param i32 i32) (result i32)
    (local i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 1
          local.get 0
          i32.xor
          i32.const 3
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 1
          i32.load8_u
          local.set 2
          br 1 (;@2;)
        end
        block  ;; label = @3
          local.get 1
          i32.const 3
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 0
          local.get 1
          i32.load8_u
          local.tee 2
          i32.store8
          block  ;; label = @4
            local.get 2
            br_if 0 (;@4;)
            local.get 0
            return
          end
          local.get 0
          i32.const 1
          i32.add
          local.set 2
          block  ;; label = @4
            local.get 1
            i32.const 1
            i32.add
            local.tee 3
            i32.const 3
            i32.and
            br_if 0 (;@4;)
            local.get 2
            local.set 0
            local.get 3
            local.set 1
            br 1 (;@3;)
          end
          local.get 2
          local.get 3
          i32.load8_u
          local.tee 3
          i32.store8
          local.get 3
          i32.eqz
          br_if 2 (;@1;)
          local.get 0
          i32.const 2
          i32.add
          local.set 2
          block  ;; label = @4
            local.get 1
            i32.const 2
            i32.add
            local.tee 3
            i32.const 3
            i32.and
            br_if 0 (;@4;)
            local.get 2
            local.set 0
            local.get 3
            local.set 1
            br 1 (;@3;)
          end
          local.get 2
          local.get 3
          i32.load8_u
          local.tee 3
          i32.store8
          local.get 3
          i32.eqz
          br_if 2 (;@1;)
          local.get 0
          i32.const 3
          i32.add
          local.set 2
          block  ;; label = @4
            local.get 1
            i32.const 3
            i32.add
            local.tee 3
            i32.const 3
            i32.and
            br_if 0 (;@4;)
            local.get 2
            local.set 0
            local.get 3
            local.set 1
            br 1 (;@3;)
          end
          local.get 2
          local.get 3
          i32.load8_u
          local.tee 3
          i32.store8
          local.get 3
          i32.eqz
          br_if 2 (;@1;)
          local.get 0
          i32.const 4
          i32.add
          local.set 0
          local.get 1
          i32.const 4
          i32.add
          local.set 1
        end
        local.get 1
        i32.load
        local.tee 2
        i32.const -1
        i32.xor
        local.get 2
        i32.const -16843009
        i32.add
        i32.and
        i32.const -2139062144
        i32.and
        br_if 0 (;@2;)
        loop  ;; label = @3
          local.get 0
          local.get 2
          i32.store
          local.get 0
          i32.const 4
          i32.add
          local.set 0
          local.get 1
          i32.const 4
          i32.add
          local.tee 1
          i32.load
          local.tee 2
          i32.const -1
          i32.xor
          local.get 2
          i32.const -16843009
          i32.add
          i32.and
          i32.const -2139062144
          i32.and
          i32.eqz
          br_if 0 (;@3;)
        end
      end
      local.get 0
      local.get 2
      i32.store8
      block  ;; label = @2
        local.get 2
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        local.get 0
        return
      end
      local.get 1
      i32.const 1
      i32.add
      local.set 1
      local.get 0
      local.set 2
      loop  ;; label = @2
        local.get 2
        local.get 1
        i32.load8_u
        local.tee 0
        i32.store8 offset=1
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        local.get 2
        i32.const 1
        i32.add
        local.set 2
        local.get 0
        br_if 0 (;@2;)
      end
    end
    local.get 2)
  (func $strcpy (type 2) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call $__stpcpy
    drop
    local.get 0)
  (func $__strchrnul (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 1
            i32.const 255
            i32.and
            local.tee 2
            i32.eqz
            br_if 0 (;@4;)
            local.get 0
            i32.const 3
            i32.and
            i32.eqz
            br_if 2 (;@2;)
            block  ;; label = @5
              local.get 0
              i32.load8_u
              local.tee 3
              br_if 0 (;@5;)
              local.get 0
              return
            end
            local.get 3
            local.get 1
            i32.const 255
            i32.and
            i32.ne
            br_if 1 (;@3;)
            local.get 0
            return
          end
          local.get 0
          local.get 0
          call $strlen
          i32.add
          return
        end
        block  ;; label = @3
          local.get 0
          i32.const 1
          i32.add
          local.tee 3
          i32.const 3
          i32.and
          br_if 0 (;@3;)
          local.get 3
          local.set 0
          br 1 (;@2;)
        end
        local.get 3
        i32.load8_u
        local.tee 4
        i32.eqz
        br_if 1 (;@1;)
        local.get 4
        local.get 1
        i32.const 255
        i32.and
        i32.eq
        br_if 1 (;@1;)
        block  ;; label = @3
          local.get 0
          i32.const 2
          i32.add
          local.tee 3
          i32.const 3
          i32.and
          br_if 0 (;@3;)
          local.get 3
          local.set 0
          br 1 (;@2;)
        end
        local.get 3
        i32.load8_u
        local.tee 4
        i32.eqz
        br_if 1 (;@1;)
        local.get 4
        local.get 1
        i32.const 255
        i32.and
        i32.eq
        br_if 1 (;@1;)
        block  ;; label = @3
          local.get 0
          i32.const 3
          i32.add
          local.tee 3
          i32.const 3
          i32.and
          br_if 0 (;@3;)
          local.get 3
          local.set 0
          br 1 (;@2;)
        end
        local.get 3
        i32.load8_u
        local.tee 4
        i32.eqz
        br_if 1 (;@1;)
        local.get 4
        local.get 1
        i32.const 255
        i32.and
        i32.eq
        br_if 1 (;@1;)
        local.get 0
        i32.const 4
        i32.add
        local.set 0
      end
      block  ;; label = @2
        local.get 0
        i32.load
        local.tee 3
        i32.const -1
        i32.xor
        local.get 3
        i32.const -16843009
        i32.add
        i32.and
        i32.const -2139062144
        i32.and
        br_if 0 (;@2;)
        local.get 2
        i32.const 16843009
        i32.mul
        local.set 2
        loop  ;; label = @3
          local.get 3
          local.get 2
          i32.xor
          local.tee 3
          i32.const -1
          i32.xor
          local.get 3
          i32.const -16843009
          i32.add
          i32.and
          i32.const -2139062144
          i32.and
          br_if 1 (;@2;)
          local.get 0
          i32.const 4
          i32.add
          local.tee 0
          i32.load
          local.tee 3
          i32.const -1
          i32.xor
          local.get 3
          i32.const -16843009
          i32.add
          i32.and
          i32.const -2139062144
          i32.and
          i32.eqz
          br_if 0 (;@3;)
        end
      end
      local.get 0
      i32.const -1
      i32.add
      local.set 3
      loop  ;; label = @2
        local.get 3
        i32.const 1
        i32.add
        local.tee 3
        i32.load8_u
        local.tee 0
        i32.eqz
        br_if 1 (;@1;)
        local.get 0
        local.get 1
        i32.const 255
        i32.and
        i32.ne
        br_if 0 (;@2;)
      end
    end
    local.get 3)
  (func $strlen (type 4) (param i32) (result i32)
    (local i32 i32)
    local.get 0
    local.set 1
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.const 3
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        local.set 1
        local.get 0
        i32.load8_u
        i32.eqz
        br_if 1 (;@1;)
        local.get 0
        i32.const 1
        i32.add
        local.tee 1
        i32.const 3
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        i32.load8_u
        i32.eqz
        br_if 1 (;@1;)
        local.get 0
        i32.const 2
        i32.add
        local.tee 1
        i32.const 3
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        i32.load8_u
        i32.eqz
        br_if 1 (;@1;)
        local.get 0
        i32.const 3
        i32.add
        local.tee 1
        i32.const 3
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        i32.load8_u
        i32.eqz
        br_if 1 (;@1;)
        local.get 0
        i32.const 4
        i32.add
        local.set 1
      end
      local.get 1
      i32.const -5
      i32.add
      local.set 1
      loop  ;; label = @2
        local.get 1
        i32.const 5
        i32.add
        local.set 2
        local.get 1
        i32.const 4
        i32.add
        local.set 1
        local.get 2
        i32.load
        local.tee 2
        i32.const -1
        i32.xor
        local.get 2
        i32.const -16843009
        i32.add
        i32.and
        i32.const -2139062144
        i32.and
        i32.eqz
        br_if 0 (;@2;)
      end
      loop  ;; label = @2
        local.get 1
        i32.const 1
        i32.add
        local.tee 1
        i32.load8_u
        br_if 0 (;@2;)
      end
    end
    local.get 1
    local.get 0
    i32.sub)
  (func $memcpy (type 7) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 2
          i32.const 32
          i32.gt_u
          br_if 0 (;@3;)
          local.get 1
          i32.const 3
          i32.and
          i32.eqz
          br_if 1 (;@2;)
          local.get 2
          i32.eqz
          br_if 1 (;@2;)
          local.get 0
          local.get 1
          i32.load8_u
          i32.store8
          local.get 2
          i32.const -1
          i32.add
          local.set 3
          local.get 0
          i32.const 1
          i32.add
          local.set 4
          local.get 1
          i32.const 1
          i32.add
          local.tee 5
          i32.const 3
          i32.and
          i32.eqz
          br_if 2 (;@1;)
          local.get 3
          i32.eqz
          br_if 2 (;@1;)
          local.get 0
          local.get 1
          i32.load8_u offset=1
          i32.store8 offset=1
          local.get 2
          i32.const -2
          i32.add
          local.set 3
          local.get 0
          i32.const 2
          i32.add
          local.set 4
          local.get 1
          i32.const 2
          i32.add
          local.tee 5
          i32.const 3
          i32.and
          i32.eqz
          br_if 2 (;@1;)
          local.get 3
          i32.eqz
          br_if 2 (;@1;)
          local.get 0
          local.get 1
          i32.load8_u offset=2
          i32.store8 offset=2
          local.get 2
          i32.const -3
          i32.add
          local.set 3
          local.get 0
          i32.const 3
          i32.add
          local.set 4
          local.get 1
          i32.const 3
          i32.add
          local.tee 5
          i32.const 3
          i32.and
          i32.eqz
          br_if 2 (;@1;)
          local.get 3
          i32.eqz
          br_if 2 (;@1;)
          local.get 0
          local.get 1
          i32.load8_u offset=3
          i32.store8 offset=3
          local.get 2
          i32.const -4
          i32.add
          local.set 3
          local.get 0
          i32.const 4
          i32.add
          local.set 4
          local.get 1
          i32.const 4
          i32.add
          local.set 5
          br 2 (;@1;)
        end
        local.get 0
        local.get 1
        local.get 2
        memory.copy
        local.get 0
        return
      end
      local.get 2
      local.set 3
      local.get 0
      local.set 4
      local.get 1
      local.set 5
    end
    block  ;; label = @1
      block  ;; label = @2
        local.get 4
        i32.const 3
        i32.and
        local.tee 2
        br_if 0 (;@2;)
        block  ;; label = @3
          block  ;; label = @4
            local.get 3
            i32.const 16
            i32.ge_u
            br_if 0 (;@4;)
            local.get 3
            local.set 2
            br 1 (;@3;)
          end
          block  ;; label = @4
            local.get 3
            i32.const -16
            i32.add
            local.tee 2
            i32.const 16
            i32.and
            br_if 0 (;@4;)
            local.get 4
            local.get 5
            i64.load align=4
            i64.store align=4
            local.get 4
            local.get 5
            i64.load offset=8 align=4
            i64.store offset=8 align=4
            local.get 4
            i32.const 16
            i32.add
            local.set 4
            local.get 5
            i32.const 16
            i32.add
            local.set 5
            local.get 2
            local.set 3
          end
          local.get 2
          i32.const 16
          i32.lt_u
          br_if 0 (;@3;)
          local.get 3
          local.set 2
          loop  ;; label = @4
            local.get 4
            local.get 5
            i64.load align=4
            i64.store align=4
            local.get 4
            local.get 5
            i64.load offset=8 align=4
            i64.store offset=8 align=4
            local.get 4
            local.get 5
            i64.load offset=16 align=4
            i64.store offset=16 align=4
            local.get 4
            local.get 5
            i64.load offset=24 align=4
            i64.store offset=24 align=4
            local.get 4
            i32.const 32
            i32.add
            local.set 4
            local.get 5
            i32.const 32
            i32.add
            local.set 5
            local.get 2
            i32.const -32
            i32.add
            local.tee 2
            i32.const 15
            i32.gt_u
            br_if 0 (;@4;)
          end
        end
        block  ;; label = @3
          local.get 2
          i32.const 8
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 4
          local.get 5
          i64.load align=4
          i64.store align=4
          local.get 5
          i32.const 8
          i32.add
          local.set 5
          local.get 4
          i32.const 8
          i32.add
          local.set 4
        end
        block  ;; label = @3
          local.get 2
          i32.const 4
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 4
          local.get 5
          i32.load
          i32.store
          local.get 5
          i32.const 4
          i32.add
          local.set 5
          local.get 4
          i32.const 4
          i32.add
          local.set 4
        end
        block  ;; label = @3
          local.get 2
          i32.const 2
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 4
          local.get 5
          i32.load16_u align=1
          i32.store16 align=1
          local.get 4
          i32.const 2
          i32.add
          local.set 4
          local.get 5
          i32.const 2
          i32.add
          local.set 5
        end
        local.get 2
        i32.const 1
        i32.and
        i32.eqz
        br_if 1 (;@1;)
        local.get 4
        local.get 5
        i32.load8_u
        i32.store8
        local.get 0
        return
      end
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 3
                i32.const 32
                i32.lt_u
                br_if 0 (;@6;)
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 2
                    i32.const -1
                    i32.add
                    br_table 3 (;@5;) 0 (;@8;) 1 (;@7;) 7 (;@1;)
                  end
                  local.get 4
                  local.get 5
                  i32.load
                  i32.store16 align=1
                  local.get 4
                  local.get 5
                  i32.const 2
                  i32.add
                  i32.load align=2
                  i32.store offset=2
                  local.get 4
                  local.get 5
                  i32.const 6
                  i32.add
                  i64.load align=2
                  i64.store offset=6 align=4
                  local.get 4
                  i32.const 18
                  i32.add
                  local.set 2
                  local.get 5
                  i32.const 18
                  i32.add
                  local.set 1
                  i32.const 14
                  local.set 6
                  local.get 5
                  i32.const 14
                  i32.add
                  i32.load align=2
                  local.set 5
                  i32.const 14
                  local.set 3
                  br 3 (;@4;)
                end
                local.get 4
                local.get 5
                i32.load
                i32.store8
                local.get 4
                local.get 5
                i32.const 1
                i32.add
                i32.load align=1
                i32.store offset=1
                local.get 4
                local.get 5
                i32.const 5
                i32.add
                i64.load align=1
                i64.store offset=5 align=4
                local.get 4
                i32.const 17
                i32.add
                local.set 2
                local.get 5
                i32.const 17
                i32.add
                local.set 1
                i32.const 13
                local.set 6
                local.get 5
                i32.const 13
                i32.add
                i32.load align=1
                local.set 5
                i32.const 15
                local.set 3
                br 2 (;@4;)
              end
              block  ;; label = @6
                block  ;; label = @7
                  local.get 3
                  i32.const 16
                  i32.and
                  br_if 0 (;@7;)
                  local.get 4
                  local.set 2
                  local.get 5
                  local.set 1
                  br 1 (;@6;)
                end
                local.get 4
                local.get 5
                i32.load8_u
                i32.store8
                local.get 4
                local.get 5
                i32.load offset=1 align=1
                i32.store offset=1 align=1
                local.get 4
                local.get 5
                i64.load offset=5 align=1
                i64.store offset=5 align=1
                local.get 4
                local.get 5
                i32.load16_u offset=13 align=1
                i32.store16 offset=13 align=1
                local.get 4
                local.get 5
                i32.load8_u offset=15
                i32.store8 offset=15
                local.get 4
                i32.const 16
                i32.add
                local.set 2
                local.get 5
                i32.const 16
                i32.add
                local.set 1
              end
              local.get 3
              i32.const 8
              i32.and
              br_if 2 (;@3;)
              br 3 (;@2;)
            end
            local.get 4
            local.get 5
            i32.load
            local.tee 2
            i32.store8
            local.get 4
            local.get 2
            i32.const 16
            i32.shr_u
            i32.store8 offset=2
            local.get 4
            local.get 2
            i32.const 8
            i32.shr_u
            i32.store8 offset=1
            local.get 4
            local.get 5
            i32.const 3
            i32.add
            i32.load align=1
            i32.store offset=3
            local.get 4
            local.get 5
            i32.const 7
            i32.add
            i64.load align=1
            i64.store offset=7 align=4
            local.get 4
            i32.const 19
            i32.add
            local.set 2
            local.get 5
            i32.const 19
            i32.add
            local.set 1
            i32.const 15
            local.set 6
            local.get 5
            i32.const 15
            i32.add
            i32.load align=1
            local.set 5
            i32.const 13
            local.set 3
          end
          local.get 4
          local.get 6
          i32.add
          local.get 5
          i32.store
        end
        local.get 2
        local.get 1
        i64.load align=1
        i64.store align=1
        local.get 2
        i32.const 8
        i32.add
        local.set 2
        local.get 1
        i32.const 8
        i32.add
        local.set 1
      end
      block  ;; label = @2
        local.get 3
        i32.const 4
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        local.get 1
        i32.load align=1
        i32.store align=1
        local.get 2
        i32.const 4
        i32.add
        local.set 2
        local.get 1
        i32.const 4
        i32.add
        local.set 1
      end
      block  ;; label = @2
        local.get 3
        i32.const 2
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        local.get 1
        i32.load16_u align=1
        i32.store16 align=1
        local.get 2
        i32.const 2
        i32.add
        local.set 2
        local.get 1
        i32.const 2
        i32.add
        local.set 1
      end
      local.get 3
      i32.const 1
      i32.and
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      local.get 1
      i32.load8_u
      i32.store8
    end
    local.get 0)
  (func $memset (type 7) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i64)
    block  ;; label = @1
      local.get 2
      i32.const 33
      i32.lt_u
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      local.get 2
      memory.fill
      local.get 0
      return
    end
    block  ;; label = @1
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      i32.store8
      local.get 2
      local.get 0
      i32.add
      local.tee 3
      i32.const -1
      i32.add
      local.get 1
      i32.store8
      local.get 2
      i32.const 3
      i32.lt_u
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      i32.store8 offset=2
      local.get 0
      local.get 1
      i32.store8 offset=1
      local.get 3
      i32.const -3
      i32.add
      local.get 1
      i32.store8
      local.get 3
      i32.const -2
      i32.add
      local.get 1
      i32.store8
      local.get 2
      i32.const 7
      i32.lt_u
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      i32.store8 offset=3
      local.get 3
      i32.const -4
      i32.add
      local.get 1
      i32.store8
      local.get 2
      i32.const 9
      i32.lt_u
      br_if 0 (;@1;)
      local.get 0
      i32.const 0
      local.get 0
      i32.sub
      i32.const 3
      i32.and
      local.tee 4
      i32.add
      local.tee 5
      local.get 1
      i32.const 255
      i32.and
      i32.const 16843009
      i32.mul
      local.tee 3
      i32.store
      local.get 5
      local.get 2
      local.get 4
      i32.sub
      i32.const -4
      i32.and
      local.tee 1
      i32.add
      local.tee 2
      i32.const -4
      i32.add
      local.get 3
      i32.store
      local.get 1
      i32.const 9
      i32.lt_u
      br_if 0 (;@1;)
      local.get 5
      local.get 3
      i32.store offset=8
      local.get 5
      local.get 3
      i32.store offset=4
      local.get 2
      i32.const -8
      i32.add
      local.get 3
      i32.store
      local.get 2
      i32.const -12
      i32.add
      local.get 3
      i32.store
      local.get 1
      i32.const 25
      i32.lt_u
      br_if 0 (;@1;)
      local.get 5
      local.get 3
      i32.store offset=24
      local.get 5
      local.get 3
      i32.store offset=20
      local.get 5
      local.get 3
      i32.store offset=16
      local.get 5
      local.get 3
      i32.store offset=12
      local.get 2
      i32.const -16
      i32.add
      local.get 3
      i32.store
      local.get 2
      i32.const -20
      i32.add
      local.get 3
      i32.store
      local.get 2
      i32.const -24
      i32.add
      local.get 3
      i32.store
      local.get 2
      i32.const -28
      i32.add
      local.get 3
      i32.store
      local.get 1
      local.get 5
      i32.const 4
      i32.and
      i32.const 24
      i32.or
      local.tee 2
      i32.sub
      local.tee 1
      i32.const 32
      i32.lt_u
      br_if 0 (;@1;)
      local.get 3
      i64.extend_i32_u
      i64.const 4294967297
      i64.mul
      local.set 6
      local.get 5
      local.get 2
      i32.add
      local.set 2
      loop  ;; label = @2
        local.get 2
        local.get 6
        i64.store offset=24
        local.get 2
        local.get 6
        i64.store offset=16
        local.get 2
        local.get 6
        i64.store offset=8
        local.get 2
        local.get 6
        i64.store
        local.get 2
        i32.const 32
        i32.add
        local.set 2
        local.get 1
        i32.const -32
        i32.add
        local.tee 1
        i32.const 31
        i32.gt_u
        br_if 0 (;@2;)
      end
    end
    local.get 0)
  (func $strncmp (type 7) (param i32 i32 i32) (result i32)
    (local i32 i32 i32)
    block  ;; label = @1
      local.get 2
      br_if 0 (;@1;)
      i32.const 0
      return
    end
    i32.const 0
    local.set 3
    block  ;; label = @1
      local.get 0
      i32.load8_u
      local.tee 4
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.const 1
      i32.add
      local.set 0
      local.get 2
      i32.const -1
      i32.add
      local.set 2
      loop  ;; label = @2
        block  ;; label = @3
          local.get 1
          i32.load8_u
          local.tee 5
          br_if 0 (;@3;)
          local.get 4
          local.set 3
          br 2 (;@1;)
        end
        block  ;; label = @3
          local.get 2
          br_if 0 (;@3;)
          local.get 4
          local.set 3
          br 2 (;@1;)
        end
        block  ;; label = @3
          local.get 4
          i32.const 255
          i32.and
          local.get 5
          i32.eq
          br_if 0 (;@3;)
          local.get 4
          local.set 3
          br 2 (;@1;)
        end
        local.get 2
        i32.const -1
        i32.add
        local.set 2
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        local.get 0
        i32.load8_u
        local.set 4
        local.get 0
        i32.const 1
        i32.add
        local.set 0
        local.get 4
        br_if 0 (;@2;)
      end
    end
    local.get 3
    i32.const 255
    i32.and
    local.get 1
    i32.load8_u
    i32.sub)
  (func $strerror (type 4) (param i32) (result i32)
    (local i32)
    block  ;; label = @1
      i32.const 0
      i32.load offset=1059708
      local.tee 1
      br_if 0 (;@1;)
      i32.const 1059684
      local.set 1
      i32.const 0
      i32.const 1059684
      i32.store offset=1059708
    end
    i32.const 0
    local.get 0
    local.get 0
    i32.const 76
    i32.gt_u
    select
    i32.const 1
    i32.shl
    i32.const 1054336
    i32.add
    i32.load16_u
    i32.const 1052769
    i32.add
    local.get 1
    i32.load offset=20
    call $__lctrans)
  (func $strerror_r (type 7) (param i32 i32 i32) (result i32)
    (local i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        call $strerror
        local.tee 0
        call $strlen
        local.tee 3
        local.get 2
        i32.lt_u
        br_if 0 (;@2;)
        i32.const 68
        local.set 3
        local.get 2
        i32.eqz
        br_if 1 (;@1;)
        local.get 1
        local.get 0
        local.get 2
        i32.const -1
        i32.add
        local.tee 2
        call $memcpy
        local.get 2
        i32.add
        i32.const 0
        i32.store8
        i32.const 68
        return
      end
      local.get 1
      local.get 0
      local.get 3
      i32.const 1
      i32.add
      call $memcpy
      drop
      i32.const 0
      local.set 3
    end
    local.get 3)
  (func $strcmp (type 2) (param i32 i32) (result i32)
    (local i32 i32)
    local.get 1
    i32.load8_u
    local.set 2
    block  ;; label = @1
      local.get 0
      i32.load8_u
      local.tee 3
      i32.eqz
      br_if 0 (;@1;)
      local.get 3
      local.get 2
      i32.const 255
      i32.and
      i32.ne
      br_if 0 (;@1;)
      local.get 0
      i32.const 1
      i32.add
      local.set 0
      local.get 1
      i32.const 1
      i32.add
      local.set 1
      loop  ;; label = @2
        local.get 1
        i32.load8_u
        local.set 2
        local.get 0
        i32.load8_u
        local.tee 3
        i32.eqz
        br_if 1 (;@1;)
        local.get 0
        i32.const 1
        i32.add
        local.set 0
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        local.get 3
        local.get 2
        i32.const 255
        i32.and
        i32.eq
        br_if 0 (;@2;)
      end
    end
    local.get 3
    local.get 2
    i32.const 255
    i32.and
    i32.sub)
  (func $dummy.1 (type 2) (param i32 i32) (result i32)
    local.get 0)
  (func $__lctrans (type 2) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call $dummy.1)
  (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17ha4ae67ad4237772aE (type 2) (param i32 i32) (result i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 0
    i32.load
    local.tee 0
    i32.load offset=8
    local.set 3
    local.get 0
    i32.load offset=4
    local.set 0
    local.get 2
    local.get 1
    call $_ZN4core3fmt9Formatter10debug_list17h89c83d2b4d2f51e2E
    block  ;; label = @1
      local.get 3
      i32.eqz
      br_if 0 (;@1;)
      loop  ;; label = @2
        local.get 2
        local.get 0
        i32.store offset=12
        local.get 2
        local.get 2
        i32.const 12
        i32.add
        i32.const 1054516
        call $_ZN4core3fmt8builders8DebugSet5entry17h65c60cb7d5a3b232E
        drop
        local.get 0
        i32.const 1
        i32.add
        local.set 0
        local.get 3
        i32.const -1
        i32.add
        local.tee 3
        br_if 0 (;@2;)
      end
    end
    local.get 2
    call $_ZN4core3fmt8builders9DebugList6finish17h0808958983b26dd3E
    local.set 0
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 0)
  (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hdf42619595b66d37E (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.set 0
    block  ;; label = @1
      local.get 1
      call $_ZN4core3fmt9Formatter15debug_lower_hex17h4338310221a5ab5bE
      br_if 0 (;@1;)
      block  ;; label = @2
        local.get 1
        call $_ZN4core3fmt9Formatter15debug_upper_hex17hba00f3ad3b67e305E
        br_if 0 (;@2;)
        local.get 0
        local.get 1
        call $_ZN4core3fmt3num3imp51_$LT$impl$u20$core..fmt..Display$u20$for$u20$u8$GT$3fmt17h60761d11360ffc45E
        return
      end
      local.get 0
      local.get 1
      call $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i8$GT$3fmt17h0f0b80b3697c42f8E
      return
    end
    local.get 0
    local.get 1
    call $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i8$GT$3fmt17h49c546b8e26ca6fbE)
  (func $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hffbfc641a26cd37cE (type 2) (param i32 i32) (result i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        call $_ZN4core3fmt9Formatter15debug_lower_hex17h4338310221a5ab5bE
        br_if 0 (;@2;)
        local.get 1
        call $_ZN4core3fmt9Formatter15debug_upper_hex17hba00f3ad3b67e305E
        br_if 1 (;@1;)
        local.get 0
        local.get 1
        call $_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17hc287c07e1a56819aE
        return
      end
      local.get 0
      local.get 1
      call $_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i32$GT$3fmt17h67ac2fbea4a9fd5fE
      return
    end
    local.get 0
    local.get 1
    call $_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i32$GT$3fmt17hf603227db3da36cbE)
  (func $_ZN4core3ptr26drop_in_place$LT$usize$GT$17h84e39a189230a888E (type 1) (param i32))
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17hedbee5f483d2714eE (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 1
    call $_ZN5alloc6string6String4push17h938b10e056bfc051E
    i32.const 0)
  (func $_ZN5alloc6string6String4push17h938b10e056bfc051E (type 6) (param i32 i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.const 127
        i32.gt_u
        br_if 0 (;@2;)
        block  ;; label = @3
          local.get 0
          i32.load offset=8
          local.tee 3
          local.get 0
          i32.load
          i32.ne
          br_if 0 (;@3;)
          local.get 0
          local.get 3
          call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17h8ee794783211d5ffE
          local.get 0
          i32.load offset=8
          local.set 3
        end
        local.get 0
        local.get 3
        i32.const 1
        i32.add
        i32.store offset=8
        local.get 0
        i32.load offset=4
        local.get 3
        i32.add
        local.get 1
        i32.store8
        br 1 (;@1;)
      end
      local.get 2
      i32.const 0
      i32.store offset=12
      block  ;; label = @2
        block  ;; label = @3
          local.get 1
          i32.const 2048
          i32.lt_u
          br_if 0 (;@3;)
          block  ;; label = @4
            local.get 1
            i32.const 65536
            i32.lt_u
            br_if 0 (;@4;)
            local.get 2
            local.get 1
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=15
            local.get 2
            local.get 1
            i32.const 6
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=14
            local.get 2
            local.get 1
            i32.const 12
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=13
            local.get 2
            local.get 1
            i32.const 18
            i32.shr_u
            i32.const 7
            i32.and
            i32.const 240
            i32.or
            i32.store8 offset=12
            i32.const 4
            local.set 1
            br 2 (;@2;)
          end
          local.get 2
          local.get 1
          i32.const 63
          i32.and
          i32.const 128
          i32.or
          i32.store8 offset=14
          local.get 2
          local.get 1
          i32.const 12
          i32.shr_u
          i32.const 224
          i32.or
          i32.store8 offset=12
          local.get 2
          local.get 1
          i32.const 6
          i32.shr_u
          i32.const 63
          i32.and
          i32.const 128
          i32.or
          i32.store8 offset=13
          i32.const 3
          local.set 1
          br 1 (;@2;)
        end
        local.get 2
        local.get 1
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=13
        local.get 2
        local.get 1
        i32.const 6
        i32.shr_u
        i32.const 192
        i32.or
        i32.store8 offset=12
        i32.const 2
        local.set 1
      end
      block  ;; label = @2
        local.get 0
        i32.load
        local.get 0
        i32.load offset=8
        local.tee 3
        i32.sub
        local.get 1
        i32.ge_u
        br_if 0 (;@2;)
        local.get 0
        local.get 3
        local.get 1
        call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17had59578487ba7d61E
        local.get 0
        i32.load offset=8
        local.set 3
      end
      local.get 0
      i32.load offset=4
      local.get 3
      i32.add
      local.get 2
      i32.const 12
      i32.add
      local.get 1
      call $memcpy
      drop
      local.get 0
      local.get 3
      local.get 1
      i32.add
      i32.store offset=8
    end
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer)
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17h754307fcaa43f1fbE (type 2) (param i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 0
    i32.load
    local.set 0
    local.get 2
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    local.get 1
    i64.load align=4
    i64.store offset=8
    local.get 2
    local.get 0
    i32.store offset=4
    local.get 2
    i32.const 4
    i32.add
    i32.const 1054492
    local.get 2
    i32.const 8
    i32.add
    call $_ZN4core3fmt5write17h56e014c9dc514616E
    local.set 1
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
    local.get 1)
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17h622aaed9ad907416E (type 7) (param i32 i32 i32) (result i32)
    (local i32)
    block  ;; label = @1
      local.get 0
      i32.load
      local.tee 0
      i32.load
      local.get 0
      i32.load offset=8
      local.tee 3
      i32.sub
      local.get 2
      i32.ge_u
      br_if 0 (;@1;)
      local.get 0
      local.get 3
      local.get 2
      call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17had59578487ba7d61E
      local.get 0
      i32.load offset=8
      local.set 3
    end
    local.get 0
    i32.load offset=4
    local.get 3
    i32.add
    local.get 1
    local.get 2
    call $memcpy
    drop
    local.get 0
    local.get 3
    local.get 2
    i32.add
    i32.store offset=8
    i32.const 0)
  (func $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17had59578487ba7d61E (type 5) (param i32 i32 i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        local.get 2
        i32.add
        local.tee 2
        local.get 1
        i32.lt_u
        br_if 0 (;@2;)
        local.get 0
        i32.load
        local.tee 1
        i32.const 1
        i32.shl
        local.tee 4
        local.get 2
        local.get 4
        local.get 2
        i32.gt_u
        select
        local.tee 2
        i32.const 8
        local.get 2
        i32.const 8
        i32.gt_u
        select
        local.tee 2
        i32.const -1
        i32.xor
        i32.const 31
        i32.shr_u
        local.set 4
        block  ;; label = @3
          block  ;; label = @4
            local.get 1
            i32.eqz
            br_if 0 (;@4;)
            local.get 3
            i32.const 1
            i32.store offset=24
            local.get 3
            local.get 1
            i32.store offset=20
            local.get 3
            local.get 0
            i32.const 4
            i32.add
            i32.load
            i32.store offset=16
            br 1 (;@3;)
          end
          local.get 3
          i32.const 0
          i32.store offset=24
        end
        local.get 3
        local.get 2
        local.get 4
        local.get 3
        i32.const 16
        i32.add
        call $_ZN5alloc7raw_vec11finish_grow17ha39b0a8463e2c9a0E
        local.get 3
        i32.load offset=4
        local.set 1
        block  ;; label = @3
          local.get 3
          i32.load
          br_if 0 (;@3;)
          local.get 0
          local.get 2
          i32.store
          local.get 0
          local.get 1
          i32.store offset=4
          br 2 (;@1;)
        end
        local.get 3
        i32.const 8
        i32.add
        i32.load
        local.tee 0
        i32.const -2147483647
        i32.eq
        br_if 1 (;@1;)
        local.get 0
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        local.get 0
        call $_ZN5alloc5alloc18handle_alloc_error17h4888856d90202140E
        unreachable
      end
      call $_ZN5alloc7raw_vec17capacity_overflow17hbb754ab7ed70e9c1E
      unreachable
    end
    local.get 3
    i32.const 32
    i32.add
    global.set $__stack_pointer)
  (func $_ZN5alloc7raw_vec11finish_grow17ha39b0a8463e2c9a0E (type 11) (param i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        i32.eqz
        br_if 0 (;@2;)
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 1
                    i32.const -1
                    i32.le_s
                    br_if 0 (;@8;)
                    local.get 3
                    i32.load offset=8
                    br_if 1 (;@7;)
                    local.get 1
                    br_if 2 (;@6;)
                    i32.const 1
                    local.set 2
                    br 4 (;@4;)
                  end
                  local.get 0
                  i32.const 8
                  i32.add
                  i32.const 0
                  i32.store
                  br 6 (;@1;)
                end
                block  ;; label = @7
                  local.get 3
                  i32.load offset=4
                  local.tee 2
                  br_if 0 (;@7;)
                  block  ;; label = @8
                    local.get 1
                    br_if 0 (;@8;)
                    i32.const 1
                    local.set 2
                    br 4 (;@4;)
                  end
                  local.get 1
                  i32.const 1
                  call $__rust_alloc
                  local.set 2
                  br 2 (;@5;)
                end
                local.get 3
                i32.load
                local.get 2
                i32.const 1
                local.get 1
                call $__rust_realloc
                local.set 2
                br 1 (;@5;)
              end
              local.get 1
              i32.const 1
              call $__rust_alloc
              local.set 2
            end
            local.get 2
            i32.eqz
            br_if 1 (;@3;)
          end
          local.get 0
          local.get 2
          i32.store offset=4
          local.get 0
          i32.const 8
          i32.add
          local.get 1
          i32.store
          local.get 0
          i32.const 0
          i32.store
          return
        end
        local.get 0
        local.get 1
        i32.store offset=4
        local.get 0
        i32.const 8
        i32.add
        i32.const 1
        i32.store
        local.get 0
        i32.const 1
        i32.store
        return
      end
      local.get 0
      local.get 1
      i32.store offset=4
      local.get 0
      i32.const 8
      i32.add
      i32.const 0
      i32.store
    end
    local.get 0
    i32.const 1
    i32.store)
  (func $_ZN5alloc5alloc18handle_alloc_error17h4888856d90202140E (type 6) (param i32 i32)
    local.get 0
    local.get 1
    call $_ZN5alloc5alloc18handle_alloc_error8rt_error17hee105bb96c1e1ff8E
    unreachable)
  (func $_ZN5alloc7raw_vec17capacity_overflow17hbb754ab7ed70e9c1E (type 0)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 0
    global.set $__stack_pointer
    local.get 0
    i32.const 20
    i32.add
    i32.const 1
    i32.store
    local.get 0
    i32.const 28
    i32.add
    i32.const 0
    i32.store
    local.get 0
    i32.const 1054580
    i32.store offset=16
    local.get 0
    i32.const 1054532
    i32.store offset=24
    local.get 0
    i32.const 0
    i32.store offset=8
    local.get 0
    i32.const 8
    i32.add
    i32.const 1054588
    call $_ZN4core9panicking9panic_fmt17h0724b0a601bd3679E
    unreachable)
  (func $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17h8ee794783211d5ffE (type 6) (param i32 i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.const 1
        i32.add
        local.tee 1
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        i32.load
        local.tee 3
        i32.const 1
        i32.shl
        local.tee 4
        local.get 1
        local.get 4
        local.get 1
        i32.gt_u
        select
        local.tee 1
        i32.const 8
        local.get 1
        i32.const 8
        i32.gt_u
        select
        local.tee 1
        i32.const -1
        i32.xor
        i32.const 31
        i32.shr_u
        local.set 4
        block  ;; label = @3
          block  ;; label = @4
            local.get 3
            i32.eqz
            br_if 0 (;@4;)
            local.get 2
            i32.const 1
            i32.store offset=24
            local.get 2
            local.get 3
            i32.store offset=20
            local.get 2
            local.get 0
            i32.const 4
            i32.add
            i32.load
            i32.store offset=16
            br 1 (;@3;)
          end
          local.get 2
          i32.const 0
          i32.store offset=24
        end
        local.get 2
        local.get 1
        local.get 4
        local.get 2
        i32.const 16
        i32.add
        call $_ZN5alloc7raw_vec11finish_grow17ha39b0a8463e2c9a0E
        local.get 2
        i32.load offset=4
        local.set 3
        block  ;; label = @3
          local.get 2
          i32.load
          br_if 0 (;@3;)
          local.get 0
          local.get 1
          i32.store
          local.get 0
          local.get 3
          i32.store offset=4
          br 2 (;@1;)
        end
        local.get 2
        i32.const 8
        i32.add
        i32.load
        local.tee 0
        i32.const -2147483647
        i32.eq
        br_if 1 (;@1;)
        local.get 0
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        local.get 0
        call $_ZN5alloc5alloc18handle_alloc_error17h4888856d90202140E
        unreachable
      end
      call $_ZN5alloc7raw_vec17capacity_overflow17hbb754ab7ed70e9c1E
      unreachable
    end
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer)
  (func $_ZN5alloc5alloc18handle_alloc_error8rt_error17hee105bb96c1e1ff8E (type 6) (param i32 i32)
    local.get 0
    local.get 1
    call $__rust_alloc_error_handler
    unreachable)
  (func $_ZN223_$LT$$LT$alloc..boxed..Box$LT$dyn$u20$core..error..Error$u2b$core..marker..Send$u2b$core..marker..Sync$GT$$u20$as$u20$core..convert..From$LT$alloc..string..String$GT$$GT$..from..StringError$u20$as$u20$core..error..Error$GT$11description17h0df52edac3281a52E (type 6) (param i32 i32)
    local.get 0
    local.get 1
    i64.load offset=4 align=4
    i64.store)
  (func $_ZN223_$LT$$LT$alloc..boxed..Box$LT$dyn$u20$core..error..Error$u2b$core..marker..Send$u2b$core..marker..Sync$GT$$u20$as$u20$core..convert..From$LT$alloc..string..String$GT$$GT$..from..StringError$u20$as$u20$core..fmt..Display$GT$3fmt17hb6057d54086b2d2cE (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load offset=4
    local.get 0
    i32.load offset=8
    local.get 1
    call $_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17haca8e2a25628fb8aE)
  (func $_ZN221_$LT$$LT$alloc..boxed..Box$LT$dyn$u20$core..error..Error$u2b$core..marker..Send$u2b$core..marker..Sync$GT$$u20$as$u20$core..convert..From$LT$alloc..string..String$GT$$GT$..from..StringError$u20$as$u20$core..fmt..Debug$GT$3fmt17hf29197e7c3e2a133E (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load offset=4
    local.get 0
    i32.load offset=8
    local.get 1
    call $_ZN40_$LT$str$u20$as$u20$core..fmt..Debug$GT$3fmt17hab99d706fe6e9e7bE)
  (func $_ZN72_$LT$$RF$str$u20$as$u20$alloc..ffi..c_str..CString..new..SpecNewImpl$GT$13spec_new_impl17hc2e87543d0ee2275E (type 5) (param i32 i32 i32)
    (local i32 i32 i32 i32 i32 i64)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 2
              i32.const 1
              i32.add
              local.tee 4
              i32.eqz
              br_if 0 (;@5;)
              local.get 4
              i32.const -1
              i32.le_s
              br_if 1 (;@4;)
              local.get 4
              i32.const 1
              call $__rust_alloc
              local.tee 5
              i32.eqz
              br_if 2 (;@3;)
              local.get 5
              local.get 1
              local.get 2
              call $memcpy
              local.set 6
              block  ;; label = @6
                local.get 2
                i32.const 8
                i32.lt_u
                br_if 0 (;@6;)
                local.get 3
                i32.const 8
                i32.add
                i32.const 0
                local.get 1
                local.get 2
                call $_ZN4core5slice6memchr14memchr_aligned17h4ab011c4b696055dE
                local.get 3
                i32.load offset=12
                local.set 7
                local.get 3
                i32.load offset=8
                local.set 5
                br 5 (;@1;)
              end
              block  ;; label = @6
                local.get 2
                br_if 0 (;@6;)
                i32.const 0
                local.set 7
                i32.const 0
                local.set 5
                br 5 (;@1;)
              end
              block  ;; label = @6
                local.get 1
                i32.load8_u
                br_if 0 (;@6;)
                i32.const 1
                local.set 5
                i32.const 0
                local.set 7
                br 5 (;@1;)
              end
              i32.const 1
              local.set 5
              local.get 2
              i32.const 1
              i32.eq
              br_if 3 (;@2;)
              block  ;; label = @6
                local.get 1
                i32.load8_u offset=1
                br_if 0 (;@6;)
                i32.const 1
                local.set 7
                br 5 (;@1;)
              end
              i32.const 2
              local.set 7
              local.get 2
              i32.const 2
              i32.eq
              br_if 3 (;@2;)
              local.get 1
              i32.load8_u offset=2
              i32.eqz
              br_if 4 (;@1;)
              i32.const 3
              local.set 7
              local.get 2
              i32.const 3
              i32.eq
              br_if 3 (;@2;)
              local.get 1
              i32.load8_u offset=3
              i32.eqz
              br_if 4 (;@1;)
              i32.const 4
              local.set 7
              local.get 2
              i32.const 4
              i32.eq
              br_if 3 (;@2;)
              local.get 1
              i32.load8_u offset=4
              i32.eqz
              br_if 4 (;@1;)
              i32.const 5
              local.set 7
              local.get 2
              i32.const 5
              i32.eq
              br_if 3 (;@2;)
              local.get 1
              i32.load8_u offset=5
              i32.eqz
              br_if 4 (;@1;)
              local.get 2
              local.set 7
              i32.const 0
              local.set 5
              local.get 2
              i32.const 6
              i32.eq
              br_if 4 (;@1;)
              local.get 2
              i32.const 6
              local.get 1
              i32.load8_u offset=6
              local.tee 1
              select
              local.set 7
              local.get 1
              i32.eqz
              local.set 5
              br 4 (;@1;)
            end
            i32.const 1054604
            i32.const 43
            i32.const 1054680
            call $_ZN4core9panicking5panic17h33aac59f7456cb9eE
            unreachable
          end
          call $_ZN5alloc7raw_vec17capacity_overflow17hbb754ab7ed70e9c1E
          unreachable
        end
        local.get 4
        i32.const 1
        call $_ZN5alloc5alloc18handle_alloc_error17h4888856d90202140E
        unreachable
      end
      local.get 2
      local.set 7
      i32.const 0
      local.set 5
    end
    block  ;; label = @1
      block  ;; label = @2
        local.get 5
        br_if 0 (;@2;)
        local.get 3
        local.get 2
        i32.store offset=24
        local.get 3
        local.get 6
        i32.store offset=20
        local.get 3
        local.get 4
        i32.store offset=16
        local.get 3
        local.get 3
        i32.const 16
        i32.add
        call $_ZN5alloc3ffi5c_str7CString19_from_vec_unchecked17h983df2e7c2795c96E
        local.get 3
        i64.load
        local.set 8
        local.get 0
        i32.const 0
        i32.store offset=8
        local.get 0
        local.get 8
        i64.store align=4
        br 1 (;@1;)
      end
      local.get 0
      local.get 2
      i32.store offset=12
      local.get 0
      local.get 6
      i32.store offset=8
      local.get 0
      local.get 4
      i32.store offset=4
      local.get 0
      local.get 7
      i32.store
    end
    local.get 3
    i32.const 32
    i32.add
    global.set $__stack_pointer)
  (func $_ZN5alloc3ffi5c_str7CString19_from_vec_unchecked17h983df2e7c2795c96E (type 6) (param i32 i32)
    (local i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 1
              i32.load
              local.tee 3
              local.get 1
              i32.load offset=8
              local.tee 4
              i32.ne
              br_if 0 (;@5;)
              local.get 4
              i32.const 1
              i32.add
              local.tee 3
              i32.eqz
              br_if 2 (;@3;)
              local.get 3
              i32.const -1
              i32.xor
              i32.const 31
              i32.shr_u
              local.set 5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 4
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 2
                  i32.const 1
                  i32.store offset=24
                  local.get 2
                  local.get 4
                  i32.store offset=20
                  local.get 2
                  local.get 1
                  i32.const 4
                  i32.add
                  i32.load
                  i32.store offset=16
                  br 1 (;@6;)
                end
                local.get 2
                i32.const 0
                i32.store offset=24
              end
              local.get 2
              local.get 3
              local.get 5
              local.get 2
              i32.const 16
              i32.add
              call $_ZN5alloc7raw_vec11finish_grow17ha39b0a8463e2c9a0E
              local.get 2
              i32.load offset=4
              local.set 5
              local.get 2
              i32.load
              br_if 1 (;@4;)
              local.get 1
              local.get 3
              i32.store
              local.get 1
              local.get 5
              i32.store offset=4
            end
            local.get 4
            local.get 3
            i32.ne
            br_if 3 (;@1;)
            br 2 (;@2;)
          end
          local.get 2
          i32.const 8
          i32.add
          i32.load
          local.tee 3
          i32.const -2147483647
          i32.eq
          br_if 1 (;@2;)
          local.get 3
          i32.eqz
          br_if 0 (;@3;)
          local.get 5
          local.get 3
          call $_ZN5alloc5alloc18handle_alloc_error17h4888856d90202140E
          unreachable
        end
        call $_ZN5alloc7raw_vec17capacity_overflow17hbb754ab7ed70e9c1E
        unreachable
      end
      local.get 1
      local.get 4
      call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17h8ee794783211d5ffE
      local.get 1
      i32.load
      local.set 3
      local.get 1
      i32.load offset=8
      local.set 4
    end
    local.get 1
    local.get 4
    i32.const 1
    i32.add
    local.tee 5
    i32.store offset=8
    local.get 1
    i32.load offset=4
    local.tee 1
    local.get 4
    i32.add
    i32.const 0
    i32.store8
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 3
          local.get 5
          i32.gt_u
          br_if 0 (;@3;)
          local.get 1
          local.set 4
          br 1 (;@2;)
        end
        block  ;; label = @3
          local.get 5
          br_if 0 (;@3;)
          i32.const 1
          local.set 4
          local.get 1
          local.get 3
          i32.const 1
          call $__rust_dealloc
          br 1 (;@2;)
        end
        local.get 1
        local.get 3
        i32.const 1
        local.get 5
        call $__rust_realloc
        local.tee 4
        i32.eqz
        br_if 1 (;@1;)
      end
      local.get 0
      local.get 5
      i32.store offset=4
      local.get 0
      local.get 4
      i32.store
      local.get 2
      i32.const 32
      i32.add
      global.set $__stack_pointer
      return
    end
    local.get 5
    i32.const 1
    call $_ZN5alloc5alloc18handle_alloc_error17h4888856d90202140E
    unreachable)
  (func $_ZN5alloc3fmt6format12format_inner17hff04a732548263f0E (type 6) (param i32 i32)
    (local i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.const 12
        i32.add
        i32.load
        local.tee 3
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        i32.load offset=8
        local.set 4
        local.get 3
        i32.const -1
        i32.add
        i32.const 536870911
        i32.and
        local.tee 3
        i32.const 1
        i32.add
        local.tee 5
        i32.const 7
        i32.and
        local.set 6
        block  ;; label = @3
          block  ;; label = @4
            local.get 3
            i32.const 7
            i32.ge_u
            br_if 0 (;@4;)
            i32.const 0
            local.set 5
            local.get 4
            local.set 3
            br 1 (;@3;)
          end
          local.get 4
          i32.const 60
          i32.add
          local.set 3
          local.get 5
          i32.const 1073741816
          i32.and
          local.set 7
          i32.const 0
          local.set 5
          loop  ;; label = @4
            local.get 3
            i32.load
            local.get 3
            i32.const -8
            i32.add
            i32.load
            local.get 3
            i32.const -16
            i32.add
            i32.load
            local.get 3
            i32.const -24
            i32.add
            i32.load
            local.get 3
            i32.const -32
            i32.add
            i32.load
            local.get 3
            i32.const -40
            i32.add
            i32.load
            local.get 3
            i32.const -48
            i32.add
            i32.load
            local.get 3
            i32.const -56
            i32.add
            i32.load
            local.get 5
            i32.add
            i32.add
            i32.add
            i32.add
            i32.add
            i32.add
            i32.add
            i32.add
            local.set 5
            local.get 3
            i32.const 64
            i32.add
            local.set 3
            local.get 7
            i32.const -8
            i32.add
            local.tee 7
            br_if 0 (;@4;)
          end
          local.get 3
          i32.const -60
          i32.add
          local.set 3
        end
        block  ;; label = @3
          local.get 6
          i32.eqz
          br_if 0 (;@3;)
          local.get 3
          i32.const 4
          i32.add
          local.set 3
          loop  ;; label = @4
            local.get 3
            i32.load
            local.get 5
            i32.add
            local.set 5
            local.get 3
            i32.const 8
            i32.add
            local.set 3
            local.get 6
            i32.const -1
            i32.add
            local.tee 6
            br_if 0 (;@4;)
          end
        end
        block  ;; label = @3
          local.get 1
          i32.const 20
          i32.add
          i32.load
          i32.eqz
          br_if 0 (;@3;)
          i32.const 1
          local.set 3
          i32.const 0
          local.set 6
          local.get 5
          i32.const 0
          i32.lt_s
          br_if 2 (;@1;)
          local.get 5
          i32.const 16
          i32.lt_u
          local.get 4
          i32.load offset=4
          i32.eqz
          i32.and
          br_if 2 (;@1;)
          local.get 5
          i32.const 1
          i32.shl
          local.set 5
        end
        local.get 5
        i32.eqz
        br_if 0 (;@2;)
        block  ;; label = @3
          block  ;; label = @4
            local.get 5
            i32.const -1
            i32.le_s
            br_if 0 (;@4;)
            local.get 5
            i32.const 1
            call $__rust_alloc
            local.tee 3
            i32.eqz
            br_if 1 (;@3;)
            local.get 5
            local.set 6
            br 3 (;@1;)
          end
          call $_ZN5alloc7raw_vec17capacity_overflow17hbb754ab7ed70e9c1E
          unreachable
        end
        local.get 5
        i32.const 1
        call $_ZN5alloc5alloc18handle_alloc_error17h4888856d90202140E
        unreachable
      end
      i32.const 1
      local.set 3
      i32.const 0
      local.set 6
    end
    local.get 0
    i32.const 0
    i32.store offset=8
    local.get 0
    local.get 3
    i32.store offset=4
    local.get 0
    local.get 6
    i32.store
    local.get 2
    i32.const 16
    i32.add
    i32.const 16
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    i32.const 16
    i32.add
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    local.get 1
    i64.load align=4
    i64.store offset=16
    local.get 2
    local.get 0
    i32.store offset=12
    block  ;; label = @1
      local.get 2
      i32.const 12
      i32.add
      i32.const 1054492
      local.get 2
      i32.const 16
      i32.add
      call $_ZN4core3fmt5write17h56e014c9dc514616E
      i32.eqz
      br_if 0 (;@1;)
      i32.const 1054696
      i32.const 51
      local.get 2
      i32.const 40
      i32.add
      i32.const 1054748
      i32.const 1054788
      call $_ZN4core6result13unwrap_failed17h0cc42b3b30eb09ddE
      unreachable
    end
    local.get 2
    i32.const 48
    i32.add
    global.set $__stack_pointer)
  (func $_ZN5alloc4sync32arcinner_layout_for_value_layout17h85ba5c6b8bd0fdb9E (type 5) (param i32 i32 i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    block  ;; label = @1
      local.get 2
      i32.const 7
      i32.add
      i32.const 0
      local.get 2
      i32.sub
      i32.and
      local.tee 4
      local.get 4
      i32.const -8
      i32.add
      i32.lt_u
      br_if 0 (;@1;)
      local.get 4
      local.get 1
      i32.add
      local.tee 1
      local.get 4
      i32.lt_u
      br_if 0 (;@1;)
      i32.const -2147483648
      local.get 2
      i32.const 4
      local.get 2
      i32.const 4
      i32.gt_u
      select
      local.tee 2
      i32.sub
      local.get 1
      i32.lt_u
      br_if 0 (;@1;)
      local.get 0
      local.get 2
      i32.store offset=4
      local.get 0
      local.get 2
      local.get 1
      i32.add
      i32.const -1
      i32.add
      i32.const 0
      local.get 2
      i32.sub
      i32.and
      i32.store
      local.get 3
      i32.const 16
      i32.add
      global.set $__stack_pointer
      return
    end
    i32.const 1054804
    i32.const 43
    local.get 3
    i32.const 8
    i32.add
    i32.const 1054848
    i32.const 1054892
    call $_ZN4core6result13unwrap_failed17h0cc42b3b30eb09ddE
    unreachable)
  (func $_ZN64_$LT$alloc..ffi..c_str..NulError$u20$as$u20$core..fmt..Debug$GT$3fmt17h2161d80c8a9f6f64E (type 2) (param i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    local.get 0
    i32.const 4
    i32.add
    i32.store offset=12
    local.get 1
    i32.const 1054908
    i32.const 8
    local.get 0
    i32.const 1054916
    local.get 2
    i32.const 12
    i32.add
    i32.const 1054932
    call $_ZN4core3fmt9Formatter25debug_tuple_field2_finish17h6b7a77d622adf9f6E
    local.set 0
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 0)
  (func $_ZN4core3ops8function6FnOnce9call_once17haeae9a1c0a16fac0E (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    drop
    loop (result i32)  ;; label = @1
      br 0 (;@1;)
    end)
  (func $_ZN4core3ptr102drop_in_place$LT$$RF$core..iter..adapters..copied..Copied$LT$core..slice..iter..Iter$LT$u8$GT$$GT$$GT$17hf914f599202dabbdE (type 1) (param i32))
  (func $_ZN4core9panicking9panic_fmt17h0724b0a601bd3679E (type 6) (param i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    i32.const 1
    i32.store8 offset=24
    local.get 2
    local.get 1
    i32.store offset=20
    local.get 2
    local.get 0
    i32.store offset=16
    local.get 2
    i32.const 1055164
    i32.store offset=12
    local.get 2
    i32.const 1054992
    i32.store offset=8
    local.get 2
    i32.const 8
    i32.add
    call $rust_begin_unwind
    unreachable)
  (func $_ZN4core9panicking18panic_bounds_check17h575df7fe5ecb957dE (type 5) (param i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    local.get 1
    i32.store offset=4
    local.get 3
    local.get 0
    i32.store
    local.get 3
    i32.const 8
    i32.add
    i32.const 12
    i32.add
    i32.const 2
    i32.store
    local.get 3
    i32.const 28
    i32.add
    i32.const 2
    i32.store
    local.get 3
    i32.const 32
    i32.add
    i32.const 12
    i32.add
    i32.const 11
    i32.store
    local.get 3
    i32.const 1055232
    i32.store offset=16
    local.get 3
    i32.const 0
    i32.store offset=8
    local.get 3
    i32.const 11
    i32.store offset=36
    local.get 3
    local.get 3
    i32.const 32
    i32.add
    i32.store offset=24
    local.get 3
    local.get 3
    i32.store offset=40
    local.get 3
    local.get 3
    i32.const 4
    i32.add
    i32.store offset=32
    local.get 3
    i32.const 8
    i32.add
    local.get 2
    call $_ZN4core9panicking9panic_fmt17h0724b0a601bd3679E
    unreachable)
  (func $_ZN4core5slice5index26slice_start_index_len_fail17h12ea13b84dc5aa33E (type 5) (param i32 i32 i32)
    local.get 0
    local.get 1
    local.get 2
    call $_ZN4core5slice5index29slice_start_index_len_fail_rt17h597176734be4f636E
    unreachable)
  (func $_ZN4core5slice5index24slice_end_index_len_fail17h6cad56aacf5f762dE (type 5) (param i32 i32 i32)
    local.get 0
    local.get 1
    local.get 2
    call $_ZN4core5slice5index27slice_end_index_len_fail_rt17hd780f3b79b77f071E
    unreachable)
  (func $_ZN4core3fmt9Formatter3pad17hea5c16334677f526E (type 7) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load offset=8
        local.tee 3
        local.get 0
        i32.load offset=16
        local.tee 4
        i32.or
        i32.eqz
        br_if 0 (;@2;)
        block  ;; label = @3
          local.get 4
          i32.eqz
          br_if 0 (;@3;)
          local.get 1
          local.get 2
          i32.add
          local.set 5
          local.get 0
          i32.const 20
          i32.add
          i32.load
          i32.const 1
          i32.add
          local.set 6
          i32.const 0
          local.set 7
          local.get 1
          local.set 8
          block  ;; label = @4
            loop  ;; label = @5
              local.get 8
              local.set 4
              local.get 6
              i32.const -1
              i32.add
              local.tee 6
              i32.eqz
              br_if 1 (;@4;)
              local.get 4
              local.get 5
              i32.eq
              br_if 2 (;@3;)
              block  ;; label = @6
                block  ;; label = @7
                  local.get 4
                  i32.load8_s
                  local.tee 9
                  i32.const -1
                  i32.le_s
                  br_if 0 (;@7;)
                  local.get 4
                  i32.const 1
                  i32.add
                  local.set 8
                  local.get 9
                  i32.const 255
                  i32.and
                  local.set 9
                  br 1 (;@6;)
                end
                local.get 4
                i32.load8_u offset=1
                i32.const 63
                i32.and
                local.set 10
                local.get 9
                i32.const 31
                i32.and
                local.set 8
                block  ;; label = @7
                  local.get 9
                  i32.const -33
                  i32.gt_u
                  br_if 0 (;@7;)
                  local.get 8
                  i32.const 6
                  i32.shl
                  local.get 10
                  i32.or
                  local.set 9
                  local.get 4
                  i32.const 2
                  i32.add
                  local.set 8
                  br 1 (;@6;)
                end
                local.get 10
                i32.const 6
                i32.shl
                local.get 4
                i32.load8_u offset=2
                i32.const 63
                i32.and
                i32.or
                local.set 10
                block  ;; label = @7
                  local.get 9
                  i32.const -16
                  i32.ge_u
                  br_if 0 (;@7;)
                  local.get 10
                  local.get 8
                  i32.const 12
                  i32.shl
                  i32.or
                  local.set 9
                  local.get 4
                  i32.const 3
                  i32.add
                  local.set 8
                  br 1 (;@6;)
                end
                local.get 10
                i32.const 6
                i32.shl
                local.get 4
                i32.load8_u offset=3
                i32.const 63
                i32.and
                i32.or
                local.get 8
                i32.const 18
                i32.shl
                i32.const 1835008
                i32.and
                i32.or
                local.tee 9
                i32.const 1114112
                i32.eq
                br_if 3 (;@3;)
                local.get 4
                i32.const 4
                i32.add
                local.set 8
              end
              local.get 7
              local.get 4
              i32.sub
              local.get 8
              i32.add
              local.set 7
              local.get 9
              i32.const 1114112
              i32.ne
              br_if 0 (;@5;)
              br 2 (;@3;)
            end
          end
          local.get 4
          local.get 5
          i32.eq
          br_if 0 (;@3;)
          block  ;; label = @4
            local.get 4
            i32.load8_s
            local.tee 8
            i32.const -1
            i32.gt_s
            br_if 0 (;@4;)
            local.get 8
            i32.const -32
            i32.lt_u
            br_if 0 (;@4;)
            local.get 8
            i32.const -16
            i32.lt_u
            br_if 0 (;@4;)
            local.get 4
            i32.load8_u offset=2
            i32.const 63
            i32.and
            i32.const 6
            i32.shl
            local.get 4
            i32.load8_u offset=1
            i32.const 63
            i32.and
            i32.const 12
            i32.shl
            i32.or
            local.get 4
            i32.load8_u offset=3
            i32.const 63
            i32.and
            i32.or
            local.get 8
            i32.const 255
            i32.and
            i32.const 18
            i32.shl
            i32.const 1835008
            i32.and
            i32.or
            i32.const 1114112
            i32.eq
            br_if 1 (;@3;)
          end
          block  ;; label = @4
            block  ;; label = @5
              local.get 7
              i32.eqz
              br_if 0 (;@5;)
              block  ;; label = @6
                local.get 7
                local.get 2
                i32.lt_u
                br_if 0 (;@6;)
                i32.const 0
                local.set 4
                local.get 7
                local.get 2
                i32.eq
                br_if 1 (;@5;)
                br 2 (;@4;)
              end
              i32.const 0
              local.set 4
              local.get 1
              local.get 7
              i32.add
              i32.load8_s
              i32.const -64
              i32.lt_s
              br_if 1 (;@4;)
            end
            local.get 1
            local.set 4
          end
          local.get 7
          local.get 2
          local.get 4
          select
          local.set 2
          local.get 4
          local.get 1
          local.get 4
          select
          local.set 1
        end
        block  ;; label = @3
          local.get 3
          br_if 0 (;@3;)
          local.get 0
          i32.load
          local.get 1
          local.get 2
          local.get 0
          i32.load offset=4
          i32.load offset=12
          call_indirect (type 7)
          return
        end
        local.get 0
        i32.const 12
        i32.add
        i32.load
        local.set 7
        block  ;; label = @3
          block  ;; label = @4
            local.get 2
            i32.const 16
            i32.lt_u
            br_if 0 (;@4;)
            local.get 1
            local.get 2
            call $_ZN4core3str5count14do_count_chars17h9ebe54268b64ed3aE
            local.set 8
            br 1 (;@3;)
          end
          block  ;; label = @4
            local.get 2
            br_if 0 (;@4;)
            i32.const 0
            local.set 8
            br 1 (;@3;)
          end
          local.get 2
          i32.const 3
          i32.and
          local.set 9
          block  ;; label = @4
            block  ;; label = @5
              local.get 2
              i32.const 4
              i32.ge_u
              br_if 0 (;@5;)
              i32.const 0
              local.set 8
              local.get 1
              local.set 4
              br 1 (;@4;)
            end
            local.get 2
            i32.const -4
            i32.and
            local.set 6
            i32.const 0
            local.set 8
            local.get 1
            local.set 4
            loop  ;; label = @5
              local.get 8
              local.get 4
              i32.load8_s
              i32.const -65
              i32.gt_s
              i32.add
              local.get 4
              i32.load8_s offset=1
              i32.const -65
              i32.gt_s
              i32.add
              local.get 4
              i32.load8_s offset=2
              i32.const -65
              i32.gt_s
              i32.add
              local.get 4
              i32.load8_s offset=3
              i32.const -65
              i32.gt_s
              i32.add
              local.set 8
              local.get 4
              i32.const 4
              i32.add
              local.set 4
              local.get 6
              i32.const -4
              i32.add
              local.tee 6
              br_if 0 (;@5;)
            end
          end
          local.get 9
          i32.eqz
          br_if 0 (;@3;)
          loop  ;; label = @4
            local.get 8
            local.get 4
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.set 8
            local.get 4
            i32.const 1
            i32.add
            local.set 4
            local.get 9
            i32.const -1
            i32.add
            local.tee 9
            br_if 0 (;@4;)
          end
        end
        block  ;; label = @3
          local.get 7
          local.get 8
          i32.le_u
          br_if 0 (;@3;)
          local.get 7
          local.get 8
          i32.sub
          local.tee 8
          local.set 7
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                i32.const 0
                local.get 0
                i32.load8_u offset=32
                local.tee 4
                local.get 4
                i32.const 3
                i32.eq
                select
                local.tee 4
                br_table 2 (;@4;) 0 (;@6;) 1 (;@5;) 2 (;@4;)
              end
              i32.const 0
              local.set 7
              local.get 8
              local.set 4
              br 1 (;@4;)
            end
            local.get 8
            i32.const 1
            i32.shr_u
            local.set 4
            local.get 8
            i32.const 1
            i32.add
            i32.const 1
            i32.shr_u
            local.set 7
          end
          local.get 4
          i32.const 1
          i32.add
          local.set 4
          local.get 0
          i32.const 4
          i32.add
          i32.load
          local.set 9
          local.get 0
          i32.load offset=28
          local.set 8
          local.get 0
          i32.load
          local.set 6
          block  ;; label = @4
            loop  ;; label = @5
              local.get 4
              i32.const -1
              i32.add
              local.tee 4
              i32.eqz
              br_if 1 (;@4;)
              local.get 6
              local.get 8
              local.get 9
              i32.load offset=16
              call_indirect (type 2)
              i32.eqz
              br_if 0 (;@5;)
            end
            i32.const 1
            return
          end
          i32.const 1
          local.set 4
          local.get 8
          i32.const 1114112
          i32.eq
          br_if 2 (;@1;)
          local.get 6
          local.get 1
          local.get 2
          local.get 9
          i32.load offset=12
          call_indirect (type 7)
          br_if 2 (;@1;)
          i32.const 0
          local.set 4
          loop  ;; label = @4
            block  ;; label = @5
              local.get 7
              local.get 4
              i32.ne
              br_if 0 (;@5;)
              local.get 7
              local.get 7
              i32.lt_u
              return
            end
            local.get 4
            i32.const 1
            i32.add
            local.set 4
            local.get 6
            local.get 8
            local.get 9
            i32.load offset=16
            call_indirect (type 2)
            i32.eqz
            br_if 0 (;@4;)
          end
          local.get 4
          i32.const -1
          i32.add
          local.get 7
          i32.lt_u
          return
        end
        local.get 0
        i32.load
        local.get 1
        local.get 2
        local.get 0
        i32.load offset=4
        i32.load offset=12
        call_indirect (type 7)
        return
      end
      local.get 0
      i32.load
      local.get 1
      local.get 2
      local.get 0
      i32.load offset=4
      i32.load offset=12
      call_indirect (type 7)
      local.set 4
    end
    local.get 4)
  (func $_ZN4core9panicking5panic17h33aac59f7456cb9eE (type 5) (param i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    i32.const 12
    i32.add
    i32.const 1
    i32.store
    local.get 3
    i32.const 20
    i32.add
    i32.const 0
    i32.store
    local.get 3
    i32.const 1054992
    i32.store offset=16
    local.get 3
    i32.const 0
    i32.store
    local.get 3
    local.get 1
    i32.store offset=28
    local.get 3
    local.get 0
    i32.store offset=24
    local.get 3
    local.get 3
    i32.const 24
    i32.add
    i32.store offset=8
    local.get 3
    local.get 2
    call $_ZN4core9panicking9panic_fmt17h0724b0a601bd3679E
    unreachable)
  (func $_ZN4core5slice5index22slice_index_order_fail17hb44181e9b62b1c0cE (type 5) (param i32 i32 i32)
    local.get 0
    local.get 1
    local.get 2
    call $_ZN4core5slice5index25slice_index_order_fail_rt17h2fa58c5f8613fe67E
    unreachable)
  (func $_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17hc287c07e1a56819aE (type 2) (param i32 i32) (result i32)
    local.get 0
    i64.load32_u
    i32.const 1
    local.get 1
    call $_ZN4core3fmt3num3imp7fmt_u6417he8d38148f6fbdea6E)
  (func $_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u32$GT$3fmt17h8b3eea51f609ced3E (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 128
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 1
              i32.load offset=24
              local.tee 3
              i32.const 16
              i32.and
              br_if 0 (;@5;)
              local.get 3
              i32.const 32
              i32.and
              br_if 1 (;@4;)
              local.get 0
              i64.load32_u
              i32.const 1
              local.get 1
              call $_ZN4core3fmt3num3imp7fmt_u6417he8d38148f6fbdea6E
              local.set 0
              br 4 (;@1;)
            end
            local.get 0
            i32.load
            local.set 0
            i32.const 0
            local.set 3
            loop  ;; label = @5
              local.get 2
              local.get 3
              i32.add
              i32.const 127
              i32.add
              i32.const 48
              i32.const 87
              local.get 0
              i32.const 15
              i32.and
              local.tee 4
              i32.const 10
              i32.lt_u
              select
              local.get 4
              i32.add
              i32.store8
              local.get 3
              i32.const -1
              i32.add
              local.set 3
              local.get 0
              i32.const 15
              i32.gt_u
              local.set 4
              local.get 0
              i32.const 4
              i32.shr_u
              local.set 0
              local.get 4
              br_if 0 (;@5;)
            end
            local.get 3
            i32.const 128
            i32.add
            local.tee 0
            i32.const 129
            i32.ge_u
            br_if 1 (;@3;)
            local.get 1
            i32.const 1
            i32.const 1055516
            i32.const 2
            local.get 2
            local.get 3
            i32.add
            i32.const 128
            i32.add
            i32.const 0
            local.get 3
            i32.sub
            call $_ZN4core3fmt9Formatter12pad_integral17h2602ad621a1201d6E
            local.set 0
            br 3 (;@1;)
          end
          local.get 0
          i32.load
          local.set 0
          i32.const 0
          local.set 3
          loop  ;; label = @4
            local.get 2
            local.get 3
            i32.add
            i32.const 127
            i32.add
            i32.const 48
            i32.const 55
            local.get 0
            i32.const 15
            i32.and
            local.tee 4
            i32.const 10
            i32.lt_u
            select
            local.get 4
            i32.add
            i32.store8
            local.get 3
            i32.const -1
            i32.add
            local.set 3
            local.get 0
            i32.const 15
            i32.gt_u
            local.set 4
            local.get 0
            i32.const 4
            i32.shr_u
            local.set 0
            local.get 4
            br_if 0 (;@4;)
          end
          local.get 3
          i32.const 128
          i32.add
          local.tee 0
          i32.const 129
          i32.ge_u
          br_if 1 (;@2;)
          local.get 1
          i32.const 1
          i32.const 1055516
          i32.const 2
          local.get 2
          local.get 3
          i32.add
          i32.const 128
          i32.add
          i32.const 0
          local.get 3
          i32.sub
          call $_ZN4core3fmt9Formatter12pad_integral17h2602ad621a1201d6E
          local.set 0
          br 2 (;@1;)
        end
        local.get 0
        i32.const 128
        i32.const 1055500
        call $_ZN4core5slice5index26slice_start_index_len_fail17h12ea13b84dc5aa33E
        unreachable
      end
      local.get 0
      i32.const 128
      i32.const 1055500
      call $_ZN4core5slice5index26slice_start_index_len_fail17h12ea13b84dc5aa33E
      unreachable
    end
    local.get 2
    i32.const 128
    i32.add
    global.set $__stack_pointer
    local.get 0)
  (func $_ZN4core3fmt5write17h56e014c9dc514616E (type 7) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    i32.const 3
    i32.store8 offset=40
    local.get 3
    i64.const 137438953472
    i64.store offset=32
    i32.const 0
    local.set 4
    local.get 3
    i32.const 0
    i32.store offset=24
    local.get 3
    i32.const 0
    i32.store offset=16
    local.get 3
    local.get 1
    i32.store offset=12
    local.get 3
    local.get 0
    i32.store offset=8
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 2
            i32.load
            local.tee 5
            br_if 0 (;@4;)
            local.get 2
            i32.const 20
            i32.add
            i32.load
            local.tee 0
            i32.eqz
            br_if 1 (;@3;)
            local.get 2
            i32.load offset=16
            local.set 1
            local.get 0
            i32.const 3
            i32.shl
            local.set 6
            local.get 0
            i32.const -1
            i32.add
            i32.const 536870911
            i32.and
            i32.const 1
            i32.add
            local.set 4
            local.get 2
            i32.load offset=8
            local.set 0
            loop  ;; label = @5
              block  ;; label = @6
                local.get 0
                i32.const 4
                i32.add
                i32.load
                local.tee 7
                i32.eqz
                br_if 0 (;@6;)
                local.get 3
                i32.load offset=8
                local.get 0
                i32.load
                local.get 7
                local.get 3
                i32.load offset=12
                i32.load offset=12
                call_indirect (type 7)
                br_if 4 (;@2;)
              end
              local.get 1
              i32.load
              local.get 3
              i32.const 8
              i32.add
              local.get 1
              i32.const 4
              i32.add
              i32.load
              call_indirect (type 2)
              br_if 3 (;@2;)
              local.get 1
              i32.const 8
              i32.add
              local.set 1
              local.get 0
              i32.const 8
              i32.add
              local.set 0
              local.get 6
              i32.const -8
              i32.add
              local.tee 6
              br_if 0 (;@5;)
              br 2 (;@3;)
            end
          end
          local.get 2
          i32.load offset=4
          local.tee 1
          i32.eqz
          br_if 0 (;@3;)
          local.get 1
          i32.const 5
          i32.shl
          local.set 8
          local.get 1
          i32.const -1
          i32.add
          i32.const 134217727
          i32.and
          i32.const 1
          i32.add
          local.set 4
          local.get 2
          i32.load offset=8
          local.set 0
          i32.const 0
          local.set 6
          loop  ;; label = @4
            block  ;; label = @5
              local.get 0
              i32.const 4
              i32.add
              i32.load
              local.tee 1
              i32.eqz
              br_if 0 (;@5;)
              local.get 3
              i32.load offset=8
              local.get 0
              i32.load
              local.get 1
              local.get 3
              i32.load offset=12
              i32.load offset=12
              call_indirect (type 7)
              br_if 3 (;@2;)
            end
            local.get 3
            local.get 5
            local.get 6
            i32.add
            local.tee 1
            i32.const 28
            i32.add
            i32.load8_u
            i32.store8 offset=40
            local.get 3
            local.get 1
            i32.const 20
            i32.add
            i64.load align=4
            i64.store offset=32
            local.get 1
            i32.const 16
            i32.add
            i32.load
            local.set 9
            local.get 2
            i32.load offset=16
            local.set 10
            i32.const 0
            local.set 11
            i32.const 0
            local.set 7
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 1
                  i32.const 12
                  i32.add
                  i32.load
                  br_table 1 (;@6;) 0 (;@7;) 2 (;@5;) 1 (;@6;)
                end
                local.get 9
                i32.const 3
                i32.shl
                local.set 12
                i32.const 0
                local.set 7
                local.get 10
                local.get 12
                i32.add
                local.tee 12
                i32.const 4
                i32.add
                i32.load
                i32.const 83
                i32.ne
                br_if 1 (;@5;)
                local.get 12
                i32.load
                i32.load
                local.set 9
              end
              i32.const 1
              local.set 7
            end
            local.get 3
            local.get 9
            i32.store offset=20
            local.get 3
            local.get 7
            i32.store offset=16
            local.get 1
            i32.const 8
            i32.add
            i32.load
            local.set 7
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 1
                  i32.const 4
                  i32.add
                  i32.load
                  br_table 1 (;@6;) 0 (;@7;) 2 (;@5;) 1 (;@6;)
                end
                local.get 7
                i32.const 3
                i32.shl
                local.set 9
                local.get 10
                local.get 9
                i32.add
                local.tee 9
                i32.const 4
                i32.add
                i32.load
                i32.const 83
                i32.ne
                br_if 1 (;@5;)
                local.get 9
                i32.load
                i32.load
                local.set 7
              end
              i32.const 1
              local.set 11
            end
            local.get 3
            local.get 7
            i32.store offset=28
            local.get 3
            local.get 11
            i32.store offset=24
            local.get 10
            local.get 1
            i32.load
            i32.const 3
            i32.shl
            i32.add
            local.tee 1
            i32.load
            local.get 3
            i32.const 8
            i32.add
            local.get 1
            i32.load offset=4
            call_indirect (type 2)
            br_if 2 (;@2;)
            local.get 0
            i32.const 8
            i32.add
            local.set 0
            local.get 8
            local.get 6
            i32.const 32
            i32.add
            local.tee 6
            i32.ne
            br_if 0 (;@4;)
          end
        end
        block  ;; label = @3
          local.get 4
          local.get 2
          i32.const 12
          i32.add
          i32.load
          i32.ge_u
          br_if 0 (;@3;)
          local.get 3
          i32.load offset=8
          local.get 2
          i32.load offset=8
          local.get 4
          i32.const 3
          i32.shl
          i32.add
          local.tee 1
          i32.load
          local.get 1
          i32.load offset=4
          local.get 3
          i32.load offset=12
          i32.load offset=12
          call_indirect (type 7)
          br_if 1 (;@2;)
        end
        i32.const 0
        local.set 1
        br 1 (;@1;)
      end
      i32.const 1
      local.set 1
    end
    local.get 3
    i32.const 48
    i32.add
    global.set $__stack_pointer
    local.get 1)
  (func $_ZN4core3fmt9Formatter12pad_integral17h2602ad621a1201d6E (type 15) (param i32 i32 i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.eqz
        br_if 0 (;@2;)
        i32.const 43
        i32.const 1114112
        local.get 0
        i32.load offset=24
        local.tee 6
        i32.const 1
        i32.and
        local.tee 1
        select
        local.set 7
        local.get 1
        local.get 5
        i32.add
        local.set 8
        br 1 (;@1;)
      end
      local.get 5
      i32.const 1
      i32.add
      local.set 8
      local.get 0
      i32.load offset=24
      local.set 6
      i32.const 45
      local.set 7
    end
    block  ;; label = @1
      block  ;; label = @2
        local.get 6
        i32.const 4
        i32.and
        br_if 0 (;@2;)
        i32.const 0
        local.set 2
        br 1 (;@1;)
      end
      block  ;; label = @2
        block  ;; label = @3
          local.get 3
          i32.const 16
          i32.lt_u
          br_if 0 (;@3;)
          local.get 2
          local.get 3
          call $_ZN4core3str5count14do_count_chars17h9ebe54268b64ed3aE
          local.set 9
          br 1 (;@2;)
        end
        block  ;; label = @3
          local.get 3
          br_if 0 (;@3;)
          i32.const 0
          local.set 9
          br 1 (;@2;)
        end
        local.get 3
        i32.const 3
        i32.and
        local.set 10
        block  ;; label = @3
          block  ;; label = @4
            local.get 3
            i32.const 4
            i32.ge_u
            br_if 0 (;@4;)
            i32.const 0
            local.set 9
            local.get 2
            local.set 1
            br 1 (;@3;)
          end
          local.get 3
          i32.const -4
          i32.and
          local.set 11
          i32.const 0
          local.set 9
          local.get 2
          local.set 1
          loop  ;; label = @4
            local.get 9
            local.get 1
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.get 1
            i32.load8_s offset=1
            i32.const -65
            i32.gt_s
            i32.add
            local.get 1
            i32.load8_s offset=2
            i32.const -65
            i32.gt_s
            i32.add
            local.get 1
            i32.load8_s offset=3
            i32.const -65
            i32.gt_s
            i32.add
            local.set 9
            local.get 1
            i32.const 4
            i32.add
            local.set 1
            local.get 11
            i32.const -4
            i32.add
            local.tee 11
            br_if 0 (;@4;)
          end
        end
        local.get 10
        i32.eqz
        br_if 0 (;@2;)
        loop  ;; label = @3
          local.get 9
          local.get 1
          i32.load8_s
          i32.const -65
          i32.gt_s
          i32.add
          local.set 9
          local.get 1
          i32.const 1
          i32.add
          local.set 1
          local.get 10
          i32.const -1
          i32.add
          local.tee 10
          br_if 0 (;@3;)
        end
      end
      local.get 9
      local.get 8
      i32.add
      local.set 8
    end
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load offset=8
        br_if 0 (;@2;)
        i32.const 1
        local.set 1
        local.get 0
        i32.load
        local.tee 9
        local.get 0
        i32.const 4
        i32.add
        i32.load
        local.tee 0
        local.get 7
        local.get 2
        local.get 3
        call $_ZN4core3fmt9Formatter12pad_integral12write_prefix17h19c5e647cdfac30eE
        br_if 1 (;@1;)
        local.get 9
        local.get 4
        local.get 5
        local.get 0
        i32.load offset=12
        call_indirect (type 7)
        return
      end
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 0
                i32.const 12
                i32.add
                i32.load
                local.tee 11
                local.get 8
                i32.le_u
                br_if 0 (;@6;)
                local.get 6
                i32.const 8
                i32.and
                br_if 4 (;@2;)
                local.get 11
                local.get 8
                i32.sub
                local.tee 9
                local.set 11
                i32.const 1
                local.get 0
                i32.load8_u offset=32
                local.tee 1
                local.get 1
                i32.const 3
                i32.eq
                select
                local.tee 1
                br_table 3 (;@3;) 1 (;@5;) 2 (;@4;) 3 (;@3;)
              end
              i32.const 1
              local.set 1
              local.get 0
              i32.load
              local.tee 9
              local.get 0
              i32.const 4
              i32.add
              i32.load
              local.tee 0
              local.get 7
              local.get 2
              local.get 3
              call $_ZN4core3fmt9Formatter12pad_integral12write_prefix17h19c5e647cdfac30eE
              br_if 4 (;@1;)
              local.get 9
              local.get 4
              local.get 5
              local.get 0
              i32.load offset=12
              call_indirect (type 7)
              return
            end
            i32.const 0
            local.set 11
            local.get 9
            local.set 1
            br 1 (;@3;)
          end
          local.get 9
          i32.const 1
          i32.shr_u
          local.set 1
          local.get 9
          i32.const 1
          i32.add
          i32.const 1
          i32.shr_u
          local.set 11
        end
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        local.get 0
        i32.const 4
        i32.add
        i32.load
        local.set 10
        local.get 0
        i32.load offset=28
        local.set 9
        local.get 0
        i32.load
        local.set 0
        block  ;; label = @3
          loop  ;; label = @4
            local.get 1
            i32.const -1
            i32.add
            local.tee 1
            i32.eqz
            br_if 1 (;@3;)
            local.get 0
            local.get 9
            local.get 10
            i32.load offset=16
            call_indirect (type 2)
            i32.eqz
            br_if 0 (;@4;)
          end
          i32.const 1
          return
        end
        i32.const 1
        local.set 1
        local.get 9
        i32.const 1114112
        i32.eq
        br_if 1 (;@1;)
        local.get 0
        local.get 10
        local.get 7
        local.get 2
        local.get 3
        call $_ZN4core3fmt9Formatter12pad_integral12write_prefix17h19c5e647cdfac30eE
        br_if 1 (;@1;)
        local.get 0
        local.get 4
        local.get 5
        local.get 10
        i32.load offset=12
        call_indirect (type 7)
        br_if 1 (;@1;)
        i32.const 0
        local.set 1
        block  ;; label = @3
          loop  ;; label = @4
            block  ;; label = @5
              local.get 11
              local.get 1
              i32.ne
              br_if 0 (;@5;)
              local.get 11
              local.set 1
              br 2 (;@3;)
            end
            local.get 1
            i32.const 1
            i32.add
            local.set 1
            local.get 0
            local.get 9
            local.get 10
            i32.load offset=16
            call_indirect (type 2)
            i32.eqz
            br_if 0 (;@4;)
          end
          local.get 1
          i32.const -1
          i32.add
          local.set 1
        end
        local.get 1
        local.get 11
        i32.lt_u
        local.set 1
        br 1 (;@1;)
      end
      local.get 0
      i32.load offset=28
      local.set 6
      local.get 0
      i32.const 48
      i32.store offset=28
      local.get 0
      i32.load8_u offset=32
      local.set 12
      i32.const 1
      local.set 1
      local.get 0
      i32.const 1
      i32.store8 offset=32
      local.get 0
      i32.load
      local.tee 9
      local.get 0
      i32.const 4
      i32.add
      i32.load
      local.tee 10
      local.get 7
      local.get 2
      local.get 3
      call $_ZN4core3fmt9Formatter12pad_integral12write_prefix17h19c5e647cdfac30eE
      br_if 0 (;@1;)
      local.get 11
      local.get 8
      i32.sub
      i32.const 1
      i32.add
      local.set 1
      block  ;; label = @2
        loop  ;; label = @3
          local.get 1
          i32.const -1
          i32.add
          local.tee 1
          i32.eqz
          br_if 1 (;@2;)
          local.get 9
          i32.const 48
          local.get 10
          i32.load offset=16
          call_indirect (type 2)
          i32.eqz
          br_if 0 (;@3;)
        end
        i32.const 1
        return
      end
      i32.const 1
      local.set 1
      local.get 9
      local.get 4
      local.get 5
      local.get 10
      i32.load offset=12
      call_indirect (type 7)
      br_if 0 (;@1;)
      local.get 0
      local.get 12
      i32.store8 offset=32
      local.get 0
      local.get 6
      i32.store offset=28
      i32.const 0
      return
    end
    local.get 1)
  (func $_ZN71_$LT$core..ops..range..Range$LT$Idx$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17h4234e500297b1dd2E (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    i32.const 1
    local.set 3
    block  ;; label = @1
      local.get 0
      local.get 1
      call $_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u32$GT$3fmt17h8b3eea51f609ced3E
      br_if 0 (;@1;)
      local.get 1
      i32.load offset=4
      local.set 4
      local.get 1
      i32.load
      local.set 5
      local.get 2
      i32.const 0
      i32.store offset=28
      local.get 2
      i32.const 1054992
      i32.store offset=24
      i32.const 1
      local.set 3
      local.get 2
      i32.const 1
      i32.store offset=20
      local.get 2
      i32.const 1054996
      i32.store offset=16
      local.get 2
      i32.const 0
      i32.store offset=8
      local.get 5
      local.get 4
      local.get 2
      i32.const 8
      i32.add
      call $_ZN4core3fmt5write17h56e014c9dc514616E
      br_if 0 (;@1;)
      local.get 0
      i32.const 4
      i32.add
      local.get 1
      call $_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u32$GT$3fmt17h8b3eea51f609ced3E
      local.set 3
    end
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
    local.get 3)
  (func $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h961caf76bf45b94bE (type 3) (param i32) (result i64)
    i64.const -5213601185211584148)
  (func $_ZN63_$LT$core..cell..BorrowMutError$u20$as$u20$core..fmt..Debug$GT$3fmt17h3ea277d26af0c621E (type 2) (param i32 i32) (result i32)
    local.get 1
    i32.load
    i32.const 1055064
    i32.const 14
    local.get 1
    i32.load offset=4
    i32.load offset=12
    call_indirect (type 7))
  (func $_ZN59_$LT$core..ffi..c_str..CStr$u20$as$u20$core..fmt..Debug$GT$3fmt17hdf615f64c1fe01a9E (type 7) (param i32 i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 64
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    i32.const 84
    i32.store offset=12
    local.get 3
    i32.const 0
    i32.store8 offset=31
    local.get 3
    i32.const 0
    i32.store8 offset=24
    local.get 3
    local.get 0
    i32.store offset=20
    local.get 3
    local.get 1
    local.get 0
    i32.add
    i32.const -1
    i32.add
    i32.store offset=16
    local.get 3
    local.get 3
    i32.const 16
    i32.add
    i32.store offset=8
    local.get 2
    i32.load
    local.set 0
    local.get 2
    i32.load offset=4
    local.set 2
    local.get 3
    i32.const 1
    i32.store offset=60
    local.get 3
    i32.const 2
    i32.store offset=52
    local.get 3
    i32.const 1055080
    i32.store offset=48
    local.get 3
    i32.const 0
    i32.store offset=40
    local.get 3
    local.get 3
    i32.const 8
    i32.add
    i32.store offset=56
    local.get 0
    local.get 2
    local.get 3
    i32.const 40
    i32.add
    call $_ZN4core3fmt5write17h56e014c9dc514616E
    local.set 0
    local.get 3
    i32.const 64
    i32.add
    global.set $__stack_pointer
    local.get 0)
  (func $_ZN70_$LT$core..slice..ascii..EscapeAscii$u20$as$u20$core..fmt..Display$GT$3fmt17h5411013031d90505E (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 0
    i32.const 9
    i32.add
    i32.load align=1
    local.set 3
    local.get 0
    i32.const 13
    i32.add
    local.set 4
    local.get 0
    i32.const 14
    i32.add
    i32.load8_u
    local.set 5
    local.get 0
    i32.const 4
    i32.add
    local.set 6
    local.get 0
    i32.load8_u offset=8
    local.set 7
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load8_u offset=15
        local.tee 8
        br_if 0 (;@2;)
        i32.const 0
        local.set 9
        br 1 (;@1;)
      end
      local.get 0
      i32.const 16
      i32.add
      i32.load
      local.set 10
      local.get 0
      i32.const 21
      i32.add
      i32.load8_u
      local.set 11
      local.get 0
      i32.const 20
      i32.add
      i32.load8_u
      local.set 12
      i32.const 1
      local.set 9
    end
    local.get 4
    i32.load8_u
    local.set 4
    local.get 6
    i32.load
    local.set 6
    local.get 0
    i32.load
    local.set 13
    local.get 2
    local.get 11
    i32.store8 offset=29
    local.get 2
    local.get 12
    i32.store8 offset=28
    local.get 2
    local.get 10
    i32.store offset=24
    local.get 2
    local.get 9
    i32.store8 offset=23
    local.get 2
    local.get 5
    i32.store8 offset=22
    local.get 2
    local.get 3
    i32.store offset=17 align=1
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 7
              i32.const 255
              i32.and
              i32.eqz
              br_if 0 (;@5;)
              local.get 2
              i32.const 8
              i32.add
              i32.const 9
              i32.add
              local.set 10
              local.get 4
              i32.const 255
              i32.and
              local.tee 0
              i32.const 4
              local.get 0
              i32.const 4
              i32.gt_u
              select
              local.set 9
              local.get 5
              i32.const 255
              i32.and
              local.tee 3
              local.get 0
              local.get 3
              local.get 0
              i32.gt_u
              select
              local.set 7
              local.get 1
              i32.load offset=4
              local.set 14
              local.get 1
              i32.load
              local.set 15
              loop  ;; label = @6
                local.get 7
                local.get 0
                i32.eq
                br_if 1 (;@5;)
                local.get 2
                local.get 0
                i32.const 1
                i32.add
                local.tee 3
                i32.store8 offset=21
                local.get 9
                local.get 0
                i32.eq
                br_if 5 (;@1;)
                local.get 10
                local.get 0
                i32.add
                local.set 4
                local.get 3
                local.set 0
                local.get 15
                local.get 4
                i32.load8_u
                local.get 14
                i32.load offset=16
                call_indirect (type 2)
                i32.eqz
                br_if 0 (;@6;)
              end
              i32.const 1
              local.set 15
              local.get 3
              i32.const -1
              i32.add
              local.get 5
              i32.const 255
              i32.and
              i32.lt_u
              br_if 1 (;@4;)
            end
            block  ;; label = @5
              local.get 6
              i32.eqz
              br_if 0 (;@5;)
              local.get 6
              local.get 13
              i32.eq
              br_if 0 (;@5;)
              local.get 1
              i32.load
              local.set 10
              local.get 1
              i32.load offset=4
              i32.load offset=16
              local.set 14
              loop  ;; label = @6
                i32.const 2
                local.set 7
                i32.const 0
                local.set 4
                i32.const 29788
                local.set 0
                i32.const 1
                local.set 3
                i32.const 0
                local.set 9
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              block  ;; label = @14
                                block  ;; label = @15
                                  block  ;; label = @16
                                    local.get 6
                                    i32.load8_u
                                    local.tee 15
                                    i32.const -9
                                    i32.add
                                    br_table 9 (;@7;) 3 (;@13;) 1 (;@15;) 1 (;@15;) 2 (;@14;) 1 (;@15;) 1 (;@15;) 1 (;@15;) 1 (;@15;) 1 (;@15;) 1 (;@15;) 1 (;@15;) 1 (;@15;) 1 (;@15;) 1 (;@15;) 1 (;@15;) 1 (;@15;) 1 (;@15;) 1 (;@15;) 1 (;@15;) 1 (;@15;) 1 (;@15;) 1 (;@15;) 1 (;@15;) 1 (;@15;) 6 (;@10;) 1 (;@15;) 1 (;@15;) 1 (;@15;) 1 (;@15;) 5 (;@11;) 0 (;@16;)
                                  end
                                  local.get 15
                                  i32.const 92
                                  i32.eq
                                  br_if 3 (;@12;)
                                end
                                local.get 15
                                i32.const -32
                                i32.add
                                i32.const 255
                                i32.and
                                i32.const 95
                                i32.ge_u
                                br_if 5 (;@9;)
                                i32.const 1
                                local.set 4
                                i32.const 0
                                local.set 3
                                local.get 15
                                local.set 0
                                i32.const 0
                                local.set 9
                                i32.const 1
                                local.set 7
                                br 7 (;@7;)
                              end
                              i32.const 29276
                              local.set 0
                              br 5 (;@8;)
                            end
                            i32.const 28252
                            local.set 0
                            br 4 (;@8;)
                          end
                          i32.const 23644
                          local.set 0
                          br 3 (;@8;)
                        end
                        i32.const 10076
                        local.set 0
                        br 2 (;@8;)
                      end
                      i32.const 8796
                      local.set 0
                      br 1 (;@8;)
                    end
                    i32.const 4
                    local.set 7
                    local.get 15
                    i32.const 4
                    i32.shr_u
                    i32.const 1055004
                    i32.add
                    i32.load8_u
                    i32.const 16
                    i32.shl
                    local.get 15
                    i32.const 15
                    i32.and
                    i32.const 1055004
                    i32.add
                    i32.load8_u
                    i32.const 24
                    i32.shl
                    i32.or
                    i32.const 30812
                    i32.or
                    local.set 0
                    i32.const 1
                    local.set 9
                    i32.const 0
                    local.set 4
                    i32.const 0
                    local.set 3
                    br 1 (;@7;)
                  end
                  i32.const 1
                  local.set 3
                  i32.const 0
                  local.set 9
                end
                i32.const 1
                local.set 15
                local.get 10
                local.get 0
                i32.const 255
                i32.and
                local.get 14
                call_indirect (type 2)
                br_if 2 (;@4;)
                block  ;; label = @7
                  local.get 4
                  br_if 0 (;@7;)
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 3
                      local.get 10
                      local.get 0
                      i32.const 8
                      i32.shr_u
                      i32.const 255
                      i32.and
                      local.get 14
                      call_indirect (type 2)
                      local.tee 4
                      i32.or
                      i32.eqz
                      br_if 0 (;@9;)
                      i32.const 1
                      local.get 7
                      local.get 4
                      select
                      local.set 3
                      br 1 (;@8;)
                    end
                    i32.const 2
                    local.set 3
                    local.get 10
                    local.get 0
                    i32.const 16
                    i32.shr_u
                    i32.const 255
                    i32.and
                    local.get 14
                    call_indirect (type 2)
                    br_if 0 (;@8;)
                    local.get 9
                    local.get 10
                    local.get 0
                    i32.const 24
                    i32.shr_u
                    local.get 14
                    call_indirect (type 2)
                    local.tee 0
                    i32.or
                    i32.const 1
                    i32.ne
                    br_if 5 (;@3;)
                    i32.const 3
                    local.get 7
                    local.get 0
                    select
                    local.set 3
                  end
                  local.get 3
                  local.get 7
                  i32.lt_u
                  br_if 3 (;@4;)
                end
                local.get 6
                i32.const 1
                i32.add
                local.tee 6
                local.get 13
                i32.ne
                br_if 0 (;@6;)
              end
            end
            block  ;; label = @5
              local.get 8
              i32.eqz
              br_if 0 (;@5;)
              local.get 2
              i32.const 24
              i32.add
              local.set 9
              local.get 12
              i32.const 255
              i32.and
              local.tee 0
              i32.const 4
              local.get 0
              i32.const 4
              i32.gt_u
              select
              local.set 7
              local.get 11
              i32.const 255
              i32.and
              local.tee 3
              local.get 0
              local.get 3
              local.get 0
              i32.gt_u
              select
              local.set 6
              local.get 1
              i32.load offset=4
              local.set 10
              local.get 1
              i32.load
              local.set 14
              loop  ;; label = @6
                local.get 6
                local.get 0
                i32.eq
                br_if 1 (;@5;)
                local.get 2
                local.get 0
                i32.const 1
                i32.add
                local.tee 3
                i32.store8 offset=28
                local.get 7
                local.get 0
                i32.eq
                br_if 4 (;@2;)
                local.get 9
                local.get 0
                i32.add
                local.set 4
                local.get 3
                local.set 0
                local.get 14
                local.get 4
                i32.load8_u
                local.get 10
                i32.load offset=16
                call_indirect (type 2)
                i32.eqz
                br_if 0 (;@6;)
              end
              i32.const 1
              local.set 15
              local.get 3
              i32.const -1
              i32.add
              local.get 11
              i32.const 255
              i32.and
              i32.lt_u
              br_if 1 (;@4;)
            end
            i32.const 0
            local.set 15
          end
          local.get 2
          i32.const 32
          i32.add
          global.set $__stack_pointer
          local.get 15
          return
        end
        i32.const 4
        i32.const 4
        i32.const 1055048
        call $_ZN4core9panicking18panic_bounds_check17h575df7fe5ecb957dE
        unreachable
      end
      local.get 7
      i32.const 4
      i32.const 1055048
      call $_ZN4core9panicking18panic_bounds_check17h575df7fe5ecb957dE
      unreachable
    end
    local.get 9
    i32.const 4
    i32.const 1055048
    call $_ZN4core9panicking18panic_bounds_check17h575df7fe5ecb957dE
    unreachable)
  (func $_ZN4core3ffi5c_str4CStr8from_ptr9strlen_rt17h15b37b953b28d4f9E (type 4) (param i32) (result i32)
    local.get 0
    call $strlen)
  (func $_ZN4core5slice6memchr14memchr_aligned17h4ab011c4b696055dE (type 11) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 2
                i32.const 3
                i32.add
                i32.const -4
                i32.and
                local.tee 4
                local.get 2
                i32.eq
                br_if 0 (;@6;)
                local.get 4
                local.get 2
                i32.sub
                local.tee 4
                local.get 3
                local.get 4
                local.get 3
                i32.lt_u
                select
                local.tee 4
                i32.eqz
                br_if 0 (;@6;)
                i32.const 0
                local.set 5
                local.get 1
                i32.const 255
                i32.and
                local.set 6
                i32.const 1
                local.set 7
                loop  ;; label = @7
                  local.get 2
                  local.get 5
                  i32.add
                  i32.load8_u
                  local.get 6
                  i32.eq
                  br_if 6 (;@1;)
                  local.get 4
                  local.get 5
                  i32.const 1
                  i32.add
                  local.tee 5
                  i32.ne
                  br_if 0 (;@7;)
                end
                local.get 4
                local.get 3
                i32.const -8
                i32.add
                local.tee 8
                i32.gt_u
                br_if 2 (;@4;)
                br 1 (;@5;)
              end
              local.get 3
              i32.const -8
              i32.add
              local.set 8
              i32.const 0
              local.set 4
            end
            local.get 1
            i32.const 255
            i32.and
            i32.const 16843009
            i32.mul
            local.set 5
            block  ;; label = @5
              loop  ;; label = @6
                local.get 2
                local.get 4
                i32.add
                local.tee 7
                i32.load
                local.get 5
                i32.xor
                local.tee 6
                i32.const -1
                i32.xor
                local.get 6
                i32.const -16843009
                i32.add
                i32.and
                i32.const -2139062144
                i32.and
                br_if 1 (;@5;)
                local.get 7
                i32.const 4
                i32.add
                i32.load
                local.get 5
                i32.xor
                local.tee 6
                i32.const -1
                i32.xor
                local.get 6
                i32.const -16843009
                i32.add
                i32.and
                i32.const -2139062144
                i32.and
                br_if 1 (;@5;)
                local.get 4
                i32.const 8
                i32.add
                local.tee 4
                local.get 8
                i32.le_u
                br_if 0 (;@6;)
              end
            end
            local.get 4
            local.get 3
            i32.gt_u
            br_if 1 (;@3;)
          end
          i32.const 0
          local.set 7
          local.get 4
          local.get 3
          i32.eq
          br_if 1 (;@2;)
          local.get 1
          i32.const 255
          i32.and
          local.set 5
          loop  ;; label = @4
            block  ;; label = @5
              local.get 2
              local.get 4
              i32.add
              i32.load8_u
              local.get 5
              i32.ne
              br_if 0 (;@5;)
              local.get 4
              local.set 5
              i32.const 1
              local.set 7
              br 4 (;@1;)
            end
            local.get 3
            local.get 4
            i32.const 1
            i32.add
            local.tee 4
            i32.eq
            br_if 2 (;@2;)
            br 0 (;@4;)
          end
        end
        local.get 4
        local.get 3
        i32.const 1055844
        call $_ZN4core5slice5index26slice_start_index_len_fail17h12ea13b84dc5aa33E
        unreachable
      end
      local.get 3
      local.set 5
    end
    local.get 0
    local.get 5
    i32.store offset=4
    local.get 0
    local.get 7
    i32.store)
  (func $_ZN4core3ffi5c_str4CStr19from_bytes_with_nul17h2353c14c48c58dfeE (type 5) (param i32 i32 i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        i32.const 8
        i32.lt_u
        br_if 0 (;@2;)
        local.get 3
        i32.const 8
        i32.add
        i32.const 0
        local.get 1
        local.get 2
        call $_ZN4core5slice6memchr14memchr_aligned17h4ab011c4b696055dE
        local.get 3
        i32.load offset=12
        local.set 4
        local.get 3
        i32.load offset=8
        local.set 5
        br 1 (;@1;)
      end
      block  ;; label = @2
        local.get 2
        br_if 0 (;@2;)
        i32.const 0
        local.set 4
        i32.const 0
        local.set 5
        br 1 (;@1;)
      end
      block  ;; label = @2
        local.get 1
        i32.load8_u
        br_if 0 (;@2;)
        i32.const 1
        local.set 5
        i32.const 0
        local.set 4
        br 1 (;@1;)
      end
      i32.const 1
      local.set 5
      block  ;; label = @2
        local.get 2
        i32.const 1
        i32.eq
        br_if 0 (;@2;)
        block  ;; label = @3
          local.get 1
          i32.load8_u offset=1
          br_if 0 (;@3;)
          i32.const 1
          local.set 4
          br 2 (;@1;)
        end
        i32.const 2
        local.set 4
        local.get 2
        i32.const 2
        i32.eq
        br_if 0 (;@2;)
        local.get 1
        i32.load8_u offset=2
        i32.eqz
        br_if 1 (;@1;)
        i32.const 3
        local.set 4
        local.get 2
        i32.const 3
        i32.eq
        br_if 0 (;@2;)
        local.get 1
        i32.load8_u offset=3
        i32.eqz
        br_if 1 (;@1;)
        i32.const 4
        local.set 4
        local.get 2
        i32.const 4
        i32.eq
        br_if 0 (;@2;)
        local.get 1
        i32.load8_u offset=4
        i32.eqz
        br_if 1 (;@1;)
        i32.const 5
        local.set 4
        local.get 2
        i32.const 5
        i32.eq
        br_if 0 (;@2;)
        local.get 1
        i32.load8_u offset=5
        i32.eqz
        br_if 1 (;@1;)
        local.get 2
        local.set 4
        i32.const 0
        local.set 5
        local.get 2
        i32.const 6
        i32.eq
        br_if 1 (;@1;)
        local.get 2
        i32.const 6
        local.get 1
        i32.load8_u offset=6
        local.tee 5
        select
        local.set 4
        local.get 5
        i32.eqz
        local.set 5
        br 1 (;@1;)
      end
      local.get 2
      local.set 4
      i32.const 0
      local.set 5
    end
    block  ;; label = @1
      block  ;; label = @2
        local.get 5
        br_if 0 (;@2;)
        i32.const 1
        local.set 5
        local.get 0
        i32.const 1
        i32.store offset=4
        br 1 (;@1;)
      end
      i32.const 1
      local.set 5
      block  ;; label = @2
        local.get 4
        i32.const 1
        i32.add
        local.get 2
        i32.eq
        br_if 0 (;@2;)
        local.get 0
        i32.const 0
        i32.store offset=4
        local.get 0
        i32.const 8
        i32.add
        local.get 4
        i32.store
        br 1 (;@1;)
      end
      local.get 0
      local.get 1
      i32.store offset=4
      local.get 0
      i32.const 8
      i32.add
      local.get 2
      i32.store
      i32.const 0
      local.set 5
    end
    local.get 0
    local.get 5
    i32.store
    local.get 3
    i32.const 16
    i32.add
    global.set $__stack_pointer)
  (func $_ZN4core3str8converts9from_utf817he9b4783fc054fc5dE (type 5) (param i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32)
    block  ;; label = @1
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      i32.const 0
      local.get 2
      i32.const -7
      i32.add
      local.tee 3
      local.get 3
      local.get 2
      i32.gt_u
      select
      local.set 4
      local.get 1
      i32.const 3
      i32.add
      i32.const -4
      i32.and
      local.get 1
      i32.sub
      local.tee 5
      i32.const -1
      i32.eq
      local.set 6
      i32.const 0
      local.set 3
      loop  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            local.get 1
                            local.get 3
                            i32.add
                            i32.load8_u
                            local.tee 7
                            i32.extend8_s
                            local.tee 8
                            i32.const 0
                            i32.lt_s
                            br_if 0 (;@12;)
                            local.get 6
                            br_if 1 (;@11;)
                            local.get 5
                            local.get 3
                            i32.sub
                            i32.const 3
                            i32.and
                            br_if 1 (;@11;)
                            local.get 3
                            local.get 4
                            i32.lt_u
                            br_if 2 (;@10;)
                            br 8 (;@4;)
                          end
                          i32.const 1
                          local.set 9
                          i32.const 1
                          local.set 10
                          block  ;; label = @12
                            block  ;; label = @13
                              block  ;; label = @14
                                block  ;; label = @15
                                  block  ;; label = @16
                                    block  ;; label = @17
                                      block  ;; label = @18
                                        block  ;; label = @19
                                          local.get 7
                                          i32.const 1056012
                                          i32.add
                                          i32.load8_u
                                          i32.const -2
                                          i32.add
                                          br_table 0 (;@19;) 1 (;@18;) 2 (;@17;) 14 (;@5;)
                                        end
                                        local.get 3
                                        i32.const 1
                                        i32.add
                                        local.tee 7
                                        local.get 2
                                        i32.lt_u
                                        br_if 6 (;@12;)
                                        i32.const 0
                                        local.set 10
                                        br 13 (;@5;)
                                      end
                                      i32.const 1
                                      local.set 9
                                      i32.const 0
                                      local.set 10
                                      local.get 3
                                      i32.const 1
                                      i32.add
                                      local.tee 11
                                      local.get 2
                                      i32.ge_u
                                      br_if 12 (;@5;)
                                      local.get 1
                                      local.get 11
                                      i32.add
                                      i32.load8_s
                                      local.set 11
                                      local.get 7
                                      i32.const -224
                                      i32.add
                                      br_table 1 (;@16;) 3 (;@14;) 3 (;@14;) 3 (;@14;) 3 (;@14;) 3 (;@14;) 3 (;@14;) 3 (;@14;) 3 (;@14;) 3 (;@14;) 3 (;@14;) 3 (;@14;) 3 (;@14;) 2 (;@15;) 3 (;@14;)
                                    end
                                    i32.const 1
                                    local.set 9
                                    block  ;; label = @17
                                      local.get 3
                                      i32.const 1
                                      i32.add
                                      local.tee 10
                                      local.get 2
                                      i32.lt_u
                                      br_if 0 (;@17;)
                                      i32.const 0
                                      local.set 10
                                      br 12 (;@5;)
                                    end
                                    local.get 1
                                    local.get 10
                                    i32.add
                                    i32.load8_s
                                    local.set 11
                                    block  ;; label = @17
                                      block  ;; label = @18
                                        block  ;; label = @19
                                          local.get 7
                                          i32.const -240
                                          i32.add
                                          br_table 1 (;@18;) 0 (;@19;) 0 (;@19;) 0 (;@19;) 2 (;@17;) 0 (;@19;)
                                        end
                                        i32.const 1
                                        local.set 9
                                        local.get 8
                                        i32.const 15
                                        i32.add
                                        i32.const 255
                                        i32.and
                                        i32.const 2
                                        i32.le_u
                                        br_if 9 (;@9;)
                                        i32.const 1
                                        local.set 10
                                        br 13 (;@5;)
                                      end
                                      local.get 11
                                      i32.const 112
                                      i32.add
                                      i32.const 255
                                      i32.and
                                      i32.const 48
                                      i32.lt_u
                                      br_if 9 (;@8;)
                                      br 11 (;@6;)
                                    end
                                    local.get 11
                                    i32.const -113
                                    i32.gt_s
                                    br_if 10 (;@6;)
                                    br 8 (;@8;)
                                  end
                                  local.get 11
                                  i32.const -32
                                  i32.and
                                  i32.const -96
                                  i32.ne
                                  br_if 9 (;@6;)
                                  br 2 (;@13;)
                                end
                                local.get 11
                                i32.const -96
                                i32.ge_s
                                br_if 8 (;@6;)
                                br 1 (;@13;)
                              end
                              block  ;; label = @14
                                block  ;; label = @15
                                  local.get 8
                                  i32.const 31
                                  i32.add
                                  i32.const 255
                                  i32.and
                                  i32.const 12
                                  i32.lt_u
                                  br_if 0 (;@15;)
                                  i32.const 1
                                  local.set 9
                                  local.get 8
                                  i32.const -2
                                  i32.and
                                  i32.const -18
                                  i32.eq
                                  br_if 1 (;@14;)
                                  i32.const 1
                                  local.set 10
                                  br 10 (;@5;)
                                end
                                local.get 11
                                i32.const -65
                                i32.gt_s
                                br_if 8 (;@6;)
                                br 1 (;@13;)
                              end
                              i32.const 1
                              local.set 10
                              local.get 11
                              i32.const -64
                              i32.ge_s
                              br_if 8 (;@5;)
                            end
                            i32.const 0
                            local.set 10
                            local.get 3
                            i32.const 2
                            i32.add
                            local.tee 7
                            local.get 2
                            i32.ge_u
                            br_if 7 (;@5;)
                            local.get 1
                            local.get 7
                            i32.add
                            i32.load8_s
                            i32.const -65
                            i32.le_s
                            br_if 5 (;@7;)
                            i32.const 1
                            local.set 10
                            i32.const 2
                            local.set 9
                            br 7 (;@5;)
                          end
                          local.get 1
                          local.get 7
                          i32.add
                          i32.load8_s
                          i32.const -65
                          i32.gt_s
                          br_if 5 (;@6;)
                          br 4 (;@7;)
                        end
                        local.get 3
                        i32.const 1
                        i32.add
                        local.set 3
                        br 7 (;@3;)
                      end
                      loop  ;; label = @10
                        local.get 1
                        local.get 3
                        i32.add
                        local.tee 7
                        i32.load
                        i32.const -2139062144
                        i32.and
                        br_if 6 (;@4;)
                        local.get 7
                        i32.const 4
                        i32.add
                        i32.load
                        i32.const -2139062144
                        i32.and
                        br_if 6 (;@4;)
                        local.get 3
                        i32.const 8
                        i32.add
                        local.tee 3
                        local.get 4
                        i32.ge_u
                        br_if 6 (;@4;)
                        br 0 (;@10;)
                      end
                    end
                    i32.const 1
                    local.set 10
                    local.get 11
                    i32.const -64
                    i32.ge_s
                    br_if 3 (;@5;)
                  end
                  block  ;; label = @8
                    local.get 3
                    i32.const 2
                    i32.add
                    local.tee 7
                    local.get 2
                    i32.lt_u
                    br_if 0 (;@8;)
                    i32.const 0
                    local.set 10
                    br 3 (;@5;)
                  end
                  block  ;; label = @8
                    local.get 1
                    local.get 7
                    i32.add
                    i32.load8_s
                    i32.const -65
                    i32.le_s
                    br_if 0 (;@8;)
                    i32.const 2
                    local.set 9
                    i32.const 1
                    local.set 10
                    br 3 (;@5;)
                  end
                  i32.const 0
                  local.set 10
                  local.get 3
                  i32.const 3
                  i32.add
                  local.tee 7
                  local.get 2
                  i32.ge_u
                  br_if 2 (;@5;)
                  local.get 1
                  local.get 7
                  i32.add
                  i32.load8_s
                  i32.const -65
                  i32.le_s
                  br_if 0 (;@7;)
                  i32.const 3
                  local.set 9
                  i32.const 1
                  local.set 10
                  br 2 (;@5;)
                end
                local.get 7
                i32.const 1
                i32.add
                local.set 3
                br 3 (;@3;)
              end
              i32.const 1
              local.set 9
              i32.const 1
              local.set 10
            end
            local.get 0
            local.get 3
            i32.store offset=4
            local.get 0
            i32.const 9
            i32.add
            local.get 9
            i32.store8
            local.get 0
            i32.const 8
            i32.add
            local.get 10
            i32.store8
            local.get 0
            i32.const 1
            i32.store
            return
          end
          local.get 3
          local.get 2
          i32.ge_u
          br_if 0 (;@3;)
          loop  ;; label = @4
            local.get 1
            local.get 3
            i32.add
            i32.load8_s
            i32.const 0
            i32.lt_s
            br_if 1 (;@3;)
            local.get 2
            local.get 3
            i32.const 1
            i32.add
            local.tee 3
            i32.ne
            br_if 0 (;@4;)
            br 3 (;@1;)
          end
        end
        local.get 3
        local.get 2
        i32.lt_u
        br_if 0 (;@2;)
      end
    end
    local.get 0
    local.get 1
    i32.store offset=4
    local.get 0
    i32.const 8
    i32.add
    local.get 2
    i32.store
    local.get 0
    i32.const 0
    i32.store)
  (func $_ZN4core3fmt8builders11DebugStruct5field17hc48bc4b5ba431201E (type 12) (param i32 i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i64 i64)
    global.get $__stack_pointer
    i32.const 64
    i32.sub
    local.tee 5
    global.set $__stack_pointer
    i32.const 1
    local.set 6
    block  ;; label = @1
      local.get 0
      i32.load8_u offset=4
      br_if 0 (;@1;)
      local.get 0
      i32.load8_u offset=5
      local.set 7
      block  ;; label = @2
        local.get 0
        i32.load
        local.tee 8
        i32.load offset=24
        local.tee 9
        i32.const 4
        i32.and
        br_if 0 (;@2;)
        i32.const 1
        local.set 6
        local.get 8
        i32.load
        i32.const 1055441
        i32.const 1055443
        local.get 7
        i32.const 255
        i32.and
        local.tee 7
        select
        i32.const 2
        i32.const 3
        local.get 7
        select
        local.get 8
        i32.load offset=4
        i32.load offset=12
        call_indirect (type 7)
        br_if 1 (;@1;)
        i32.const 1
        local.set 6
        local.get 8
        i32.load
        local.get 1
        local.get 2
        local.get 8
        i32.load offset=4
        i32.load offset=12
        call_indirect (type 7)
        br_if 1 (;@1;)
        i32.const 1
        local.set 6
        local.get 8
        i32.load
        i32.const 1055388
        i32.const 2
        local.get 8
        i32.load offset=4
        i32.load offset=12
        call_indirect (type 7)
        br_if 1 (;@1;)
        local.get 3
        local.get 8
        local.get 4
        i32.load offset=12
        call_indirect (type 2)
        local.set 6
        br 1 (;@1;)
      end
      block  ;; label = @2
        local.get 7
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 1
        local.set 6
        local.get 8
        i32.load
        i32.const 1055436
        i32.const 3
        local.get 8
        i32.load offset=4
        i32.load offset=12
        call_indirect (type 7)
        br_if 1 (;@1;)
        local.get 8
        i32.load offset=24
        local.set 9
      end
      i32.const 1
      local.set 6
      local.get 5
      i32.const 1
      i32.store8 offset=23
      local.get 5
      i32.const 1055408
      i32.store offset=28
      local.get 5
      local.get 8
      i64.load align=4
      i64.store offset=8
      local.get 5
      local.get 5
      i32.const 23
      i32.add
      i32.store offset=16
      local.get 8
      i64.load offset=8 align=4
      local.set 10
      local.get 8
      i64.load offset=16 align=4
      local.set 11
      local.get 5
      local.get 8
      i32.load8_u offset=32
      i32.store8 offset=56
      local.get 5
      local.get 8
      i32.load offset=28
      i32.store offset=52
      local.get 5
      local.get 9
      i32.store offset=48
      local.get 5
      local.get 11
      i64.store offset=40
      local.get 5
      local.get 10
      i64.store offset=32
      local.get 5
      local.get 5
      i32.const 8
      i32.add
      i32.store offset=24
      local.get 5
      i32.const 8
      i32.add
      local.get 1
      local.get 2
      call $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h074df0fdbb17926eE
      br_if 0 (;@1;)
      local.get 5
      i32.const 8
      i32.add
      i32.const 1055388
      i32.const 2
      call $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h074df0fdbb17926eE
      br_if 0 (;@1;)
      local.get 3
      local.get 5
      i32.const 24
      i32.add
      local.get 4
      i32.load offset=12
      call_indirect (type 2)
      br_if 0 (;@1;)
      local.get 5
      i32.load offset=24
      i32.const 1055439
      i32.const 2
      local.get 5
      i32.load offset=28
      i32.load offset=12
      call_indirect (type 7)
      local.set 6
    end
    local.get 0
    i32.const 1
    i32.store8 offset=5
    local.get 0
    local.get 6
    i32.store8 offset=4
    local.get 5
    i32.const 64
    i32.add
    global.set $__stack_pointer
    local.get 0)
  (func $_ZN4core3fmt3num3imp51_$LT$impl$u20$core..fmt..Display$u20$for$u20$u8$GT$3fmt17h60761d11360ffc45E (type 2) (param i32 i32) (result i32)
    local.get 0
    i64.load8_u
    i32.const 1
    local.get 1
    call $_ZN4core3fmt3num3imp7fmt_u6417he8d38148f6fbdea6E)
  (func $_ZN4core6result13unwrap_failed17h0cc42b3b30eb09ddE (type 13) (param i32 i32 i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 64
    i32.sub
    local.tee 5
    global.set $__stack_pointer
    local.get 5
    local.get 1
    i32.store offset=12
    local.get 5
    local.get 0
    i32.store offset=8
    local.get 5
    local.get 3
    i32.store offset=20
    local.get 5
    local.get 2
    i32.store offset=16
    local.get 5
    i32.const 24
    i32.add
    i32.const 12
    i32.add
    i32.const 2
    i32.store
    local.get 5
    i32.const 44
    i32.add
    i32.const 2
    i32.store
    local.get 5
    i32.const 48
    i32.add
    i32.const 12
    i32.add
    i32.const 85
    i32.store
    local.get 5
    i32.const 1055392
    i32.store offset=32
    local.get 5
    i32.const 0
    i32.store offset=24
    local.get 5
    i32.const 86
    i32.store offset=52
    local.get 5
    local.get 5
    i32.const 48
    i32.add
    i32.store offset=40
    local.get 5
    local.get 5
    i32.const 16
    i32.add
    i32.store offset=56
    local.get 5
    local.get 5
    i32.const 8
    i32.add
    i32.store offset=48
    local.get 5
    i32.const 24
    i32.add
    local.get 4
    call $_ZN4core9panicking9panic_fmt17h0724b0a601bd3679E
    unreachable)
  (func $_ZN4core6option13expect_failed17h001f1263852c0fc9E (type 5) (param i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    local.get 1
    i32.store offset=12
    local.get 3
    local.get 0
    i32.store offset=8
    local.get 3
    i32.const 28
    i32.add
    i32.const 1
    i32.store
    local.get 3
    i32.const 36
    i32.add
    i32.const 1
    i32.store
    local.get 3
    i32.const 1055096
    i32.store offset=24
    local.get 3
    i32.const 0
    i32.store offset=16
    local.get 3
    i32.const 86
    i32.store offset=44
    local.get 3
    local.get 3
    i32.const 40
    i32.add
    i32.store offset=32
    local.get 3
    local.get 3
    i32.const 8
    i32.add
    i32.store offset=40
    local.get 3
    i32.const 16
    i32.add
    local.get 2
    call $_ZN4core9panicking9panic_fmt17h0724b0a601bd3679E
    unreachable)
  (func $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h8ff29995c2d0d3f7E (type 2) (param i32 i32) (result i32)
    local.get 1
    local.get 0
    i32.load
    local.get 0
    i32.load offset=4
    call $_ZN4core3fmt9Formatter3pad17hea5c16334677f526E)
  (func $_ZN70_$LT$core..panic..location..Location$u20$as$u20$core..fmt..Display$GT$3fmt17h606141a768b76557E (type 2) (param i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    i32.const 20
    i32.add
    i32.const 11
    i32.store
    local.get 2
    i32.const 12
    i32.add
    i32.const 11
    i32.store
    local.get 2
    i32.const 86
    i32.store offset=4
    local.get 2
    local.get 0
    i32.store
    local.get 2
    local.get 0
    i32.const 12
    i32.add
    i32.store offset=16
    local.get 2
    local.get 0
    i32.const 8
    i32.add
    i32.store offset=8
    local.get 1
    i32.load
    local.set 0
    local.get 1
    i32.load offset=4
    local.set 1
    local.get 2
    i32.const 3
    i32.store offset=44
    local.get 2
    i32.const 3
    i32.store offset=36
    local.get 2
    i32.const 1055108
    i32.store offset=32
    local.get 2
    i32.const 0
    i32.store offset=24
    local.get 2
    local.get 2
    i32.store offset=40
    local.get 0
    local.get 1
    local.get 2
    i32.const 24
    i32.add
    call $_ZN4core3fmt5write17h56e014c9dc514616E
    local.set 0
    local.get 2
    i32.const 48
    i32.add
    global.set $__stack_pointer
    local.get 0)
  (func $_ZN4core5panic10panic_info9PanicInfo7payload17h9a735917efe7934bE (type 6) (param i32 i32)
    local.get 0
    local.get 1
    i64.load align=4
    i64.store)
  (func $_ZN4core5panic10panic_info9PanicInfo7message17hf52e4c2cd1887affE (type 4) (param i32) (result i32)
    local.get 0
    i32.load offset=8)
  (func $_ZN4core5panic10panic_info9PanicInfo8location17hac97990c2b530673E (type 4) (param i32) (result i32)
    local.get 0
    i32.load offset=12)
  (func $_ZN4core5panic10panic_info9PanicInfo10can_unwind17h3bf06bc3f77600faE (type 4) (param i32) (result i32)
    local.get 0
    i32.load8_u offset=16)
  (func $_ZN73_$LT$core..panic..panic_info..PanicInfo$u20$as$u20$core..fmt..Display$GT$3fmt17hda9c7998793e9d25E (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 64
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    i32.const 1
    local.set 3
    block  ;; label = @1
      local.get 1
      i32.load
      local.tee 4
      i32.const 1055132
      i32.const 12
      local.get 1
      i32.load offset=4
      local.tee 1
      i32.load offset=12
      call_indirect (type 7)
      br_if 0 (;@1;)
      block  ;; label = @2
        block  ;; label = @3
          local.get 0
          i32.load offset=8
          local.tee 3
          i32.eqz
          br_if 0 (;@3;)
          local.get 2
          local.get 3
          i32.store offset=12
          local.get 2
          i32.const 87
          i32.store offset=20
          local.get 2
          local.get 2
          i32.const 12
          i32.add
          i32.store offset=16
          i32.const 1
          local.set 3
          local.get 2
          i32.const 1
          i32.store offset=60
          local.get 2
          i32.const 2
          i32.store offset=52
          local.get 2
          i32.const 1055148
          i32.store offset=48
          local.get 2
          i32.const 0
          i32.store offset=40
          local.get 2
          local.get 2
          i32.const 16
          i32.add
          i32.store offset=56
          local.get 4
          local.get 1
          local.get 2
          i32.const 40
          i32.add
          call $_ZN4core3fmt5write17h56e014c9dc514616E
          i32.eqz
          br_if 1 (;@2;)
          br 2 (;@1;)
        end
        local.get 0
        i32.load
        local.tee 3
        local.get 0
        i32.load offset=4
        i32.const 12
        i32.add
        i32.load
        call_indirect (type 3)
        i64.const -4493808902380553279
        i64.ne
        br_if 0 (;@2;)
        local.get 2
        local.get 3
        i32.store offset=12
        local.get 2
        i32.const 88
        i32.store offset=20
        local.get 2
        local.get 2
        i32.const 12
        i32.add
        i32.store offset=16
        i32.const 1
        local.set 3
        local.get 2
        i32.const 1
        i32.store offset=60
        local.get 2
        i32.const 2
        i32.store offset=52
        local.get 2
        i32.const 1055148
        i32.store offset=48
        local.get 2
        i32.const 0
        i32.store offset=40
        local.get 2
        local.get 2
        i32.const 16
        i32.add
        i32.store offset=56
        local.get 4
        local.get 1
        local.get 2
        i32.const 40
        i32.add
        call $_ZN4core3fmt5write17h56e014c9dc514616E
        br_if 1 (;@1;)
      end
      local.get 0
      i32.load offset=12
      local.set 3
      local.get 2
      i32.const 36
      i32.add
      i32.const 11
      i32.store
      local.get 2
      i32.const 16
      i32.add
      i32.const 12
      i32.add
      i32.const 11
      i32.store
      local.get 2
      local.get 3
      i32.const 12
      i32.add
      i32.store offset=32
      local.get 2
      local.get 3
      i32.const 8
      i32.add
      i32.store offset=24
      local.get 2
      i32.const 86
      i32.store offset=20
      local.get 2
      local.get 3
      i32.store offset=16
      local.get 2
      i32.const 3
      i32.store offset=60
      local.get 2
      i32.const 3
      i32.store offset=52
      local.get 2
      i32.const 1055108
      i32.store offset=48
      local.get 2
      i32.const 0
      i32.store offset=40
      local.get 2
      local.get 2
      i32.const 16
      i32.add
      i32.store offset=56
      local.get 4
      local.get 1
      local.get 2
      i32.const 40
      i32.add
      call $_ZN4core3fmt5write17h56e014c9dc514616E
      local.set 3
    end
    local.get 2
    i32.const 64
    i32.add
    global.set $__stack_pointer
    local.get 3)
  (func $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h20d7637be7555355E (type 2) (param i32 i32) (result i32)
    local.get 1
    i32.load
    local.get 1
    i32.load offset=4
    local.get 0
    i32.load
    call $_ZN4core3fmt5write17h56e014c9dc514616E)
  (func $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h95d6fa887e65cea7E (type 2) (param i32 i32) (result i32)
    local.get 1
    local.get 0
    i32.load
    local.tee 0
    i32.load
    local.get 0
    i32.load offset=4
    call $_ZN4core3fmt9Formatter3pad17hea5c16334677f526E)
  (func $_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i32$GT$3fmt17h67ac2fbea4a9fd5fE (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 128
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 0
    i32.load
    local.set 0
    i32.const 0
    local.set 3
    loop  ;; label = @1
      local.get 2
      local.get 3
      i32.add
      i32.const 127
      i32.add
      i32.const 48
      i32.const 87
      local.get 0
      i32.const 15
      i32.and
      local.tee 4
      i32.const 10
      i32.lt_u
      select
      local.get 4
      i32.add
      i32.store8
      local.get 3
      i32.const -1
      i32.add
      local.set 3
      local.get 0
      i32.const 15
      i32.gt_u
      local.set 4
      local.get 0
      i32.const 4
      i32.shr_u
      local.set 0
      local.get 4
      br_if 0 (;@1;)
    end
    block  ;; label = @1
      local.get 3
      i32.const 128
      i32.add
      local.tee 0
      i32.const 129
      i32.lt_u
      br_if 0 (;@1;)
      local.get 0
      i32.const 128
      i32.const 1055500
      call $_ZN4core5slice5index26slice_start_index_len_fail17h12ea13b84dc5aa33E
      unreachable
    end
    local.get 1
    i32.const 1
    i32.const 1055516
    i32.const 2
    local.get 2
    local.get 3
    i32.add
    i32.const 128
    i32.add
    i32.const 0
    local.get 3
    i32.sub
    call $_ZN4core3fmt9Formatter12pad_integral17h2602ad621a1201d6E
    local.set 0
    local.get 2
    i32.const 128
    i32.add
    global.set $__stack_pointer
    local.get 0)
  (func $_ZN4core9panicking19assert_failed_inner17hc1caefd2be2cf3dcE (type 16) (param i32 i32 i32 i32 i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 112
    i32.sub
    local.tee 7
    global.set $__stack_pointer
    local.get 7
    local.get 2
    i32.store offset=12
    local.get 7
    local.get 1
    i32.store offset=8
    local.get 7
    local.get 4
    i32.store offset=20
    local.get 7
    local.get 3
    i32.store offset=16
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 0
            i32.const 255
            i32.and
            br_table 0 (;@4;) 1 (;@3;) 2 (;@2;) 0 (;@4;)
          end
          local.get 7
          i32.const 1055257
          i32.store offset=24
          i32.const 2
          local.set 2
          br 2 (;@1;)
        end
        local.get 7
        i32.const 1055255
        i32.store offset=24
        i32.const 2
        local.set 2
        br 1 (;@1;)
      end
      local.get 7
      i32.const 1055248
      i32.store offset=24
      i32.const 7
      local.set 2
    end
    local.get 7
    local.get 2
    i32.store offset=28
    block  ;; label = @1
      local.get 5
      i32.load offset=8
      br_if 0 (;@1;)
      local.get 7
      i32.const 56
      i32.add
      i32.const 20
      i32.add
      i32.const 85
      i32.store
      local.get 7
      i32.const 56
      i32.add
      i32.const 12
      i32.add
      i32.const 85
      i32.store
      local.get 7
      i32.const 88
      i32.add
      i32.const 12
      i32.add
      i32.const 4
      i32.store
      local.get 7
      i32.const 88
      i32.add
      i32.const 20
      i32.add
      i32.const 3
      i32.store
      local.get 7
      i32.const 1055356
      i32.store offset=96
      local.get 7
      i32.const 0
      i32.store offset=88
      local.get 7
      i32.const 86
      i32.store offset=60
      local.get 7
      local.get 7
      i32.const 56
      i32.add
      i32.store offset=104
      local.get 7
      local.get 7
      i32.const 16
      i32.add
      i32.store offset=72
      local.get 7
      local.get 7
      i32.const 8
      i32.add
      i32.store offset=64
      local.get 7
      local.get 7
      i32.const 24
      i32.add
      i32.store offset=56
      local.get 7
      i32.const 88
      i32.add
      local.get 6
      call $_ZN4core9panicking9panic_fmt17h0724b0a601bd3679E
      unreachable
    end
    local.get 7
    i32.const 32
    i32.add
    i32.const 16
    i32.add
    local.get 5
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 7
    i32.const 32
    i32.add
    i32.const 8
    i32.add
    local.get 5
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 7
    local.get 5
    i64.load align=4
    i64.store offset=32
    local.get 7
    i32.const 88
    i32.add
    i32.const 12
    i32.add
    i32.const 4
    i32.store
    local.get 7
    i32.const 88
    i32.add
    i32.const 20
    i32.add
    i32.const 4
    i32.store
    local.get 7
    i32.const 84
    i32.add
    i32.const 7
    i32.store
    local.get 7
    i32.const 56
    i32.add
    i32.const 20
    i32.add
    i32.const 85
    i32.store
    local.get 7
    i32.const 56
    i32.add
    i32.const 12
    i32.add
    i32.const 85
    i32.store
    local.get 7
    i32.const 1055320
    i32.store offset=96
    local.get 7
    i32.const 0
    i32.store offset=88
    local.get 7
    i32.const 86
    i32.store offset=60
    local.get 7
    local.get 7
    i32.const 56
    i32.add
    i32.store offset=104
    local.get 7
    local.get 7
    i32.const 32
    i32.add
    i32.store offset=80
    local.get 7
    local.get 7
    i32.const 16
    i32.add
    i32.store offset=72
    local.get 7
    local.get 7
    i32.const 8
    i32.add
    i32.store offset=64
    local.get 7
    local.get 7
    i32.const 24
    i32.add
    i32.store offset=56
    local.get 7
    i32.const 88
    i32.add
    local.get 6
    call $_ZN4core9panicking9panic_fmt17h0724b0a601bd3679E
    unreachable)
  (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hc77e939c53299de2E (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 1
    local.get 0
    i32.load offset=4
    i32.load offset=12
    call_indirect (type 2))
  (func $_ZN59_$LT$core..fmt..Arguments$u20$as$u20$core..fmt..Display$GT$3fmt17h0bbc610b5c9a689cE (type 2) (param i32 i32) (result i32)
    local.get 1
    i32.load
    local.get 1
    i32.load offset=4
    local.get 0
    call $_ZN4core3fmt5write17h56e014c9dc514616E)
  (func $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h074df0fdbb17926eE (type 7) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 0
    i32.load offset=4
    local.set 4
    local.get 0
    i32.load
    local.set 5
    local.get 0
    i32.load offset=8
    local.set 6
    i32.const 0
    local.set 7
    i32.const 0
    local.set 8
    i32.const 0
    local.set 9
    i32.const 0
    local.set 10
    block  ;; label = @1
      loop  ;; label = @2
        local.get 10
        i32.const 255
        i32.and
        br_if 1 (;@1;)
        block  ;; label = @3
          block  ;; label = @4
            local.get 8
            local.get 2
            i32.gt_u
            br_if 0 (;@4;)
            loop  ;; label = @5
              local.get 1
              local.get 8
              i32.add
              local.set 11
              block  ;; label = @6
                block  ;; label = @7
                  local.get 2
                  local.get 8
                  i32.sub
                  local.tee 12
                  i32.const 8
                  i32.lt_u
                  br_if 0 (;@7;)
                  local.get 3
                  i32.const 8
                  i32.add
                  i32.const 10
                  local.get 11
                  local.get 12
                  call $_ZN4core5slice6memchr14memchr_aligned17h4ab011c4b696055dE
                  local.get 3
                  i32.load offset=12
                  local.set 0
                  local.get 3
                  i32.load offset=8
                  local.set 11
                  br 1 (;@6;)
                end
                i32.const 0
                local.set 0
                block  ;; label = @7
                  local.get 12
                  br_if 0 (;@7;)
                  i32.const 0
                  local.set 11
                  br 1 (;@6;)
                end
                loop  ;; label = @7
                  block  ;; label = @8
                    local.get 11
                    local.get 0
                    i32.add
                    i32.load8_u
                    i32.const 10
                    i32.ne
                    br_if 0 (;@8;)
                    i32.const 1
                    local.set 11
                    br 2 (;@6;)
                  end
                  local.get 12
                  local.get 0
                  i32.const 1
                  i32.add
                  local.tee 0
                  i32.ne
                  br_if 0 (;@7;)
                end
                i32.const 0
                local.set 11
                local.get 12
                local.set 0
              end
              block  ;; label = @6
                local.get 11
                i32.const 1
                i32.eq
                br_if 0 (;@6;)
                local.get 2
                local.set 8
                br 2 (;@4;)
              end
              local.get 8
              local.get 0
              i32.add
              local.tee 0
              i32.const 1
              i32.add
              local.set 8
              block  ;; label = @6
                local.get 0
                local.get 2
                i32.ge_u
                br_if 0 (;@6;)
                local.get 1
                local.get 0
                i32.add
                i32.load8_u
                i32.const 10
                i32.ne
                br_if 0 (;@6;)
                i32.const 0
                local.set 10
                local.get 8
                local.set 13
                local.get 8
                local.set 0
                br 3 (;@3;)
              end
              local.get 8
              local.get 2
              i32.le_u
              br_if 0 (;@5;)
            end
          end
          i32.const 1
          local.set 10
          local.get 9
          local.set 13
          local.get 2
          local.set 0
          local.get 9
          local.get 2
          i32.eq
          br_if 2 (;@1;)
        end
        block  ;; label = @3
          block  ;; label = @4
            local.get 6
            i32.load8_u
            i32.eqz
            br_if 0 (;@4;)
            local.get 5
            i32.const 1055432
            i32.const 4
            local.get 4
            i32.load offset=12
            call_indirect (type 7)
            br_if 1 (;@3;)
          end
          local.get 1
          local.get 9
          i32.add
          local.set 12
          local.get 0
          local.get 9
          i32.sub
          local.set 11
          i32.const 0
          local.set 14
          block  ;; label = @4
            local.get 0
            local.get 9
            i32.eq
            br_if 0 (;@4;)
            local.get 11
            local.get 12
            i32.add
            i32.const -1
            i32.add
            i32.load8_u
            i32.const 10
            i32.eq
            local.set 14
          end
          local.get 6
          local.get 14
          i32.store8
          local.get 13
          local.set 9
          local.get 5
          local.get 12
          local.get 11
          local.get 4
          i32.load offset=12
          call_indirect (type 7)
          i32.eqz
          br_if 1 (;@2;)
        end
      end
      i32.const 1
      local.set 7
    end
    local.get 3
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 7)
  (func $_ZN4core3fmt8builders11DebugStruct6finish17h3cefb3b4bef49d8cE (type 4) (param i32) (result i32)
    (local i32 i32)
    local.get 0
    i32.load8_u offset=4
    local.set 1
    block  ;; label = @1
      local.get 0
      i32.load8_u offset=5
      br_if 0 (;@1;)
      local.get 1
      i32.const 255
      i32.and
      i32.const 0
      i32.ne
      return
    end
    i32.const 1
    local.set 2
    block  ;; label = @1
      local.get 1
      i32.const 255
      i32.and
      br_if 0 (;@1;)
      block  ;; label = @2
        local.get 0
        i32.load
        local.tee 1
        i32.load8_u offset=24
        i32.const 4
        i32.and
        br_if 0 (;@2;)
        local.get 0
        local.get 1
        i32.load
        i32.const 1055447
        i32.const 2
        local.get 1
        i32.load offset=4
        i32.load offset=12
        call_indirect (type 7)
        local.tee 1
        i32.store8 offset=4
        local.get 1
        return
      end
      local.get 1
      i32.load
      i32.const 1055446
      i32.const 1
      local.get 1
      i32.load offset=4
      i32.load offset=12
      call_indirect (type 7)
      local.set 2
    end
    local.get 0
    local.get 2
    i32.store8 offset=4
    local.get 2)
  (func $_ZN4core3fmt8builders10DebugTuple5field17hbd92e7b522abdb91E (type 7) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i64 i64)
    global.get $__stack_pointer
    i32.const 64
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load8_u offset=8
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        i32.load
        local.set 4
        i32.const 1
        local.set 5
        br 1 (;@1;)
      end
      local.get 0
      i32.load
      local.set 4
      block  ;; label = @2
        local.get 0
        i32.const 4
        i32.add
        i32.load
        local.tee 6
        i32.load offset=24
        local.tee 7
        i32.const 4
        i32.and
        br_if 0 (;@2;)
        i32.const 1
        local.set 5
        local.get 6
        i32.load
        i32.const 1055441
        i32.const 1055451
        local.get 4
        select
        i32.const 2
        i32.const 1
        local.get 4
        select
        local.get 6
        i32.load offset=4
        i32.load offset=12
        call_indirect (type 7)
        br_if 1 (;@1;)
        local.get 1
        local.get 6
        local.get 2
        i32.load offset=12
        call_indirect (type 2)
        local.set 5
        br 1 (;@1;)
      end
      block  ;; label = @2
        local.get 4
        br_if 0 (;@2;)
        block  ;; label = @3
          local.get 6
          i32.load
          i32.const 1055449
          i32.const 2
          local.get 6
          i32.load offset=4
          i32.load offset=12
          call_indirect (type 7)
          i32.eqz
          br_if 0 (;@3;)
          i32.const 1
          local.set 5
          i32.const 0
          local.set 4
          br 2 (;@1;)
        end
        local.get 6
        i32.load offset=24
        local.set 7
      end
      i32.const 1
      local.set 5
      local.get 3
      i32.const 1
      i32.store8 offset=23
      local.get 3
      i32.const 1055408
      i32.store offset=28
      local.get 3
      local.get 6
      i64.load align=4
      i64.store offset=8
      local.get 3
      local.get 3
      i32.const 23
      i32.add
      i32.store offset=16
      local.get 6
      i64.load offset=8 align=4
      local.set 8
      local.get 6
      i64.load offset=16 align=4
      local.set 9
      local.get 3
      local.get 6
      i32.load8_u offset=32
      i32.store8 offset=56
      local.get 3
      local.get 6
      i32.load offset=28
      i32.store offset=52
      local.get 3
      local.get 7
      i32.store offset=48
      local.get 3
      local.get 9
      i64.store offset=40
      local.get 3
      local.get 8
      i64.store offset=32
      local.get 3
      local.get 3
      i32.const 8
      i32.add
      i32.store offset=24
      local.get 1
      local.get 3
      i32.const 24
      i32.add
      local.get 2
      i32.load offset=12
      call_indirect (type 2)
      br_if 0 (;@1;)
      local.get 3
      i32.load offset=24
      i32.const 1055439
      i32.const 2
      local.get 3
      i32.load offset=28
      i32.load offset=12
      call_indirect (type 7)
      local.set 5
    end
    local.get 0
    local.get 5
    i32.store8 offset=8
    local.get 0
    local.get 4
    i32.const 1
    i32.add
    i32.store
    local.get 3
    i32.const 64
    i32.add
    global.set $__stack_pointer
    local.get 0)
  (func $_ZN4core3fmt8builders10DebugTuple6finish17hda7cfb178db66c11E (type 4) (param i32) (result i32)
    (local i32 i32 i32)
    local.get 0
    i32.load8_u offset=8
    local.set 1
    block  ;; label = @1
      local.get 0
      i32.load
      local.tee 2
      br_if 0 (;@1;)
      local.get 1
      i32.const 255
      i32.and
      i32.const 0
      i32.ne
      return
    end
    i32.const 1
    local.set 3
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        block  ;; label = @3
          block  ;; label = @4
            local.get 2
            i32.const 1
            i32.ne
            br_if 0 (;@4;)
            local.get 0
            i32.load8_u offset=9
            i32.const 255
            i32.and
            br_if 1 (;@3;)
          end
          local.get 0
          i32.load offset=4
          local.set 1
          br 2 (;@1;)
        end
        local.get 0
        i32.const 4
        i32.add
        i32.load
        local.tee 1
        i32.load8_u offset=24
        i32.const 4
        i32.and
        br_if 1 (;@1;)
        i32.const 1
        local.set 3
        local.get 1
        i32.load
        i32.const 1055452
        i32.const 1
        local.get 1
        i32.load offset=4
        i32.load offset=12
        call_indirect (type 7)
        i32.eqz
        br_if 1 (;@1;)
      end
      local.get 0
      local.get 3
      i32.store8 offset=8
      local.get 3
      return
    end
    local.get 0
    local.get 1
    i32.load
    i32.const 1054992
    i32.const 1
    local.get 1
    i32.load offset=4
    i32.load offset=12
    call_indirect (type 7)
    local.tee 1
    i32.store8 offset=8
    local.get 1)
  (func $_ZN4core3fmt8builders10DebugInner5entry17hf769770a1eda9eb7E (type 5) (param i32 i32 i32)
    (local i32 i32 i32 i32 i64 i64)
    global.get $__stack_pointer
    i32.const 64
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    i32.const 1
    local.set 4
    block  ;; label = @1
      local.get 0
      i32.load8_u offset=4
      br_if 0 (;@1;)
      local.get 0
      i32.load8_u offset=5
      local.set 4
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 0
              i32.load
              local.tee 5
              i32.load offset=24
              local.tee 6
              i32.const 4
              i32.and
              br_if 0 (;@5;)
              local.get 4
              i32.const 255
              i32.and
              br_if 1 (;@4;)
              br 3 (;@2;)
            end
            local.get 4
            i32.const 255
            i32.and
            br_if 1 (;@3;)
            i32.const 1
            local.set 4
            local.get 5
            i32.load
            i32.const 1055453
            i32.const 1
            local.get 5
            i32.load offset=4
            i32.load offset=12
            call_indirect (type 7)
            br_if 3 (;@1;)
            local.get 5
            i32.load offset=24
            local.set 6
            br 1 (;@3;)
          end
          i32.const 1
          local.set 4
          local.get 5
          i32.load
          i32.const 1055441
          i32.const 2
          local.get 5
          i32.load offset=4
          i32.load offset=12
          call_indirect (type 7)
          i32.eqz
          br_if 1 (;@2;)
          br 2 (;@1;)
        end
        i32.const 1
        local.set 4
        local.get 3
        i32.const 1
        i32.store8 offset=23
        local.get 3
        i32.const 1055408
        i32.store offset=28
        local.get 3
        local.get 5
        i64.load align=4
        i64.store offset=8
        local.get 3
        local.get 3
        i32.const 23
        i32.add
        i32.store offset=16
        local.get 5
        i64.load offset=8 align=4
        local.set 7
        local.get 5
        i64.load offset=16 align=4
        local.set 8
        local.get 3
        local.get 5
        i32.load8_u offset=32
        i32.store8 offset=56
        local.get 3
        local.get 5
        i32.load offset=28
        i32.store offset=52
        local.get 3
        local.get 6
        i32.store offset=48
        local.get 3
        local.get 8
        i64.store offset=40
        local.get 3
        local.get 7
        i64.store offset=32
        local.get 3
        local.get 3
        i32.const 8
        i32.add
        i32.store offset=24
        local.get 1
        local.get 3
        i32.const 24
        i32.add
        local.get 2
        i32.load offset=12
        call_indirect (type 2)
        br_if 1 (;@1;)
        local.get 3
        i32.load offset=24
        i32.const 1055439
        i32.const 2
        local.get 3
        i32.load offset=28
        i32.load offset=12
        call_indirect (type 7)
        local.set 4
        br 1 (;@1;)
      end
      local.get 1
      local.get 5
      local.get 2
      i32.load offset=12
      call_indirect (type 2)
      local.set 4
    end
    local.get 0
    i32.const 1
    i32.store8 offset=5
    local.get 0
    local.get 4
    i32.store8 offset=4
    local.get 3
    i32.const 64
    i32.add
    global.set $__stack_pointer)
  (func $_ZN4core3fmt8builders8DebugSet5entry17h65c60cb7d5a3b232E (type 7) (param i32 i32 i32) (result i32)
    local.get 0
    local.get 1
    local.get 2
    call $_ZN4core3fmt8builders10DebugInner5entry17hf769770a1eda9eb7E
    local.get 0)
  (func $_ZN4core3fmt8builders9DebugList6finish17h0808958983b26dd3E (type 4) (param i32) (result i32)
    (local i32)
    i32.const 1
    local.set 1
    block  ;; label = @1
      local.get 0
      i32.load8_u offset=4
      br_if 0 (;@1;)
      local.get 0
      i32.load
      local.tee 0
      i32.load
      i32.const 1055472
      i32.const 1
      local.get 0
      i32.const 4
      i32.add
      i32.load
      i32.load offset=12
      call_indirect (type 7)
      local.set 1
    end
    local.get 1)
  (func $_ZN4core3fmt5Write10write_char17h3b769f78203a4355E (type 2) (param i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    i32.const 0
    i32.store offset=12
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.const 128
        i32.lt_u
        br_if 0 (;@2;)
        block  ;; label = @3
          local.get 1
          i32.const 2048
          i32.lt_u
          br_if 0 (;@3;)
          block  ;; label = @4
            local.get 1
            i32.const 65536
            i32.lt_u
            br_if 0 (;@4;)
            local.get 2
            local.get 1
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=15
            local.get 2
            local.get 1
            i32.const 6
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=14
            local.get 2
            local.get 1
            i32.const 12
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=13
            local.get 2
            local.get 1
            i32.const 18
            i32.shr_u
            i32.const 7
            i32.and
            i32.const 240
            i32.or
            i32.store8 offset=12
            i32.const 4
            local.set 1
            br 3 (;@1;)
          end
          local.get 2
          local.get 1
          i32.const 63
          i32.and
          i32.const 128
          i32.or
          i32.store8 offset=14
          local.get 2
          local.get 1
          i32.const 12
          i32.shr_u
          i32.const 224
          i32.or
          i32.store8 offset=12
          local.get 2
          local.get 1
          i32.const 6
          i32.shr_u
          i32.const 63
          i32.and
          i32.const 128
          i32.or
          i32.store8 offset=13
          i32.const 3
          local.set 1
          br 2 (;@1;)
        end
        local.get 2
        local.get 1
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=13
        local.get 2
        local.get 1
        i32.const 6
        i32.shr_u
        i32.const 192
        i32.or
        i32.store8 offset=12
        i32.const 2
        local.set 1
        br 1 (;@1;)
      end
      local.get 2
      local.get 1
      i32.store8 offset=12
      i32.const 1
      local.set 1
    end
    local.get 0
    local.get 2
    i32.const 12
    i32.add
    local.get 1
    call $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h074df0fdbb17926eE
    local.set 1
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 1)
  (func $_ZN4core3fmt5Write9write_fmt17hf9f747f21f49b304E (type 2) (param i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    local.get 0
    i32.store offset=4
    local.get 2
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    local.get 1
    i64.load align=4
    i64.store offset=8
    local.get 2
    i32.const 4
    i32.add
    i32.const 1055720
    local.get 2
    i32.const 8
    i32.add
    call $_ZN4core3fmt5write17h56e014c9dc514616E
    local.set 1
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
    local.get 1)
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17hb49c5e69f7371507E (type 7) (param i32 i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 1
    local.get 2
    call $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h074df0fdbb17926eE)
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h9d7c24ae95e50eb8E (type 2) (param i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 0
    i32.load
    local.set 0
    local.get 2
    i32.const 0
    i32.store offset=12
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.const 128
        i32.lt_u
        br_if 0 (;@2;)
        block  ;; label = @3
          local.get 1
          i32.const 2048
          i32.lt_u
          br_if 0 (;@3;)
          block  ;; label = @4
            local.get 1
            i32.const 65536
            i32.lt_u
            br_if 0 (;@4;)
            local.get 2
            local.get 1
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=15
            local.get 2
            local.get 1
            i32.const 6
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=14
            local.get 2
            local.get 1
            i32.const 12
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=13
            local.get 2
            local.get 1
            i32.const 18
            i32.shr_u
            i32.const 7
            i32.and
            i32.const 240
            i32.or
            i32.store8 offset=12
            i32.const 4
            local.set 1
            br 3 (;@1;)
          end
          local.get 2
          local.get 1
          i32.const 63
          i32.and
          i32.const 128
          i32.or
          i32.store8 offset=14
          local.get 2
          local.get 1
          i32.const 12
          i32.shr_u
          i32.const 224
          i32.or
          i32.store8 offset=12
          local.get 2
          local.get 1
          i32.const 6
          i32.shr_u
          i32.const 63
          i32.and
          i32.const 128
          i32.or
          i32.store8 offset=13
          i32.const 3
          local.set 1
          br 2 (;@1;)
        end
        local.get 2
        local.get 1
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=13
        local.get 2
        local.get 1
        i32.const 6
        i32.shr_u
        i32.const 192
        i32.or
        i32.store8 offset=12
        i32.const 2
        local.set 1
        br 1 (;@1;)
      end
      local.get 2
      local.get 1
      i32.store8 offset=12
      i32.const 1
      local.set 1
    end
    local.get 0
    local.get 2
    i32.const 12
    i32.add
    local.get 1
    call $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h074df0fdbb17926eE
    local.set 1
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 1)
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17h25a03326608365dcE (type 2) (param i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 0
    i32.load
    local.set 0
    local.get 2
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    local.get 1
    i64.load align=4
    i64.store offset=8
    local.get 2
    local.get 0
    i32.store offset=4
    local.get 2
    i32.const 4
    i32.add
    i32.const 1055720
    local.get 2
    i32.const 8
    i32.add
    call $_ZN4core3fmt5write17h56e014c9dc514616E
    local.set 1
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
    local.get 1)
  (func $_ZN4core3str5count14do_count_chars17h9ebe54268b64ed3aE (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.const 3
        i32.add
        i32.const -4
        i32.and
        local.tee 2
        local.get 0
        i32.sub
        local.tee 3
        local.get 1
        i32.gt_u
        br_if 0 (;@2;)
        local.get 3
        i32.const 4
        i32.gt_u
        br_if 0 (;@2;)
        local.get 1
        local.get 3
        i32.sub
        local.tee 4
        i32.const 4
        i32.lt_u
        br_if 0 (;@2;)
        local.get 4
        i32.const 3
        i32.and
        local.set 5
        i32.const 0
        local.set 6
        i32.const 0
        local.set 1
        block  ;; label = @3
          local.get 2
          local.get 0
          i32.eq
          br_if 0 (;@3;)
          local.get 3
          i32.const 3
          i32.and
          local.set 7
          block  ;; label = @4
            block  ;; label = @5
              local.get 2
              local.get 0
              i32.const -1
              i32.xor
              i32.add
              i32.const 3
              i32.ge_u
              br_if 0 (;@5;)
              i32.const 0
              local.set 1
              local.get 0
              local.set 2
              br 1 (;@4;)
            end
            local.get 3
            i32.const -4
            i32.and
            local.set 8
            i32.const 0
            local.set 1
            local.get 0
            local.set 2
            loop  ;; label = @5
              local.get 1
              local.get 2
              i32.load8_s
              i32.const -65
              i32.gt_s
              i32.add
              local.get 2
              i32.load8_s offset=1
              i32.const -65
              i32.gt_s
              i32.add
              local.get 2
              i32.load8_s offset=2
              i32.const -65
              i32.gt_s
              i32.add
              local.get 2
              i32.load8_s offset=3
              i32.const -65
              i32.gt_s
              i32.add
              local.set 1
              local.get 2
              i32.const 4
              i32.add
              local.set 2
              local.get 8
              i32.const -4
              i32.add
              local.tee 8
              br_if 0 (;@5;)
            end
          end
          local.get 7
          i32.eqz
          br_if 0 (;@3;)
          loop  ;; label = @4
            local.get 1
            local.get 2
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.set 1
            local.get 2
            i32.const 1
            i32.add
            local.set 2
            local.get 7
            i32.const -1
            i32.add
            local.tee 7
            br_if 0 (;@4;)
          end
        end
        local.get 0
        local.get 3
        i32.add
        local.set 0
        block  ;; label = @3
          local.get 5
          i32.eqz
          br_if 0 (;@3;)
          local.get 0
          local.get 4
          i32.const -4
          i32.and
          i32.add
          local.tee 2
          i32.load8_s
          i32.const -65
          i32.gt_s
          local.set 6
          local.get 5
          i32.const 1
          i32.eq
          br_if 0 (;@3;)
          local.get 6
          local.get 2
          i32.load8_s offset=1
          i32.const -65
          i32.gt_s
          i32.add
          local.set 6
          local.get 5
          i32.const 2
          i32.eq
          br_if 0 (;@3;)
          local.get 6
          local.get 2
          i32.load8_s offset=2
          i32.const -65
          i32.gt_s
          i32.add
          local.set 6
        end
        local.get 4
        i32.const 2
        i32.shr_u
        local.set 3
        local.get 6
        local.get 1
        i32.add
        local.set 7
        loop  ;; label = @3
          local.get 0
          local.set 6
          local.get 3
          i32.eqz
          br_if 2 (;@1;)
          local.get 3
          i32.const 192
          local.get 3
          i32.const 192
          i32.lt_u
          select
          local.tee 4
          i32.const 3
          i32.and
          local.set 5
          local.get 4
          i32.const 2
          i32.shl
          local.set 9
          block  ;; label = @4
            block  ;; label = @5
              local.get 4
              i32.const 252
              i32.and
              local.tee 10
              br_if 0 (;@5;)
              i32.const 0
              local.set 2
              br 1 (;@4;)
            end
            local.get 6
            local.get 10
            i32.const 2
            i32.shl
            i32.add
            local.set 8
            i32.const 0
            local.set 2
            local.get 6
            local.set 0
            loop  ;; label = @5
              local.get 0
              i32.eqz
              br_if 1 (;@4;)
              local.get 0
              i32.const 12
              i32.add
              i32.load
              local.tee 1
              i32.const -1
              i32.xor
              i32.const 7
              i32.shr_u
              local.get 1
              i32.const 6
              i32.shr_u
              i32.or
              i32.const 16843009
              i32.and
              local.get 0
              i32.const 8
              i32.add
              i32.load
              local.tee 1
              i32.const -1
              i32.xor
              i32.const 7
              i32.shr_u
              local.get 1
              i32.const 6
              i32.shr_u
              i32.or
              i32.const 16843009
              i32.and
              local.get 0
              i32.const 4
              i32.add
              i32.load
              local.tee 1
              i32.const -1
              i32.xor
              i32.const 7
              i32.shr_u
              local.get 1
              i32.const 6
              i32.shr_u
              i32.or
              i32.const 16843009
              i32.and
              local.get 0
              i32.load
              local.tee 1
              i32.const -1
              i32.xor
              i32.const 7
              i32.shr_u
              local.get 1
              i32.const 6
              i32.shr_u
              i32.or
              i32.const 16843009
              i32.and
              local.get 2
              i32.add
              i32.add
              i32.add
              i32.add
              local.set 2
              local.get 0
              i32.const 16
              i32.add
              local.tee 0
              local.get 8
              i32.ne
              br_if 0 (;@5;)
            end
          end
          local.get 3
          local.get 4
          i32.sub
          local.set 3
          local.get 6
          local.get 9
          i32.add
          local.set 0
          local.get 2
          i32.const 8
          i32.shr_u
          i32.const 16711935
          i32.and
          local.get 2
          i32.const 16711935
          i32.and
          i32.add
          i32.const 65537
          i32.mul
          i32.const 16
          i32.shr_u
          local.get 7
          i32.add
          local.set 7
          local.get 5
          i32.eqz
          br_if 0 (;@3;)
        end
        block  ;; label = @3
          block  ;; label = @4
            local.get 6
            br_if 0 (;@4;)
            i32.const 0
            local.set 0
            br 1 (;@3;)
          end
          local.get 6
          local.get 10
          i32.const 2
          i32.shl
          i32.add
          local.tee 2
          i32.load
          local.tee 0
          i32.const -1
          i32.xor
          i32.const 7
          i32.shr_u
          local.get 0
          i32.const 6
          i32.shr_u
          i32.or
          i32.const 16843009
          i32.and
          local.set 0
          local.get 5
          i32.const 1
          i32.eq
          br_if 0 (;@3;)
          local.get 2
          i32.load offset=4
          local.tee 1
          i32.const -1
          i32.xor
          i32.const 7
          i32.shr_u
          local.get 1
          i32.const 6
          i32.shr_u
          i32.or
          i32.const 16843009
          i32.and
          local.get 0
          i32.add
          local.set 0
          local.get 5
          i32.const 2
          i32.eq
          br_if 0 (;@3;)
          local.get 2
          i32.load offset=8
          local.tee 2
          i32.const -1
          i32.xor
          i32.const 7
          i32.shr_u
          local.get 2
          i32.const 6
          i32.shr_u
          i32.or
          i32.const 16843009
          i32.and
          local.get 0
          i32.add
          local.set 0
        end
        local.get 0
        i32.const 8
        i32.shr_u
        i32.const 459007
        i32.and
        local.get 0
        i32.const 16711935
        i32.and
        i32.add
        i32.const 65537
        i32.mul
        i32.const 16
        i32.shr_u
        local.get 7
        i32.add
        return
      end
      block  ;; label = @2
        local.get 1
        br_if 0 (;@2;)
        i32.const 0
        return
      end
      local.get 1
      i32.const 3
      i32.and
      local.set 2
      block  ;; label = @2
        block  ;; label = @3
          local.get 1
          i32.const 4
          i32.ge_u
          br_if 0 (;@3;)
          i32.const 0
          local.set 7
          br 1 (;@2;)
        end
        local.get 1
        i32.const -4
        i32.and
        local.set 1
        i32.const 0
        local.set 7
        loop  ;; label = @3
          local.get 7
          local.get 0
          i32.load8_s
          i32.const -65
          i32.gt_s
          i32.add
          local.get 0
          i32.load8_s offset=1
          i32.const -65
          i32.gt_s
          i32.add
          local.get 0
          i32.load8_s offset=2
          i32.const -65
          i32.gt_s
          i32.add
          local.get 0
          i32.load8_s offset=3
          i32.const -65
          i32.gt_s
          i32.add
          local.set 7
          local.get 0
          i32.const 4
          i32.add
          local.set 0
          local.get 1
          i32.const -4
          i32.add
          local.tee 1
          br_if 0 (;@3;)
        end
      end
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      loop  ;; label = @2
        local.get 7
        local.get 0
        i32.load8_s
        i32.const -65
        i32.gt_s
        i32.add
        local.set 7
        local.get 0
        i32.const 1
        i32.add
        local.set 0
        local.get 2
        i32.const -1
        i32.add
        local.tee 2
        br_if 0 (;@2;)
      end
    end
    local.get 7)
  (func $_ZN4core3fmt9Formatter12pad_integral12write_prefix17h19c5e647cdfac30eE (type 12) (param i32 i32 i32 i32 i32) (result i32)
    (local i32)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 2
          i32.const 1114112
          i32.eq
          br_if 0 (;@3;)
          i32.const 1
          local.set 5
          local.get 0
          local.get 2
          local.get 1
          i32.load offset=16
          call_indirect (type 2)
          br_if 1 (;@2;)
        end
        local.get 3
        br_if 1 (;@1;)
        i32.const 0
        local.set 5
      end
      local.get 5
      return
    end
    local.get 0
    local.get 3
    local.get 4
    local.get 1
    i32.load offset=12
    call_indirect (type 7))
  (func $_ZN4core3str16slice_error_fail17hd8efdeca0603f5c1E (type 13) (param i32 i32 i32 i32 i32)
    local.get 0
    local.get 1
    local.get 2
    local.get 3
    local.get 4
    call $_ZN4core3str19slice_error_fail_rt17h75fd19d0e302092fE
    unreachable)
  (func $_ZN4core3fmt9Formatter9write_str17h6749db218b079d3bE (type 7) (param i32 i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 1
    local.get 2
    local.get 0
    i32.load offset=4
    i32.load offset=12
    call_indirect (type 7))
  (func $_ZN4core3fmt9Formatter9write_fmt17hd7aea38882dc75edE (type 2) (param i32 i32) (result i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 0
    i32.load offset=4
    local.set 3
    local.get 0
    i32.load
    local.set 0
    local.get 2
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    local.get 1
    i64.load align=4
    i64.store offset=8
    local.get 0
    local.get 3
    local.get 2
    i32.const 8
    i32.add
    call $_ZN4core3fmt5write17h56e014c9dc514616E
    local.set 1
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
    local.get 1)
  (func $_ZN4core3fmt9Formatter15debug_lower_hex17h4338310221a5ab5bE (type 4) (param i32) (result i32)
    local.get 0
    i32.load8_u offset=24
    i32.const 16
    i32.and
    i32.const 4
    i32.shr_u)
  (func $_ZN4core3fmt9Formatter15debug_upper_hex17hba00f3ad3b67e305E (type 4) (param i32) (result i32)
    local.get 0
    i32.load8_u offset=24
    i32.const 32
    i32.and
    i32.const 5
    i32.shr_u)
  (func $_ZN4core3fmt9Formatter12debug_struct17hd068653c6d1e2675E (type 11) (param i32 i32 i32 i32)
    local.get 1
    i32.load
    local.get 2
    local.get 3
    local.get 1
    i32.load offset=4
    i32.load offset=12
    call_indirect (type 7)
    local.set 3
    local.get 0
    i32.const 0
    i32.store8 offset=5
    local.get 0
    local.get 3
    i32.store8 offset=4
    local.get 0
    local.get 1
    i32.store)
  (func $_ZN4core3fmt9Formatter26debug_struct_field2_finish17hdae384ded0a9d4beE (type 17) (param i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 11
    global.set $__stack_pointer
    local.get 0
    i32.load
    local.get 1
    local.get 2
    local.get 0
    i32.load offset=4
    i32.load offset=12
    call_indirect (type 7)
    local.set 2
    local.get 11
    i32.const 0
    i32.store8 offset=13
    local.get 11
    local.get 2
    i32.store8 offset=12
    local.get 11
    local.get 0
    i32.store offset=8
    local.get 11
    i32.const 8
    i32.add
    local.get 3
    local.get 4
    local.get 5
    local.get 6
    call $_ZN4core3fmt8builders11DebugStruct5field17hc48bc4b5ba431201E
    local.get 7
    local.get 8
    local.get 9
    local.get 10
    call $_ZN4core3fmt8builders11DebugStruct5field17hc48bc4b5ba431201E
    local.set 1
    local.get 11
    i32.load8_u offset=12
    local.set 2
    block  ;; label = @1
      block  ;; label = @2
        local.get 11
        i32.load8_u offset=13
        br_if 0 (;@2;)
        local.get 2
        i32.const 255
        i32.and
        i32.const 0
        i32.ne
        local.set 0
        br 1 (;@1;)
      end
      i32.const 1
      local.set 0
      local.get 2
      i32.const 255
      i32.and
      br_if 0 (;@1;)
      block  ;; label = @2
        local.get 1
        i32.load
        local.tee 0
        i32.load8_u offset=24
        i32.const 4
        i32.and
        br_if 0 (;@2;)
        local.get 0
        i32.load
        i32.const 1055447
        i32.const 2
        local.get 0
        i32.load offset=4
        i32.load offset=12
        call_indirect (type 7)
        local.set 0
        br 1 (;@1;)
      end
      local.get 0
      i32.load
      i32.const 1055446
      i32.const 1
      local.get 0
      i32.load offset=4
      i32.load offset=12
      call_indirect (type 7)
      local.set 0
    end
    local.get 11
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 0)
  (func $_ZN4core3fmt9Formatter11debug_tuple17he1975a2c89512b73E (type 11) (param i32 i32 i32 i32)
    local.get 0
    local.get 1
    i32.load
    local.get 2
    local.get 3
    local.get 1
    i32.load offset=4
    i32.load offset=12
    call_indirect (type 7)
    i32.store8 offset=8
    local.get 0
    local.get 1
    i32.store offset=4
    local.get 0
    local.get 3
    i32.eqz
    i32.store8 offset=9
    local.get 0
    i32.const 0
    i32.store)
  (func $_ZN4core3fmt9Formatter25debug_tuple_field1_finish17h5f6e45e01c2a3168E (type 12) (param i32 i32 i32 i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 5
    global.set $__stack_pointer
    local.get 5
    local.get 0
    i32.load
    local.get 1
    local.get 2
    local.get 0
    i32.load offset=4
    i32.load offset=12
    call_indirect (type 7)
    i32.store8 offset=8
    local.get 5
    local.get 0
    i32.store offset=4
    local.get 5
    local.get 2
    i32.eqz
    i32.store8 offset=9
    local.get 5
    i32.const 0
    i32.store
    local.get 5
    local.get 3
    local.get 4
    call $_ZN4core3fmt8builders10DebugTuple5field17hbd92e7b522abdb91E
    local.set 0
    local.get 5
    i32.load8_u offset=8
    local.set 2
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load
        local.tee 1
        br_if 0 (;@2;)
        local.get 2
        i32.const 255
        i32.and
        i32.const 0
        i32.ne
        local.set 0
        br 1 (;@1;)
      end
      i32.const 1
      local.set 0
      local.get 2
      i32.const 255
      i32.and
      br_if 0 (;@1;)
      local.get 5
      i32.load offset=4
      local.set 2
      block  ;; label = @2
        local.get 1
        i32.const 1
        i32.ne
        br_if 0 (;@2;)
        local.get 5
        i32.load8_u offset=9
        i32.const 255
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        i32.load8_u offset=24
        i32.const 4
        i32.and
        br_if 0 (;@2;)
        i32.const 1
        local.set 0
        local.get 2
        i32.load
        i32.const 1055452
        i32.const 1
        local.get 2
        i32.load offset=4
        i32.load offset=12
        call_indirect (type 7)
        br_if 1 (;@1;)
      end
      local.get 2
      i32.load
      i32.const 1054992
      i32.const 1
      local.get 2
      i32.load offset=4
      i32.load offset=12
      call_indirect (type 7)
      local.set 0
    end
    local.get 5
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 0)
  (func $_ZN4core3fmt9Formatter25debug_tuple_field2_finish17h6b7a77d622adf9f6E (type 18) (param i32 i32 i32 i32 i32 i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 7
    global.set $__stack_pointer
    local.get 7
    local.get 0
    i32.load
    local.get 1
    local.get 2
    local.get 0
    i32.load offset=4
    i32.load offset=12
    call_indirect (type 7)
    i32.store8 offset=8
    local.get 7
    local.get 0
    i32.store offset=4
    local.get 7
    local.get 2
    i32.eqz
    i32.store8 offset=9
    local.get 7
    i32.const 0
    i32.store
    local.get 7
    local.get 3
    local.get 4
    call $_ZN4core3fmt8builders10DebugTuple5field17hbd92e7b522abdb91E
    local.get 5
    local.get 6
    call $_ZN4core3fmt8builders10DebugTuple5field17hbd92e7b522abdb91E
    local.set 0
    local.get 7
    i32.load8_u offset=8
    local.set 2
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load
        local.tee 1
        br_if 0 (;@2;)
        local.get 2
        i32.const 255
        i32.and
        i32.const 0
        i32.ne
        local.set 0
        br 1 (;@1;)
      end
      i32.const 1
      local.set 0
      local.get 2
      i32.const 255
      i32.and
      br_if 0 (;@1;)
      local.get 7
      i32.load offset=4
      local.set 2
      block  ;; label = @2
        local.get 1
        i32.const 1
        i32.ne
        br_if 0 (;@2;)
        local.get 7
        i32.load8_u offset=9
        i32.const 255
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        i32.load8_u offset=24
        i32.const 4
        i32.and
        br_if 0 (;@2;)
        i32.const 1
        local.set 0
        local.get 2
        i32.load
        i32.const 1055452
        i32.const 1
        local.get 2
        i32.load offset=4
        i32.load offset=12
        call_indirect (type 7)
        br_if 1 (;@1;)
      end
      local.get 2
      i32.load
      i32.const 1054992
      i32.const 1
      local.get 2
      i32.load offset=4
      i32.load offset=12
      call_indirect (type 7)
      local.set 0
    end
    local.get 7
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 0)
  (func $_ZN4core3fmt9Formatter10debug_list17h89c83d2b4d2f51e2E (type 6) (param i32 i32)
    (local i32)
    local.get 1
    i32.load
    i32.const 1055105
    i32.const 1
    local.get 1
    i32.load offset=4
    i32.load offset=12
    call_indirect (type 7)
    local.set 2
    local.get 0
    i32.const 0
    i32.store8 offset=5
    local.get 0
    local.get 2
    i32.store8 offset=4
    local.get 0
    local.get 1
    i32.store)
  (func $_ZN43_$LT$bool$u20$as$u20$core..fmt..Display$GT$3fmt17h3b22d81fd764e498E (type 2) (param i32 i32) (result i32)
    block  ;; label = @1
      local.get 0
      i32.load8_u
      br_if 0 (;@1;)
      local.get 1
      i32.const 1055775
      i32.const 5
      call $_ZN4core3fmt9Formatter3pad17hea5c16334677f526E
      return
    end
    local.get 1
    i32.const 1055771
    i32.const 4
    call $_ZN4core3fmt9Formatter3pad17hea5c16334677f526E)
  (func $_ZN40_$LT$str$u20$as$u20$core..fmt..Debug$GT$3fmt17hab99d706fe6e9e7bE (type 7) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 2
          i32.load
          local.tee 3
          i32.const 34
          local.get 2
          i32.load offset=4
          local.tee 4
          i32.load offset=16
          local.tee 5
          call_indirect (type 2)
          br_if 0 (;@3;)
          block  ;; label = @4
            block  ;; label = @5
              local.get 1
              br_if 0 (;@5;)
              i32.const 0
              local.set 6
              br 1 (;@4;)
            end
            local.get 0
            local.get 1
            i32.add
            local.set 7
            i32.const 0
            local.set 6
            local.get 0
            local.set 8
            i32.const 0
            local.set 9
            block  ;; label = @5
              loop  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 8
                    i32.load8_s
                    local.tee 2
                    i32.const -1
                    i32.le_s
                    br_if 0 (;@8;)
                    local.get 8
                    i32.const 1
                    i32.add
                    local.set 10
                    local.get 2
                    i32.const 255
                    i32.and
                    local.set 11
                    br 1 (;@7;)
                  end
                  local.get 8
                  i32.load8_u offset=1
                  i32.const 63
                  i32.and
                  local.set 12
                  local.get 2
                  i32.const 31
                  i32.and
                  local.set 13
                  block  ;; label = @8
                    local.get 2
                    i32.const -33
                    i32.gt_u
                    br_if 0 (;@8;)
                    local.get 13
                    i32.const 6
                    i32.shl
                    local.get 12
                    i32.or
                    local.set 11
                    local.get 8
                    i32.const 2
                    i32.add
                    local.set 10
                    br 1 (;@7;)
                  end
                  local.get 12
                  i32.const 6
                  i32.shl
                  local.get 8
                  i32.load8_u offset=2
                  i32.const 63
                  i32.and
                  i32.or
                  local.set 12
                  local.get 8
                  i32.const 3
                  i32.add
                  local.set 10
                  block  ;; label = @8
                    local.get 2
                    i32.const -16
                    i32.ge_u
                    br_if 0 (;@8;)
                    local.get 12
                    local.get 13
                    i32.const 12
                    i32.shl
                    i32.or
                    local.set 11
                    br 1 (;@7;)
                  end
                  local.get 12
                  i32.const 6
                  i32.shl
                  local.get 10
                  i32.load8_u
                  i32.const 63
                  i32.and
                  i32.or
                  local.get 13
                  i32.const 18
                  i32.shl
                  i32.const 1835008
                  i32.and
                  i32.or
                  local.tee 11
                  i32.const 1114112
                  i32.eq
                  br_if 2 (;@5;)
                  local.get 8
                  i32.const 4
                  i32.add
                  local.set 10
                end
                i32.const 1114114
                local.set 2
                i32.const 48
                local.set 14
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              block  ;; label = @14
                                block  ;; label = @15
                                  local.get 11
                                  br_table 8 (;@7;) 1 (;@14;) 1 (;@14;) 1 (;@14;) 1 (;@14;) 1 (;@14;) 1 (;@14;) 1 (;@14;) 1 (;@14;) 2 (;@13;) 4 (;@11;) 1 (;@14;) 1 (;@14;) 3 (;@12;) 1 (;@14;) 1 (;@14;) 1 (;@14;) 1 (;@14;) 1 (;@14;) 1 (;@14;) 1 (;@14;) 1 (;@14;) 1 (;@14;) 1 (;@14;) 1 (;@14;) 1 (;@14;) 1 (;@14;) 1 (;@14;) 1 (;@14;) 1 (;@14;) 1 (;@14;) 1 (;@14;) 1 (;@14;) 1 (;@14;) 5 (;@10;) 0 (;@15;)
                                end
                                local.get 11
                                i32.const 92
                                i32.eq
                                br_if 4 (;@10;)
                              end
                              local.get 11
                              call $_ZN4core7unicode12unicode_data15grapheme_extend6lookup17hc65d17b6061336f0E
                              i32.eqz
                              br_if 4 (;@9;)
                              local.get 11
                              i32.const 1
                              i32.or
                              i32.clz
                              i32.const 2
                              i32.shr_u
                              i32.const 7
                              i32.xor
                              local.set 14
                              br 5 (;@8;)
                            end
                            i32.const 116
                            local.set 14
                            br 5 (;@7;)
                          end
                          i32.const 114
                          local.set 14
                          br 4 (;@7;)
                        end
                        i32.const 110
                        local.set 14
                        br 3 (;@7;)
                      end
                      local.get 11
                      local.set 14
                      br 2 (;@7;)
                    end
                    i32.const 1114113
                    local.set 2
                    local.get 11
                    local.set 14
                    local.get 11
                    call $_ZN4core7unicode9printable12is_printable17h7eb79a8fee4532b1E
                    br_if 1 (;@7;)
                    local.get 11
                    i32.const 1
                    i32.or
                    i32.clz
                    i32.const 2
                    i32.shr_u
                    i32.const 7
                    i32.xor
                    local.set 14
                  end
                  local.get 11
                  local.set 2
                end
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 2
                    i32.const -1114112
                    i32.add
                    local.tee 12
                    i32.const 3
                    local.get 12
                    i32.const 3
                    i32.lt_u
                    select
                    i32.const 1
                    i32.eq
                    br_if 0 (;@8;)
                    local.get 9
                    local.get 6
                    i32.lt_u
                    br_if 1 (;@7;)
                    block  ;; label = @9
                      local.get 6
                      i32.eqz
                      br_if 0 (;@9;)
                      block  ;; label = @10
                        local.get 6
                        local.get 1
                        i32.lt_u
                        br_if 0 (;@10;)
                        local.get 6
                        local.get 1
                        i32.eq
                        br_if 1 (;@9;)
                        br 3 (;@7;)
                      end
                      local.get 0
                      local.get 6
                      i32.add
                      i32.load8_s
                      i32.const -64
                      i32.lt_s
                      br_if 2 (;@7;)
                    end
                    block  ;; label = @9
                      local.get 9
                      i32.eqz
                      br_if 0 (;@9;)
                      block  ;; label = @10
                        local.get 9
                        local.get 1
                        i32.lt_u
                        br_if 0 (;@10;)
                        local.get 9
                        local.get 1
                        i32.ne
                        br_if 3 (;@7;)
                        br 1 (;@9;)
                      end
                      local.get 0
                      local.get 9
                      i32.add
                      i32.load8_s
                      i32.const -65
                      i32.le_s
                      br_if 2 (;@7;)
                    end
                    block  ;; label = @9
                      local.get 3
                      local.get 0
                      local.get 6
                      i32.add
                      local.get 9
                      local.get 6
                      i32.sub
                      local.get 4
                      i32.load offset=12
                      call_indirect (type 7)
                      i32.eqz
                      br_if 0 (;@9;)
                      i32.const 1
                      return
                    end
                    i32.const 5
                    local.set 15
                    block  ;; label = @9
                      block  ;; label = @10
                        loop  ;; label = @11
                          local.get 15
                          local.set 16
                          local.get 2
                          local.set 12
                          i32.const 1114113
                          local.set 2
                          i32.const 92
                          local.set 6
                          block  ;; label = @12
                            block  ;; label = @13
                              block  ;; label = @14
                                block  ;; label = @15
                                  block  ;; label = @16
                                    local.get 12
                                    i32.const -1114112
                                    i32.add
                                    local.tee 13
                                    i32.const 3
                                    local.get 13
                                    i32.const 3
                                    i32.lt_u
                                    select
                                    br_table 2 (;@14;) 1 (;@15;) 4 (;@12;) 0 (;@16;) 2 (;@14;)
                                  end
                                  i32.const 0
                                  local.set 15
                                  i32.const 125
                                  local.set 6
                                  local.get 12
                                  local.set 2
                                  block  ;; label = @16
                                    block  ;; label = @17
                                      block  ;; label = @18
                                        local.get 16
                                        i32.const 255
                                        i32.and
                                        br_table 4 (;@14;) 6 (;@12;) 5 (;@13;) 0 (;@18;) 1 (;@17;) 2 (;@16;) 4 (;@14;)
                                      end
                                      i32.const 2
                                      local.set 15
                                      i32.const 123
                                      local.set 6
                                      local.get 12
                                      local.set 2
                                      br 5 (;@12;)
                                    end
                                    i32.const 3
                                    local.set 15
                                    i32.const 117
                                    local.set 6
                                    local.get 12
                                    local.set 2
                                    br 4 (;@12;)
                                  end
                                  i32.const 4
                                  local.set 15
                                  i32.const 92
                                  local.set 6
                                  local.get 12
                                  local.set 2
                                  br 3 (;@12;)
                                end
                                i32.const 1114112
                                local.set 2
                                local.get 14
                                local.set 6
                                local.get 16
                                local.set 15
                                local.get 14
                                i32.const 1114112
                                i32.ne
                                br_if 2 (;@12;)
                              end
                              i32.const 1
                              local.set 2
                              local.get 11
                              i32.const 128
                              i32.lt_u
                              br_if 4 (;@9;)
                              i32.const 2
                              local.set 2
                              local.get 11
                              i32.const 2047
                              i32.gt_u
                              br_if 3 (;@10;)
                              br 4 (;@9;)
                            end
                            local.get 16
                            i32.const 1
                            local.get 14
                            select
                            local.set 15
                            i32.const 48
                            i32.const 87
                            local.get 12
                            local.get 14
                            i32.const 2
                            i32.shl
                            i32.shr_u
                            i32.const 15
                            i32.and
                            local.tee 2
                            i32.const 10
                            i32.lt_u
                            select
                            local.get 2
                            i32.add
                            local.set 6
                            local.get 12
                            local.set 2
                            i32.const 0
                            local.get 14
                            i32.const -1
                            i32.add
                            local.tee 12
                            local.get 12
                            local.get 14
                            i32.gt_u
                            select
                            local.set 14
                          end
                          local.get 3
                          local.get 6
                          local.get 5
                          call_indirect (type 2)
                          i32.eqz
                          br_if 0 (;@11;)
                        end
                        i32.const 1
                        return
                      end
                      i32.const 3
                      i32.const 4
                      local.get 11
                      i32.const 65536
                      i32.lt_u
                      select
                      local.set 2
                    end
                    local.get 2
                    local.get 9
                    i32.add
                    local.set 6
                  end
                  local.get 9
                  local.get 8
                  i32.sub
                  local.get 10
                  i32.add
                  local.set 9
                  local.get 10
                  local.set 8
                  local.get 10
                  local.get 7
                  i32.ne
                  br_if 1 (;@6;)
                  br 2 (;@5;)
                end
              end
              local.get 0
              local.get 1
              local.get 6
              local.get 9
              i32.const 1055780
              call $_ZN4core3str16slice_error_fail17hd8efdeca0603f5c1E
              unreachable
            end
            block  ;; label = @5
              local.get 6
              br_if 0 (;@5;)
              i32.const 0
              local.set 6
              br 1 (;@4;)
            end
            block  ;; label = @5
              local.get 6
              local.get 1
              i32.lt_u
              br_if 0 (;@5;)
              local.get 6
              local.get 1
              i32.eq
              br_if 1 (;@4;)
              br 4 (;@1;)
            end
            local.get 0
            local.get 6
            i32.add
            i32.load8_s
            i32.const -65
            i32.le_s
            br_if 3 (;@1;)
          end
          local.get 3
          local.get 0
          local.get 6
          i32.add
          local.get 1
          local.get 6
          i32.sub
          local.get 4
          i32.load offset=12
          call_indirect (type 7)
          i32.eqz
          br_if 1 (;@2;)
        end
        i32.const 1
        return
      end
      local.get 3
      i32.const 34
      local.get 5
      call_indirect (type 2)
      return
    end
    local.get 0
    local.get 1
    local.get 6
    local.get 1
    i32.const 1055796
    call $_ZN4core3str16slice_error_fail17hd8efdeca0603f5c1E
    unreachable)
  (func $_ZN4core7unicode12unicode_data15grapheme_extend6lookup17hc65d17b6061336f0E (type 4) (param i32) (result i32)
    (local i32 i32 i32 i32 i32)
    local.get 0
    i32.const 11
    i32.shl
    local.set 1
    i32.const 0
    local.set 2
    i32.const 33
    local.set 3
    i32.const 33
    local.set 4
    block  ;; label = @1
      block  ;; label = @2
        loop  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              i32.const -1
              local.get 3
              i32.const 1
              i32.shr_u
              local.get 2
              i32.add
              local.tee 5
              i32.const 2
              i32.shl
              i32.const 1058160
              i32.add
              i32.load
              i32.const 11
              i32.shl
              local.tee 3
              local.get 1
              i32.ne
              local.get 3
              local.get 1
              i32.lt_u
              select
              local.tee 3
              i32.const 1
              i32.ne
              br_if 0 (;@5;)
              local.get 5
              local.set 4
              br 1 (;@4;)
            end
            local.get 3
            i32.const 255
            i32.and
            i32.const 255
            i32.ne
            br_if 2 (;@2;)
            local.get 5
            i32.const 1
            i32.add
            local.set 2
          end
          local.get 4
          local.get 2
          i32.sub
          local.set 3
          local.get 4
          local.get 2
          i32.gt_u
          br_if 0 (;@3;)
          br 2 (;@1;)
        end
      end
      local.get 5
      i32.const 1
      i32.add
      local.set 2
    end
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        i32.const 32
        i32.gt_u
        br_if 0 (;@2;)
        local.get 2
        i32.const 2
        i32.shl
        local.tee 1
        i32.const 1058160
        i32.add
        i32.load
        i32.const 21
        i32.shr_u
        local.set 4
        i32.const 727
        local.set 5
        block  ;; label = @3
          block  ;; label = @4
            local.get 2
            i32.const 32
            i32.eq
            br_if 0 (;@4;)
            local.get 1
            i32.const 1058164
            i32.add
            i32.load
            i32.const 21
            i32.shr_u
            local.set 5
            local.get 2
            br_if 0 (;@4;)
            i32.const 0
            local.set 1
            br 1 (;@3;)
          end
          local.get 1
          i32.const 1058156
          i32.add
          i32.load
          i32.const 2097151
          i32.and
          local.set 3
          i32.const 1
          local.set 1
        end
        local.get 5
        local.get 4
        i32.const -1
        i32.xor
        i32.add
        i32.eqz
        br_if 1 (;@1;)
        i32.const 0
        local.set 2
        local.get 0
        local.get 3
        i32.const 0
        local.get 1
        select
        i32.sub
        local.set 3
        local.get 4
        i32.const 727
        local.get 4
        i32.const 727
        i32.gt_u
        select
        local.set 1
        local.get 5
        i32.const -1
        i32.add
        local.set 5
        loop  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 1
              local.get 4
              i32.eq
              br_if 0 (;@5;)
              local.get 2
              local.get 4
              i32.const 1058292
              i32.add
              i32.load8_u
              i32.add
              local.tee 2
              local.get 3
              i32.le_u
              br_if 1 (;@4;)
              br 4 (;@1;)
            end
            local.get 1
            i32.const 727
            i32.const 1058144
            call $_ZN4core9panicking18panic_bounds_check17h575df7fe5ecb957dE
            unreachable
          end
          local.get 5
          local.get 4
          i32.const 1
          i32.add
          local.tee 4
          i32.ne
          br_if 0 (;@3;)
        end
        local.get 5
        local.set 4
        br 1 (;@1;)
      end
      local.get 2
      i32.const 33
      i32.const 1058128
      call $_ZN4core9panicking18panic_bounds_check17h575df7fe5ecb957dE
      unreachable
    end
    local.get 4
    i32.const 1
    i32.and)
  (func $_ZN4core7unicode9printable12is_printable17h7eb79a8fee4532b1E (type 4) (param i32) (result i32)
    (local i32)
    block  ;; label = @1
      local.get 0
      i32.const 32
      i32.ge_u
      br_if 0 (;@1;)
      i32.const 0
      return
    end
    i32.const 1
    local.set 1
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.const 127
        i32.lt_u
        br_if 0 (;@2;)
        local.get 0
        i32.const 65536
        i32.lt_u
        br_if 1 (;@1;)
        block  ;; label = @3
          block  ;; label = @4
            local.get 0
            i32.const 131072
            i32.lt_u
            br_if 0 (;@4;)
            block  ;; label = @5
              local.get 0
              i32.const -205744
              i32.add
              i32.const 712016
              i32.ge_u
              br_if 0 (;@5;)
              i32.const 0
              return
            end
            block  ;; label = @5
              local.get 0
              i32.const -201547
              i32.add
              i32.const 5
              i32.ge_u
              br_if 0 (;@5;)
              i32.const 0
              return
            end
            block  ;; label = @5
              local.get 0
              i32.const -195102
              i32.add
              i32.const 1506
              i32.ge_u
              br_if 0 (;@5;)
              i32.const 0
              return
            end
            block  ;; label = @5
              local.get 0
              i32.const -191457
              i32.add
              i32.const 3103
              i32.ge_u
              br_if 0 (;@5;)
              i32.const 0
              return
            end
            block  ;; label = @5
              local.get 0
              i32.const -183970
              i32.add
              i32.const 14
              i32.ge_u
              br_if 0 (;@5;)
              i32.const 0
              return
            end
            block  ;; label = @5
              local.get 0
              i32.const -2
              i32.and
              i32.const 178206
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              return
            end
            local.get 0
            i32.const -32
            i32.and
            i32.const 173792
            i32.ne
            br_if 1 (;@3;)
            i32.const 0
            return
          end
          local.get 0
          i32.const 1057278
          i32.const 44
          i32.const 1057366
          i32.const 196
          i32.const 1057562
          i32.const 450
          call $_ZN4core7unicode9printable5check17hac613972f1363ee5E
          return
        end
        i32.const 0
        local.set 1
        local.get 0
        i32.const -177978
        i32.add
        i32.const 6
        i32.lt_u
        br_if 0 (;@2;)
        local.get 0
        i32.const -1114112
        i32.add
        i32.const -196112
        i32.lt_u
        local.set 1
      end
      local.get 1
      return
    end
    local.get 0
    i32.const 1056608
    i32.const 40
    i32.const 1056688
    i32.const 287
    i32.const 1056975
    i32.const 303
    call $_ZN4core7unicode9printable5check17hac613972f1363ee5E)
  (func $_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17haca8e2a25628fb8aE (type 7) (param i32 i32 i32) (result i32)
    local.get 2
    local.get 0
    local.get 1
    call $_ZN4core3fmt9Formatter3pad17hea5c16334677f526E)
  (func $_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17hdf2bf09c0dbb3fa6E (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    i32.const 1
    local.set 2
    block  ;; label = @1
      local.get 1
      i32.load
      local.tee 3
      i32.const 39
      local.get 1
      i32.load offset=4
      i32.load offset=16
      local.tee 4
      call_indirect (type 2)
      br_if 0 (;@1;)
      i32.const 1114114
      local.set 1
      i32.const 48
      local.set 5
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 0
                        i32.load
                        local.tee 2
                        br_table 8 (;@2;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 2 (;@8;) 4 (;@6;) 1 (;@9;) 1 (;@9;) 3 (;@7;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 5 (;@5;) 0 (;@10;)
                      end
                      local.get 2
                      i32.const 92
                      i32.eq
                      br_if 4 (;@5;)
                    end
                    local.get 2
                    call $_ZN4core7unicode12unicode_data15grapheme_extend6lookup17hc65d17b6061336f0E
                    i32.eqz
                    br_if 4 (;@4;)
                    local.get 2
                    i32.const 1
                    i32.or
                    i32.clz
                    i32.const 2
                    i32.shr_u
                    i32.const 7
                    i32.xor
                    local.set 5
                    br 5 (;@3;)
                  end
                  i32.const 116
                  local.set 5
                  br 5 (;@2;)
                end
                i32.const 114
                local.set 5
                br 4 (;@2;)
              end
              i32.const 110
              local.set 5
              br 3 (;@2;)
            end
            local.get 2
            local.set 5
            br 2 (;@2;)
          end
          i32.const 1114113
          local.set 1
          block  ;; label = @4
            local.get 2
            call $_ZN4core7unicode9printable12is_printable17h7eb79a8fee4532b1E
            i32.eqz
            br_if 0 (;@4;)
            local.get 2
            local.set 5
            br 2 (;@2;)
          end
          local.get 2
          i32.const 1
          i32.or
          i32.clz
          i32.const 2
          i32.shr_u
          i32.const 7
          i32.xor
          local.set 5
        end
        local.get 2
        local.set 1
      end
      i32.const 5
      local.set 6
      loop  ;; label = @2
        local.get 6
        local.set 7
        local.get 1
        local.set 0
        i32.const 1114113
        local.set 1
        i32.const 92
        local.set 2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 0
                  i32.const -1114112
                  i32.add
                  local.tee 8
                  i32.const 3
                  local.get 8
                  i32.const 3
                  i32.lt_u
                  select
                  br_table 2 (;@5;) 1 (;@6;) 4 (;@3;) 0 (;@7;) 2 (;@5;)
                end
                i32.const 0
                local.set 6
                i32.const 125
                local.set 2
                local.get 0
                local.set 1
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 7
                      i32.const 255
                      i32.and
                      br_table 4 (;@5;) 6 (;@3;) 5 (;@4;) 0 (;@9;) 1 (;@8;) 2 (;@7;) 4 (;@5;)
                    end
                    i32.const 2
                    local.set 6
                    i32.const 123
                    local.set 2
                    local.get 0
                    local.set 1
                    br 5 (;@3;)
                  end
                  i32.const 3
                  local.set 6
                  i32.const 117
                  local.set 2
                  local.get 0
                  local.set 1
                  br 4 (;@3;)
                end
                i32.const 4
                local.set 6
                i32.const 92
                local.set 2
                local.get 0
                local.set 1
                br 3 (;@3;)
              end
              i32.const 1114112
              local.set 1
              local.get 5
              local.set 2
              local.get 7
              local.set 6
              local.get 5
              i32.const 1114112
              i32.ne
              br_if 2 (;@3;)
            end
            local.get 3
            i32.const 39
            local.get 4
            call_indirect (type 2)
            local.set 2
            br 3 (;@1;)
          end
          local.get 7
          i32.const 1
          local.get 5
          select
          local.set 6
          i32.const 48
          i32.const 87
          local.get 0
          local.get 5
          i32.const 2
          i32.shl
          i32.shr_u
          i32.const 15
          i32.and
          local.tee 1
          i32.const 10
          i32.lt_u
          select
          local.get 1
          i32.add
          local.set 2
          local.get 0
          local.set 1
          i32.const 0
          local.get 5
          i32.const -1
          i32.add
          local.tee 0
          local.get 0
          local.get 5
          i32.gt_u
          select
          local.set 5
        end
        local.get 3
        local.get 2
        local.get 4
        call_indirect (type 2)
        i32.eqz
        br_if 0 (;@2;)
      end
      i32.const 1
      return
    end
    local.get 2)
  (func $_ZN4core5slice5index29slice_start_index_len_fail_rt17h597176734be4f636E (type 5) (param i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    local.get 1
    i32.store offset=4
    local.get 3
    local.get 0
    i32.store
    local.get 3
    i32.const 8
    i32.add
    i32.const 12
    i32.add
    i32.const 2
    i32.store
    local.get 3
    i32.const 28
    i32.add
    i32.const 2
    i32.store
    local.get 3
    i32.const 32
    i32.add
    i32.const 12
    i32.add
    i32.const 11
    i32.store
    local.get 3
    i32.const 1055912
    i32.store offset=16
    local.get 3
    i32.const 0
    i32.store offset=8
    local.get 3
    i32.const 11
    i32.store offset=36
    local.get 3
    local.get 3
    i32.const 32
    i32.add
    i32.store offset=24
    local.get 3
    local.get 3
    i32.const 4
    i32.add
    i32.store offset=40
    local.get 3
    local.get 3
    i32.store offset=32
    local.get 3
    i32.const 8
    i32.add
    local.get 2
    call $_ZN4core9panicking9panic_fmt17h0724b0a601bd3679E
    unreachable)
  (func $_ZN4core5slice5index27slice_end_index_len_fail_rt17hd780f3b79b77f071E (type 5) (param i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    local.get 1
    i32.store offset=4
    local.get 3
    local.get 0
    i32.store
    local.get 3
    i32.const 8
    i32.add
    i32.const 12
    i32.add
    i32.const 2
    i32.store
    local.get 3
    i32.const 28
    i32.add
    i32.const 2
    i32.store
    local.get 3
    i32.const 32
    i32.add
    i32.const 12
    i32.add
    i32.const 11
    i32.store
    local.get 3
    i32.const 1055944
    i32.store offset=16
    local.get 3
    i32.const 0
    i32.store offset=8
    local.get 3
    i32.const 11
    i32.store offset=36
    local.get 3
    local.get 3
    i32.const 32
    i32.add
    i32.store offset=24
    local.get 3
    local.get 3
    i32.const 4
    i32.add
    i32.store offset=40
    local.get 3
    local.get 3
    i32.store offset=32
    local.get 3
    i32.const 8
    i32.add
    local.get 2
    call $_ZN4core9panicking9panic_fmt17h0724b0a601bd3679E
    unreachable)
  (func $_ZN4core5slice5index25slice_index_order_fail_rt17h2fa58c5f8613fe67E (type 5) (param i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    local.get 1
    i32.store offset=4
    local.get 3
    local.get 0
    i32.store
    local.get 3
    i32.const 8
    i32.add
    i32.const 12
    i32.add
    i32.const 2
    i32.store
    local.get 3
    i32.const 28
    i32.add
    i32.const 2
    i32.store
    local.get 3
    i32.const 32
    i32.add
    i32.const 12
    i32.add
    i32.const 11
    i32.store
    local.get 3
    i32.const 1055996
    i32.store offset=16
    local.get 3
    i32.const 0
    i32.store offset=8
    local.get 3
    i32.const 11
    i32.store offset=36
    local.get 3
    local.get 3
    i32.const 32
    i32.add
    i32.store offset=24
    local.get 3
    local.get 3
    i32.const 4
    i32.add
    i32.store offset=40
    local.get 3
    local.get 3
    i32.store offset=32
    local.get 3
    i32.const 8
    i32.add
    local.get 2
    call $_ZN4core9panicking9panic_fmt17h0724b0a601bd3679E
    unreachable)
  (func $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i8$GT$3fmt17h0f0b80b3697c42f8E (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 128
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 0
    i32.load8_u
    local.set 3
    i32.const 0
    local.set 0
    loop  ;; label = @1
      local.get 2
      local.get 0
      i32.add
      i32.const 127
      i32.add
      i32.const 48
      i32.const 55
      local.get 3
      i32.const 15
      i32.and
      local.tee 4
      i32.const 10
      i32.lt_u
      select
      local.get 4
      i32.add
      i32.store8
      local.get 0
      i32.const -1
      i32.add
      local.set 0
      local.get 3
      i32.const 255
      i32.and
      local.tee 4
      i32.const 4
      i32.shr_u
      local.set 3
      local.get 4
      i32.const 15
      i32.gt_u
      br_if 0 (;@1;)
    end
    block  ;; label = @1
      local.get 0
      i32.const 128
      i32.add
      local.tee 3
      i32.const 129
      i32.lt_u
      br_if 0 (;@1;)
      local.get 3
      i32.const 128
      i32.const 1055500
      call $_ZN4core5slice5index26slice_start_index_len_fail17h12ea13b84dc5aa33E
      unreachable
    end
    local.get 1
    i32.const 1
    i32.const 1055516
    i32.const 2
    local.get 2
    local.get 0
    i32.add
    i32.const 128
    i32.add
    i32.const 0
    local.get 0
    i32.sub
    call $_ZN4core3fmt9Formatter12pad_integral17h2602ad621a1201d6E
    local.set 0
    local.get 2
    i32.const 128
    i32.add
    global.set $__stack_pointer
    local.get 0)
  (func $_ZN4core3str19slice_error_fail_rt17h75fd19d0e302092fE (type 13) (param i32 i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 112
    i32.sub
    local.tee 5
    global.set $__stack_pointer
    local.get 5
    local.get 3
    i32.store offset=12
    local.get 5
    local.get 2
    i32.store offset=8
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 1
          i32.const 257
          i32.lt_u
          br_if 0 (;@3;)
          i32.const 256
          local.set 6
          block  ;; label = @4
            local.get 0
            i32.load8_s offset=256
            i32.const -65
            i32.gt_s
            br_if 0 (;@4;)
            i32.const 255
            local.set 6
            local.get 0
            i32.load8_s offset=255
            i32.const -65
            i32.gt_s
            br_if 0 (;@4;)
            i32.const 254
            local.set 6
            local.get 0
            i32.load8_s offset=254
            i32.const -65
            i32.gt_s
            br_if 0 (;@4;)
            i32.const 253
            local.set 6
          end
          local.get 0
          local.get 6
          i32.add
          i32.load8_s
          i32.const -65
          i32.le_s
          br_if 1 (;@2;)
          local.get 5
          local.get 6
          i32.store offset=20
          local.get 5
          local.get 0
          i32.store offset=16
          i32.const 5
          local.set 6
          i32.const 1056268
          local.set 7
          br 2 (;@1;)
        end
        local.get 5
        local.get 1
        i32.store offset=20
        local.get 5
        local.get 0
        i32.store offset=16
        i32.const 0
        local.set 6
        i32.const 1054992
        local.set 7
        br 1 (;@1;)
      end
      local.get 0
      local.get 1
      i32.const 0
      local.get 6
      local.get 4
      call $_ZN4core3str16slice_error_fail17hd8efdeca0603f5c1E
      unreachable
    end
    local.get 5
    local.get 6
    i32.store offset=28
    local.get 5
    local.get 7
    i32.store offset=24
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 2
            local.get 1
            i32.gt_u
            local.tee 6
            br_if 0 (;@4;)
            local.get 3
            local.get 1
            i32.gt_u
            br_if 0 (;@4;)
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 2
                    local.get 3
                    i32.gt_u
                    br_if 0 (;@8;)
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 2
                        i32.eqz
                        br_if 0 (;@10;)
                        block  ;; label = @11
                          local.get 2
                          local.get 1
                          i32.lt_u
                          br_if 0 (;@11;)
                          local.get 2
                          local.get 1
                          i32.eq
                          br_if 1 (;@10;)
                          br 2 (;@9;)
                        end
                        local.get 0
                        local.get 2
                        i32.add
                        i32.load8_s
                        i32.const -64
                        i32.lt_s
                        br_if 1 (;@9;)
                      end
                      local.get 3
                      local.set 2
                    end
                    local.get 5
                    local.get 2
                    i32.store offset=32
                    local.get 1
                    local.set 3
                    block  ;; label = @9
                      local.get 2
                      local.get 1
                      i32.ge_u
                      br_if 0 (;@9;)
                      local.get 2
                      i32.const 1
                      i32.add
                      local.tee 6
                      i32.const 0
                      local.get 2
                      i32.const -3
                      i32.add
                      local.tee 3
                      local.get 3
                      local.get 2
                      i32.gt_u
                      select
                      local.tee 3
                      i32.lt_u
                      br_if 6 (;@3;)
                      block  ;; label = @10
                        local.get 3
                        local.get 6
                        i32.eq
                        br_if 0 (;@10;)
                        local.get 0
                        local.get 6
                        i32.add
                        local.get 0
                        local.get 3
                        i32.add
                        local.tee 8
                        i32.sub
                        local.set 6
                        block  ;; label = @11
                          local.get 0
                          local.get 2
                          i32.add
                          local.tee 9
                          i32.load8_s
                          i32.const -65
                          i32.le_s
                          br_if 0 (;@11;)
                          local.get 6
                          i32.const -1
                          i32.add
                          local.set 7
                          br 1 (;@10;)
                        end
                        local.get 3
                        local.get 2
                        i32.eq
                        br_if 0 (;@10;)
                        block  ;; label = @11
                          local.get 9
                          i32.const -1
                          i32.add
                          local.tee 2
                          i32.load8_s
                          i32.const -65
                          i32.le_s
                          br_if 0 (;@11;)
                          local.get 6
                          i32.const -2
                          i32.add
                          local.set 7
                          br 1 (;@10;)
                        end
                        local.get 8
                        local.get 2
                        i32.eq
                        br_if 0 (;@10;)
                        block  ;; label = @11
                          local.get 2
                          i32.const -1
                          i32.add
                          local.tee 2
                          i32.load8_s
                          i32.const -65
                          i32.le_s
                          br_if 0 (;@11;)
                          local.get 6
                          i32.const -3
                          i32.add
                          local.set 7
                          br 1 (;@10;)
                        end
                        local.get 8
                        local.get 2
                        i32.eq
                        br_if 0 (;@10;)
                        block  ;; label = @11
                          local.get 2
                          i32.const -1
                          i32.add
                          local.tee 2
                          i32.load8_s
                          i32.const -65
                          i32.le_s
                          br_if 0 (;@11;)
                          local.get 6
                          i32.const -4
                          i32.add
                          local.set 7
                          br 1 (;@10;)
                        end
                        local.get 8
                        local.get 2
                        i32.eq
                        br_if 0 (;@10;)
                        local.get 6
                        i32.const -5
                        i32.add
                        local.set 7
                      end
                      local.get 7
                      local.get 3
                      i32.add
                      local.set 3
                    end
                    block  ;; label = @9
                      local.get 3
                      i32.eqz
                      br_if 0 (;@9;)
                      block  ;; label = @10
                        local.get 3
                        local.get 1
                        i32.lt_u
                        br_if 0 (;@10;)
                        local.get 3
                        local.get 1
                        i32.eq
                        br_if 1 (;@9;)
                        br 9 (;@1;)
                      end
                      local.get 0
                      local.get 3
                      i32.add
                      i32.load8_s
                      i32.const -65
                      i32.le_s
                      br_if 8 (;@1;)
                    end
                    local.get 3
                    local.get 1
                    i32.eq
                    br_if 6 (;@2;)
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 0
                        local.get 3
                        i32.add
                        local.tee 2
                        i32.load8_s
                        local.tee 1
                        i32.const -1
                        i32.gt_s
                        br_if 0 (;@10;)
                        local.get 2
                        i32.load8_u offset=1
                        i32.const 63
                        i32.and
                        local.set 0
                        local.get 1
                        i32.const 31
                        i32.and
                        local.set 6
                        local.get 1
                        i32.const -33
                        i32.gt_u
                        br_if 1 (;@9;)
                        local.get 6
                        i32.const 6
                        i32.shl
                        local.get 0
                        i32.or
                        local.set 2
                        br 4 (;@6;)
                      end
                      local.get 5
                      local.get 1
                      i32.const 255
                      i32.and
                      i32.store offset=36
                      i32.const 1
                      local.set 1
                      br 4 (;@5;)
                    end
                    local.get 0
                    i32.const 6
                    i32.shl
                    local.get 2
                    i32.load8_u offset=2
                    i32.const 63
                    i32.and
                    i32.or
                    local.set 0
                    local.get 1
                    i32.const -16
                    i32.ge_u
                    br_if 1 (;@7;)
                    local.get 0
                    local.get 6
                    i32.const 12
                    i32.shl
                    i32.or
                    local.set 2
                    br 2 (;@6;)
                  end
                  local.get 5
                  i32.const 100
                  i32.add
                  i32.const 86
                  i32.store
                  local.get 5
                  i32.const 72
                  i32.add
                  i32.const 20
                  i32.add
                  i32.const 86
                  i32.store
                  local.get 5
                  i32.const 72
                  i32.add
                  i32.const 12
                  i32.add
                  i32.const 11
                  i32.store
                  local.get 5
                  i32.const 48
                  i32.add
                  i32.const 12
                  i32.add
                  i32.const 4
                  i32.store
                  local.get 5
                  i32.const 48
                  i32.add
                  i32.const 20
                  i32.add
                  i32.const 4
                  i32.store
                  local.get 5
                  i32.const 1056368
                  i32.store offset=56
                  local.get 5
                  i32.const 0
                  i32.store offset=48
                  local.get 5
                  i32.const 11
                  i32.store offset=76
                  local.get 5
                  local.get 5
                  i32.const 72
                  i32.add
                  i32.store offset=64
                  local.get 5
                  local.get 5
                  i32.const 24
                  i32.add
                  i32.store offset=96
                  local.get 5
                  local.get 5
                  i32.const 16
                  i32.add
                  i32.store offset=88
                  local.get 5
                  local.get 5
                  i32.const 12
                  i32.add
                  i32.store offset=80
                  local.get 5
                  local.get 5
                  i32.const 8
                  i32.add
                  i32.store offset=72
                  local.get 5
                  i32.const 48
                  i32.add
                  local.get 4
                  call $_ZN4core9panicking9panic_fmt17h0724b0a601bd3679E
                  unreachable
                end
                local.get 0
                i32.const 6
                i32.shl
                local.get 2
                i32.load8_u offset=3
                i32.const 63
                i32.and
                i32.or
                local.get 6
                i32.const 18
                i32.shl
                i32.const 1835008
                i32.and
                i32.or
                local.tee 2
                i32.const 1114112
                i32.eq
                br_if 4 (;@2;)
              end
              local.get 5
              local.get 2
              i32.store offset=36
              i32.const 1
              local.set 1
              local.get 2
              i32.const 128
              i32.lt_u
              br_if 0 (;@5;)
              i32.const 2
              local.set 1
              local.get 2
              i32.const 2047
              i32.le_u
              br_if 0 (;@5;)
              i32.const 3
              i32.const 4
              local.get 2
              i32.const 65536
              i32.lt_u
              select
              local.set 1
            end
            local.get 5
            local.get 3
            i32.store offset=40
            local.get 5
            local.get 1
            local.get 3
            i32.add
            i32.store offset=44
            local.get 5
            i32.const 48
            i32.add
            i32.const 12
            i32.add
            i32.const 5
            i32.store
            local.get 5
            i32.const 48
            i32.add
            i32.const 20
            i32.add
            i32.const 5
            i32.store
            local.get 5
            i32.const 108
            i32.add
            i32.const 86
            i32.store
            local.get 5
            i32.const 100
            i32.add
            i32.const 86
            i32.store
            local.get 5
            i32.const 72
            i32.add
            i32.const 20
            i32.add
            i32.const 89
            i32.store
            local.get 5
            i32.const 72
            i32.add
            i32.const 12
            i32.add
            i32.const 90
            i32.store
            local.get 5
            i32.const 1056452
            i32.store offset=56
            local.get 5
            i32.const 0
            i32.store offset=48
            local.get 5
            i32.const 11
            i32.store offset=76
            local.get 5
            local.get 5
            i32.const 72
            i32.add
            i32.store offset=64
            local.get 5
            local.get 5
            i32.const 24
            i32.add
            i32.store offset=104
            local.get 5
            local.get 5
            i32.const 16
            i32.add
            i32.store offset=96
            local.get 5
            local.get 5
            i32.const 40
            i32.add
            i32.store offset=88
            local.get 5
            local.get 5
            i32.const 36
            i32.add
            i32.store offset=80
            local.get 5
            local.get 5
            i32.const 32
            i32.add
            i32.store offset=72
            local.get 5
            i32.const 48
            i32.add
            local.get 4
            call $_ZN4core9panicking9panic_fmt17h0724b0a601bd3679E
            unreachable
          end
          local.get 5
          local.get 2
          local.get 3
          local.get 6
          select
          i32.store offset=40
          local.get 5
          i32.const 48
          i32.add
          i32.const 12
          i32.add
          i32.const 3
          i32.store
          local.get 5
          i32.const 48
          i32.add
          i32.const 20
          i32.add
          i32.const 3
          i32.store
          local.get 5
          i32.const 72
          i32.add
          i32.const 20
          i32.add
          i32.const 86
          i32.store
          local.get 5
          i32.const 72
          i32.add
          i32.const 12
          i32.add
          i32.const 86
          i32.store
          local.get 5
          i32.const 1056308
          i32.store offset=56
          local.get 5
          i32.const 0
          i32.store offset=48
          local.get 5
          i32.const 11
          i32.store offset=76
          local.get 5
          local.get 5
          i32.const 72
          i32.add
          i32.store offset=64
          local.get 5
          local.get 5
          i32.const 24
          i32.add
          i32.store offset=88
          local.get 5
          local.get 5
          i32.const 16
          i32.add
          i32.store offset=80
          local.get 5
          local.get 5
          i32.const 40
          i32.add
          i32.store offset=72
          local.get 5
          i32.const 48
          i32.add
          local.get 4
          call $_ZN4core9panicking9panic_fmt17h0724b0a601bd3679E
          unreachable
        end
        local.get 3
        local.get 6
        i32.const 1056520
        call $_ZN4core5slice5index22slice_index_order_fail17hb44181e9b62b1c0cE
        unreachable
      end
      i32.const 1054948
      i32.const 43
      local.get 4
      call $_ZN4core9panicking5panic17h33aac59f7456cb9eE
      unreachable
    end
    local.get 0
    local.get 1
    local.get 3
    local.get 1
    local.get 4
    call $_ZN4core3str16slice_error_fail17hd8efdeca0603f5c1E
    unreachable)
  (func $_ZN4core7unicode9printable5check17hac613972f1363ee5E (type 18) (param i32 i32 i32 i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    i32.const 1
    local.set 7
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        local.get 2
        i32.const 1
        i32.shl
        i32.add
        local.set 8
        local.get 0
        i32.const 65280
        i32.and
        i32.const 8
        i32.shr_u
        local.set 9
        i32.const 0
        local.set 10
        local.get 0
        i32.const 255
        i32.and
        local.set 11
        loop  ;; label = @3
          local.get 1
          i32.const 2
          i32.add
          local.set 12
          local.get 10
          local.get 1
          i32.load8_u offset=1
          local.tee 2
          i32.add
          local.set 13
          block  ;; label = @4
            local.get 1
            i32.load8_u
            local.tee 1
            local.get 9
            i32.eq
            br_if 0 (;@4;)
            local.get 1
            local.get 9
            i32.gt_u
            br_if 2 (;@2;)
            local.get 13
            local.set 10
            local.get 12
            local.set 1
            local.get 12
            local.get 8
            i32.eq
            br_if 2 (;@2;)
            br 1 (;@3;)
          end
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 13
                local.get 10
                i32.lt_u
                br_if 0 (;@6;)
                local.get 13
                local.get 4
                i32.gt_u
                br_if 1 (;@5;)
                local.get 3
                local.get 10
                i32.add
                local.set 1
                loop  ;; label = @7
                  local.get 2
                  i32.eqz
                  br_if 3 (;@4;)
                  local.get 2
                  i32.const -1
                  i32.add
                  local.set 2
                  local.get 1
                  i32.load8_u
                  local.set 10
                  local.get 1
                  i32.const 1
                  i32.add
                  local.set 1
                  local.get 10
                  local.get 11
                  i32.ne
                  br_if 0 (;@7;)
                end
                i32.const 0
                local.set 7
                br 5 (;@1;)
              end
              local.get 10
              local.get 13
              i32.const 1056576
              call $_ZN4core5slice5index22slice_index_order_fail17hb44181e9b62b1c0cE
              unreachable
            end
            local.get 13
            local.get 4
            i32.const 1056576
            call $_ZN4core5slice5index24slice_end_index_len_fail17h6cad56aacf5f762dE
            unreachable
          end
          local.get 13
          local.set 10
          local.get 12
          local.set 1
          local.get 12
          local.get 8
          i32.ne
          br_if 0 (;@3;)
        end
      end
      local.get 6
      i32.eqz
      br_if 0 (;@1;)
      local.get 5
      local.get 6
      i32.add
      local.set 11
      local.get 0
      i32.const 65535
      i32.and
      local.set 1
      i32.const 1
      local.set 7
      block  ;; label = @2
        loop  ;; label = @3
          local.get 5
          i32.const 1
          i32.add
          local.set 10
          block  ;; label = @4
            block  ;; label = @5
              local.get 5
              i32.load8_u
              local.tee 2
              i32.extend8_s
              local.tee 13
              i32.const 0
              i32.lt_s
              br_if 0 (;@5;)
              local.get 10
              local.set 5
              br 1 (;@4;)
            end
            local.get 10
            local.get 11
            i32.eq
            br_if 2 (;@2;)
            local.get 13
            i32.const 127
            i32.and
            i32.const 8
            i32.shl
            local.get 5
            i32.load8_u offset=1
            i32.or
            local.set 2
            local.get 5
            i32.const 2
            i32.add
            local.set 5
          end
          local.get 1
          local.get 2
          i32.sub
          local.tee 1
          i32.const 0
          i32.lt_s
          br_if 2 (;@1;)
          local.get 7
          i32.const 1
          i32.xor
          local.set 7
          local.get 5
          local.get 11
          i32.ne
          br_if 0 (;@3;)
          br 2 (;@1;)
        end
      end
      i32.const 1054948
      i32.const 43
      i32.const 1056592
      call $_ZN4core9panicking5panic17h33aac59f7456cb9eE
      unreachable
    end
    local.get 7
    i32.const 1
    i32.and)
  (func $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i8$GT$3fmt17h49c546b8e26ca6fbE (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 128
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 0
    i32.load8_u
    local.set 3
    i32.const 0
    local.set 0
    loop  ;; label = @1
      local.get 2
      local.get 0
      i32.add
      i32.const 127
      i32.add
      i32.const 48
      i32.const 87
      local.get 3
      i32.const 15
      i32.and
      local.tee 4
      i32.const 10
      i32.lt_u
      select
      local.get 4
      i32.add
      i32.store8
      local.get 0
      i32.const -1
      i32.add
      local.set 0
      local.get 3
      i32.const 255
      i32.and
      local.tee 4
      i32.const 4
      i32.shr_u
      local.set 3
      local.get 4
      i32.const 15
      i32.gt_u
      br_if 0 (;@1;)
    end
    block  ;; label = @1
      local.get 0
      i32.const 128
      i32.add
      local.tee 3
      i32.const 129
      i32.lt_u
      br_if 0 (;@1;)
      local.get 3
      i32.const 128
      i32.const 1055500
      call $_ZN4core5slice5index26slice_start_index_len_fail17h12ea13b84dc5aa33E
      unreachable
    end
    local.get 1
    i32.const 1
    i32.const 1055516
    i32.const 2
    local.get 2
    local.get 0
    i32.add
    i32.const 128
    i32.add
    i32.const 0
    local.get 0
    i32.sub
    call $_ZN4core3fmt9Formatter12pad_integral17h2602ad621a1201d6E
    local.set 0
    local.get 2
    i32.const 128
    i32.add
    global.set $__stack_pointer
    local.get 0)
  (func $_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17hb8259c8f1bee9a4aE (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 128
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 1
              i32.load offset=24
              local.tee 3
              i32.const 16
              i32.and
              br_if 0 (;@5;)
              local.get 3
              i32.const 32
              i32.and
              br_if 1 (;@4;)
              local.get 0
              i64.load8_u
              i32.const 1
              local.get 1
              call $_ZN4core3fmt3num3imp7fmt_u6417he8d38148f6fbdea6E
              local.set 0
              br 4 (;@1;)
            end
            local.get 0
            i32.load8_u
            local.set 3
            i32.const 0
            local.set 0
            loop  ;; label = @5
              local.get 2
              local.get 0
              i32.add
              i32.const 127
              i32.add
              i32.const 48
              i32.const 87
              local.get 3
              i32.const 15
              i32.and
              local.tee 4
              i32.const 10
              i32.lt_u
              select
              local.get 4
              i32.add
              i32.store8
              local.get 0
              i32.const -1
              i32.add
              local.set 0
              local.get 3
              i32.const 255
              i32.and
              local.tee 4
              i32.const 4
              i32.shr_u
              local.set 3
              local.get 4
              i32.const 15
              i32.gt_u
              br_if 0 (;@5;)
            end
            local.get 0
            i32.const 128
            i32.add
            local.tee 3
            i32.const 129
            i32.ge_u
            br_if 1 (;@3;)
            local.get 1
            i32.const 1
            i32.const 1055516
            i32.const 2
            local.get 2
            local.get 0
            i32.add
            i32.const 128
            i32.add
            i32.const 0
            local.get 0
            i32.sub
            call $_ZN4core3fmt9Formatter12pad_integral17h2602ad621a1201d6E
            local.set 0
            br 3 (;@1;)
          end
          local.get 0
          i32.load8_u
          local.set 3
          i32.const 0
          local.set 0
          loop  ;; label = @4
            local.get 2
            local.get 0
            i32.add
            i32.const 127
            i32.add
            i32.const 48
            i32.const 55
            local.get 3
            i32.const 15
            i32.and
            local.tee 4
            i32.const 10
            i32.lt_u
            select
            local.get 4
            i32.add
            i32.store8
            local.get 0
            i32.const -1
            i32.add
            local.set 0
            local.get 3
            i32.const 255
            i32.and
            local.tee 4
            i32.const 4
            i32.shr_u
            local.set 3
            local.get 4
            i32.const 15
            i32.gt_u
            br_if 0 (;@4;)
          end
          local.get 0
          i32.const 128
          i32.add
          local.tee 3
          i32.const 129
          i32.ge_u
          br_if 1 (;@2;)
          local.get 1
          i32.const 1
          i32.const 1055516
          i32.const 2
          local.get 2
          local.get 0
          i32.add
          i32.const 128
          i32.add
          i32.const 0
          local.get 0
          i32.sub
          call $_ZN4core3fmt9Formatter12pad_integral17h2602ad621a1201d6E
          local.set 0
          br 2 (;@1;)
        end
        local.get 3
        i32.const 128
        i32.const 1055500
        call $_ZN4core5slice5index26slice_start_index_len_fail17h12ea13b84dc5aa33E
        unreachable
      end
      local.get 3
      i32.const 128
      i32.const 1055500
      call $_ZN4core5slice5index26slice_start_index_len_fail17h12ea13b84dc5aa33E
      unreachable
    end
    local.get 2
    i32.const 128
    i32.add
    global.set $__stack_pointer
    local.get 0)
  (func $_ZN4core3fmt3num3imp7fmt_u6417he8d38148f6fbdea6E (type 19) (param i64 i32 i32) (result i32)
    (local i32 i32 i64 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    i32.const 39
    local.set 4
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i64.const 10000
        i64.ge_u
        br_if 0 (;@2;)
        local.get 0
        local.set 5
        br 1 (;@1;)
      end
      i32.const 39
      local.set 4
      loop  ;; label = @2
        local.get 3
        i32.const 9
        i32.add
        local.get 4
        i32.add
        local.tee 6
        i32.const -4
        i32.add
        local.get 0
        local.get 0
        i64.const 10000
        i64.div_u
        local.tee 5
        i64.const 10000
        i64.mul
        i64.sub
        i32.wrap_i64
        local.tee 7
        i32.const 65535
        i32.and
        i32.const 100
        i32.div_u
        local.tee 8
        i32.const 1
        i32.shl
        i32.const 1055518
        i32.add
        i32.load16_u align=1
        i32.store16 align=1
        local.get 6
        i32.const -2
        i32.add
        local.get 7
        local.get 8
        i32.const 100
        i32.mul
        i32.sub
        i32.const 65535
        i32.and
        i32.const 1
        i32.shl
        i32.const 1055518
        i32.add
        i32.load16_u align=1
        i32.store16 align=1
        local.get 4
        i32.const -4
        i32.add
        local.set 4
        local.get 0
        i64.const 99999999
        i64.gt_u
        local.set 6
        local.get 5
        local.set 0
        local.get 6
        br_if 0 (;@2;)
      end
    end
    block  ;; label = @1
      local.get 5
      i32.wrap_i64
      local.tee 6
      i32.const 99
      i32.le_u
      br_if 0 (;@1;)
      local.get 3
      i32.const 9
      i32.add
      local.get 4
      i32.const -2
      i32.add
      local.tee 4
      i32.add
      local.get 5
      i32.wrap_i64
      local.tee 6
      local.get 6
      i32.const 65535
      i32.and
      i32.const 100
      i32.div_u
      local.tee 6
      i32.const 100
      i32.mul
      i32.sub
      i32.const 65535
      i32.and
      i32.const 1
      i32.shl
      i32.const 1055518
      i32.add
      i32.load16_u align=1
      i32.store16 align=1
    end
    block  ;; label = @1
      block  ;; label = @2
        local.get 6
        i32.const 10
        i32.lt_u
        br_if 0 (;@2;)
        local.get 3
        i32.const 9
        i32.add
        local.get 4
        i32.const -2
        i32.add
        local.tee 4
        i32.add
        local.get 6
        i32.const 1
        i32.shl
        i32.const 1055518
        i32.add
        i32.load16_u align=1
        i32.store16 align=1
        br 1 (;@1;)
      end
      local.get 3
      i32.const 9
      i32.add
      local.get 4
      i32.const -1
      i32.add
      local.tee 4
      i32.add
      local.get 6
      i32.const 48
      i32.add
      i32.store8
    end
    local.get 2
    local.get 1
    i32.const 1054992
    i32.const 0
    local.get 3
    i32.const 9
    i32.add
    local.get 4
    i32.add
    i32.const 39
    local.get 4
    i32.sub
    call $_ZN4core3fmt9Formatter12pad_integral17h2602ad621a1201d6E
    local.set 4
    local.get 3
    i32.const 48
    i32.add
    global.set $__stack_pointer
    local.get 4)
  (func $_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i32$GT$3fmt17hf603227db3da36cbE (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 128
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 0
    i32.load
    local.set 0
    i32.const 0
    local.set 3
    loop  ;; label = @1
      local.get 2
      local.get 3
      i32.add
      i32.const 127
      i32.add
      i32.const 48
      i32.const 55
      local.get 0
      i32.const 15
      i32.and
      local.tee 4
      i32.const 10
      i32.lt_u
      select
      local.get 4
      i32.add
      i32.store8
      local.get 3
      i32.const -1
      i32.add
      local.set 3
      local.get 0
      i32.const 15
      i32.gt_u
      local.set 4
      local.get 0
      i32.const 4
      i32.shr_u
      local.set 0
      local.get 4
      br_if 0 (;@1;)
    end
    block  ;; label = @1
      local.get 3
      i32.const 128
      i32.add
      local.tee 0
      i32.const 129
      i32.lt_u
      br_if 0 (;@1;)
      local.get 0
      i32.const 128
      i32.const 1055500
      call $_ZN4core5slice5index26slice_start_index_len_fail17h12ea13b84dc5aa33E
      unreachable
    end
    local.get 1
    i32.const 1
    i32.const 1055516
    i32.const 2
    local.get 2
    local.get 3
    i32.add
    i32.const 128
    i32.add
    i32.const 0
    local.get 3
    i32.sub
    call $_ZN4core3fmt9Formatter12pad_integral17h2602ad621a1201d6E
    local.set 0
    local.get 2
    i32.const 128
    i32.add
    global.set $__stack_pointer
    local.get 0)
  (func $_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17hf5ba33fe2dd172e1E (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.tee 0
    i64.extend_i32_u
    i64.const 0
    local.get 0
    i64.extend_i32_s
    i64.sub
    local.get 0
    i32.const -1
    i32.gt_s
    local.tee 0
    select
    local.get 0
    local.get 1
    call $_ZN4core3fmt3num3imp7fmt_u6417he8d38148f6fbdea6E)
  (func $_ZN53_$LT$core..fmt..Error$u20$as$u20$core..fmt..Debug$GT$3fmt17h615f44a3ab1619c5E (type 2) (param i32 i32) (result i32)
    local.get 1
    i32.load
    i32.const 1058076
    i32.const 5
    local.get 1
    i32.load offset=4
    i32.load offset=12
    call_indirect (type 7))
  (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17ha331335a409d00caE (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 1
    call $_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17hb8259c8f1bee9a4aE)
  (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hf63a9a8f82ab90d5E (type 2) (param i32 i32) (result i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load
        local.tee 3
        i32.load8_u
        br_if 0 (;@2;)
        local.get 1
        i32.load
        i32.const 1058056
        i32.const 4
        local.get 1
        i32.load offset=4
        i32.load offset=12
        call_indirect (type 7)
        local.set 0
        br 1 (;@1;)
      end
      i32.const 1
      local.set 0
      local.get 2
      local.get 3
      i32.const 1
      i32.add
      i32.store offset=12
      local.get 2
      local.get 1
      i32.load
      i32.const 1058052
      i32.const 4
      local.get 1
      i32.load offset=4
      i32.load offset=12
      call_indirect (type 7)
      i32.store8 offset=24
      local.get 2
      local.get 1
      i32.store offset=20
      local.get 2
      i32.const 0
      i32.store8 offset=25
      local.get 2
      i32.const 0
      i32.store offset=16
      local.get 2
      i32.const 16
      i32.add
      local.get 2
      i32.const 12
      i32.add
      i32.const 1055456
      call $_ZN4core3fmt8builders10DebugTuple5field17hbd92e7b522abdb91E
      local.set 1
      local.get 2
      i32.load8_u offset=24
      local.set 3
      block  ;; label = @2
        local.get 1
        i32.load
        local.tee 1
        br_if 0 (;@2;)
        local.get 3
        i32.const 255
        i32.and
        i32.const 0
        i32.ne
        local.set 0
        br 1 (;@1;)
      end
      local.get 3
      i32.const 255
      i32.and
      br_if 0 (;@1;)
      local.get 2
      i32.load offset=20
      local.set 3
      block  ;; label = @2
        local.get 1
        i32.const 1
        i32.ne
        br_if 0 (;@2;)
        local.get 2
        i32.load8_u offset=25
        i32.const 255
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        i32.load8_u offset=24
        i32.const 4
        i32.and
        br_if 0 (;@2;)
        i32.const 1
        local.set 0
        local.get 3
        i32.load
        i32.const 1055452
        i32.const 1
        local.get 3
        i32.load offset=4
        i32.load offset=12
        call_indirect (type 7)
        br_if 1 (;@1;)
      end
      local.get 3
      i32.load
      i32.const 1054992
      i32.const 1
      local.get 3
      i32.load offset=4
      i32.load offset=12
      call_indirect (type 7)
      local.set 0
    end
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
    local.get 0)
  (func $_ZN64_$LT$core..str..error..Utf8Error$u20$as$u20$core..fmt..Debug$GT$3fmt17hed5432ab1db3e468E (type 2) (param i32 i32) (result i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    local.get 0
    i32.const 4
    i32.add
    i32.store offset=4
    local.get 1
    i32.load
    i32.const 1058081
    i32.const 9
    local.get 1
    i32.load offset=4
    i32.load offset=12
    call_indirect (type 7)
    local.set 3
    local.get 2
    i32.const 0
    i32.store8 offset=13
    local.get 2
    local.get 3
    i32.store8 offset=12
    local.get 2
    local.get 1
    i32.store offset=8
    local.get 2
    i32.const 8
    i32.add
    i32.const 1058090
    i32.const 11
    local.get 0
    i32.const 1058060
    call $_ZN4core3fmt8builders11DebugStruct5field17hc48bc4b5ba431201E
    i32.const 1058101
    i32.const 9
    local.get 2
    i32.const 4
    i32.add
    i32.const 1058112
    call $_ZN4core3fmt8builders11DebugStruct5field17hc48bc4b5ba431201E
    local.set 3
    local.get 2
    i32.load8_u offset=12
    local.set 0
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        i32.load8_u offset=13
        br_if 0 (;@2;)
        local.get 0
        i32.const 255
        i32.and
        i32.const 0
        i32.ne
        local.set 1
        br 1 (;@1;)
      end
      i32.const 1
      local.set 1
      local.get 0
      i32.const 255
      i32.and
      br_if 0 (;@1;)
      block  ;; label = @2
        local.get 3
        i32.load
        local.tee 1
        i32.load8_u offset=24
        i32.const 4
        i32.and
        br_if 0 (;@2;)
        local.get 1
        i32.load
        i32.const 1055447
        i32.const 2
        local.get 1
        i32.load offset=4
        i32.load offset=12
        call_indirect (type 7)
        local.set 1
        br 1 (;@1;)
      end
      local.get 1
      i32.load
      i32.const 1055446
      i32.const 1
      local.get 1
      i32.load offset=4
      i32.load offset=12
      call_indirect (type 7)
      local.set 1
    end
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 1)
  (func $_ZN69_$LT$core..alloc..layout..LayoutError$u20$as$u20$core..fmt..Debug$GT$3fmt17hf8e1039d3ccc43faE (type 2) (param i32 i32) (result i32)
    local.get 1
    i32.load
    i32.const 1059019
    i32.const 11
    local.get 1
    i32.load offset=4
    i32.load offset=12
    call_indirect (type 7))
  (table (;0;) 102 102 funcref)
  (memory (;0;) 17)
  (global $__stack_pointer (mut i32) (i32.const 1048576))
  (export "memory" (memory 0))
  (export "_start" (func $_start))
  (export "__main_void" (func $__main_void))
  (elem (;0;) (i32.const 1) func $_ZN4core3ptr42drop_in_place$LT$std..io..error..Error$GT$17h10101e7c97466622E $_ZN58_$LT$std..io..error..Error$u20$as$u20$core..fmt..Debug$GT$3fmt17h3fdbb843780dfbdaE $_ZN5hello4main17h4f085eaf1e607c70E $_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h176bff3dde3ce504E $_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h32ac9f366558f692E $_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h1f94b09a6ec043aaE $_ZN59_$LT$core..fmt..Arguments$u20$as$u20$core..fmt..Display$GT$3fmt17h0bbc610b5c9a689cE $_ZN70_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hc0ba9e9c7e604a19E $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h7c5b97921bca430bE $_ZN91_$LT$std..sys_common..backtrace.._print..DisplayBacktrace$u20$as$u20$core..fmt..Display$GT$3fmt17h1b268c0cbb1a2de3E $_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17hc287c07e1a56819aE $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h22b17f88a102613aE $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h1d1cce09ac4fc977E $_ZN73_$LT$core..panic..panic_info..PanicInfo$u20$as$u20$core..fmt..Display$GT$3fmt17hda9c7998793e9d25E $_ZN3std5alloc24default_alloc_error_hook17h910158b746051dc0E $_ZN4core3ptr205drop_in_place$LT$$LT$alloc..boxed..Box$LT$dyn$u20$core..error..Error$u2b$core..marker..Send$u2b$core..marker..Sync$GT$$u20$as$u20$core..convert..From$LT$alloc..string..String$GT$$GT$..from..StringError$GT$17h0a9c176b58215858E $_ZN223_$LT$$LT$alloc..boxed..Box$LT$dyn$u20$core..error..Error$u2b$core..marker..Send$u2b$core..marker..Sync$GT$$u20$as$u20$core..convert..From$LT$alloc..string..String$GT$$GT$..from..StringError$u20$as$u20$core..fmt..Display$GT$3fmt17hb6057d54086b2d2cE $_ZN221_$LT$$LT$alloc..boxed..Box$LT$dyn$u20$core..error..Error$u2b$core..marker..Send$u2b$core..marker..Sync$GT$$u20$as$u20$core..convert..From$LT$alloc..string..String$GT$$GT$..from..StringError$u20$as$u20$core..fmt..Debug$GT$3fmt17hf29197e7c3e2a133E $_ZN4core5error5Error5cause17hba07ba1b29ce5bffE $_ZN4core5error5Error7type_id17h53f8e79ab2115a0eE $_ZN223_$LT$$LT$alloc..boxed..Box$LT$dyn$u20$core..error..Error$u2b$core..marker..Send$u2b$core..marker..Sync$GT$$u20$as$u20$core..convert..From$LT$alloc..string..String$GT$$GT$..from..StringError$u20$as$u20$core..error..Error$GT$11description17h0df52edac3281a52E $_ZN4core5error5Error7provide17h0c60d749fab417ebE $_ZN4core3ptr100drop_in_place$LT$$RF$mut$u20$std..io..Write..write_fmt..Adapter$LT$alloc..vec..Vec$LT$u8$GT$$GT$$GT$17h1052ad729efca941E $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17h33842eeb8c82131cE $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h5a1a43da4a400424E $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17h1e452089e8e4bbeaE $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17h763c09e7c5464cdeE $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h1085e6bde15e7786E $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17hddfc5c6b607045dbE $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17he3cf788bc3442563E $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h9212a16a243fbfafE $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17h58488dcbcdd00066E $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h748212c8b37ccc7cE $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h6c78f7e2c05482edE $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h8102055cf4895771E $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h7501dcf2f1cfc31fE $_ZN62_$LT$std..io..error..ErrorKind$u20$as$u20$core..fmt..Debug$GT$3fmt17h5659196e58541555E $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h7015614129a78a15E $_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$i32$GT$3fmt17h363321e40170dea2E $_ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Debug$GT$3fmt17h88d09d9d08f2cd1aE $_ZN63_$LT$core..cell..BorrowMutError$u20$as$u20$core..fmt..Debug$GT$3fmt17h3ea277d26af0c621E $_ZN4core3ptr88drop_in_place$LT$std..io..Write..write_fmt..Adapter$LT$alloc..vec..Vec$LT$u8$GT$$GT$$GT$17h5717bcf441afcf24E $_ZN80_$LT$std..io..Write..write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17h7eb9288352c31cc7E $_ZN4core3fmt5Write10write_char17h1dbd9303b03125e4E $_ZN4core3fmt5Write9write_fmt17hac4919787652327dE $_ZN80_$LT$std..io..Write..write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17h05a7908f2c5d636aE $_ZN4core3fmt5Write10write_char17h5e18a2590c434ed3E $_ZN4core3fmt5Write9write_fmt17h561b73a22a7a7b11E $_ZN64_$LT$std..sys..wasi..stdio..Stderr$u20$as$u20$std..io..Write$GT$5write17h56809a5b3fcaeebdE $_ZN64_$LT$std..sys..wasi..stdio..Stderr$u20$as$u20$std..io..Write$GT$14write_vectored17h5575c3c26ed3d779E $_ZN64_$LT$std..sys..wasi..stdio..Stderr$u20$as$u20$std..io..Write$GT$17is_write_vectored17h8908d08cb5e31eddE $_ZN64_$LT$std..sys..wasi..stdio..Stderr$u20$as$u20$std..io..Write$GT$5flush17h2c1e73e04b726cdeE $_ZN3std2io5Write9write_all17h8a8bac0433a121c1E $_ZN3std2io5Write18write_all_vectored17hb70535989338ad69E $_ZN3std2io5Write9write_fmt17h73c1d085b221682dE $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$5write17h390bc906e4bb57aeE $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$14write_vectored17h0a5a512fe70cd54eE $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$17is_write_vectored17heaadadfc4ca80892E $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$5flush17hf40739bb6c56c242E $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$9write_all17hc56765d60e1b01daE $_ZN3std2io5Write18write_all_vectored17he23b35b7a95124f8E $_ZN3std2io5Write9write_fmt17h3a41373f70f0d61fE $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h77d2e9ebd7fed1efE $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17hc26280eb2ec5087fE $_ZN4core3ptr70drop_in_place$LT$core..option..Option$LT$alloc..string..String$GT$$GT$17h493eaf1a04cd9ae5E $_ZN90_$LT$std..panicking..begin_panic_handler..PanicPayload$u20$as$u20$core..panic..BoxMeUp$GT$8take_box17h897bde01b250c956E $_ZN90_$LT$std..panicking..begin_panic_handler..PanicPayload$u20$as$u20$core..panic..BoxMeUp$GT$3get17h4fea0969750388b6E $_ZN93_$LT$std..panicking..begin_panic_handler..StrPanicPayload$u20$as$u20$core..panic..BoxMeUp$GT$8take_box17h8b0280949f0541bdE $_ZN93_$LT$std..panicking..begin_panic_handler..StrPanicPayload$u20$as$u20$core..panic..BoxMeUp$GT$3get17hdc699511fb6d5496E $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h55be51e7bf83a042E $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17ha5cca69876c36c70E $_ZN64_$LT$core..str..error..Utf8Error$u20$as$u20$core..fmt..Debug$GT$3fmt17hed5432ab1db3e468E $__wasilibc_find_relpath_alloc $_ZN4core3ptr26drop_in_place$LT$usize$GT$17h84e39a189230a888E $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17h622aaed9ad907416E $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17hedbee5f483d2714eE $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17h754307fcaa43f1fbE $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hdf42619595b66d37E $_ZN53_$LT$core..fmt..Error$u20$as$u20$core..fmt..Debug$GT$3fmt17h615f44a3ab1619c5E $_ZN69_$LT$core..alloc..layout..LayoutError$u20$as$u20$core..fmt..Debug$GT$3fmt17hf8e1039d3ccc43faE $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hffbfc641a26cd37cE $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17ha4ae67ad4237772aE $_ZN4core3ops8function6FnOnce9call_once17haeae9a1c0a16fac0E $_ZN70_$LT$core..slice..ascii..EscapeAscii$u20$as$u20$core..fmt..Display$GT$3fmt17h5411013031d90505E $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hc77e939c53299de2E $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h8ff29995c2d0d3f7E $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h20d7637be7555355E $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h95d6fa887e65cea7E $_ZN71_$LT$core..ops..range..Range$LT$Idx$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17h4234e500297b1dd2E $_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17hdf2bf09c0dbb3fa6E $_ZN4core3ptr102drop_in_place$LT$$RF$core..iter..adapters..copied..Copied$LT$core..slice..iter..Iter$LT$u8$GT$$GT$$GT$17hf914f599202dabbdE $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h961caf76bf45b94bE $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h074df0fdbb17926eE $_ZN4core3fmt5Write10write_char17h3b769f78203a4355E $_ZN4core3fmt5Write9write_fmt17hf9f747f21f49b304E $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17ha331335a409d00caE $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17hb49c5e69f7371507E $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h9d7c24ae95e50eb8E $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17h25a03326608365dcE $_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u32$GT$3fmt17h8b3eea51f609ced3E $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hf63a9a8f82ab90d5E)
  (data $.rodata (i32.const 1048576) "called `Result::unwrap()` on an `Err` value\00\01\00\00\00\08\00\00\00\04\00\00\00\02\00\00\00foo./src/hello.rs\00\00\00?\00\10\00\0e\00\00\00\02\00\00\00\22\00\00\00\04\00\00\00\04\00\00\00\04\00\00\00\05\00\00\00\06\00\00\00\06\00\00\00\10\00\00\00\0c\00\00\00\04\00\00\00\11\00\00\00\10\00\00\00\0c\00\00\00\04\00\00\00\12\00\00\00\11\00\00\00x\00\10\00\13\00\00\00\14\00\00\00\15\00\00\00\13\00\00\00\16\00\00\00()\00\00\17\00\00\00\04\00\00\00\04\00\00\00\18\00\00\00\19\00\00\00\1a\00\00\00\17\00\00\00\04\00\00\00\04\00\00\00\1b\00\00\00\1c\00\00\00\1d\00\00\00\17\00\00\00\04\00\00\00\04\00\00\00\1e\00\00\00\1f\00\00\00 \00\00\00\00\00\00\00\17\00\00\00\04\00\00\00\04\00\00\00!\00\00\00\17\00\00\00\04\00\00\00\04\00\00\00\22\00\00\00called `Option::unwrap()` on a `None` valueinternal error: entered unreachable code/rustc/90c541806f23a127002de5b4038be731ba1458ca/library/alloc/src/vec/mod.rs\00w\01\10\00L\00\00\00\c8\07\00\00$\00\00\00ErrOk\00\00\00\17\00\00\00\04\00\00\00\04\00\00\00#\00\00\00\17\00\00\00\04\00\00\00\04\00\00\00$\00\00\00mainfatal runtime error: \0a\00\00\00\02\10\00\15\00\00\00\15\02\10\00\01\00\00\00unwrap failed: CString::new(\22main\22) = \00\00(\02\10\00&\00\00\00library/std/src/rt.rs\00\00\00X\02\10\00\15\00\00\00r\00\00\00\0d\00\00\00library/std/src/thread/mod.rsfailed to generate unique thread ID: bitspace exhausted\9d\02\10\007\00\00\00\80\02\10\00\1d\00\00\00=\04\00\00\0d\00\00\00RUST_BACKTRACEcalled `Result::unwrap()` on an `Err` valuefailed to write the buffered data\00\00%\03\10\00!\00\00\00\17\00\00\00\00\01\10\00\00\00\00\00Errorkind\00\00\00\17\00\00\00\01\00\00\00\01\00\00\00%\00\00\00message\00\17\00\00\00\08\00\00\00\04\00\00\00&\00\00\00KindOscode\00\00\17\00\00\00\04\00\00\00\04\00\00\00'\00\00\00\10\00\00\00\0c\00\00\00\04\00\00\00(\00\00\00already borrowed\17\00\00\00\00\00\00\00\01\00\00\00)\00\00\00library/std/src/io/stdio.rs\00\dc\03\10\00\1b\00\00\00}\02\00\00\13\00\00\00library/std/src/io/mod.rs\00\00\00\08\04\10\00\19\00\00\004\05\00\00\16\00\00\00advancing io slices beyond their length\004\04\10\00'\00\00\00\08\04\10\00\19\00\00\006\05\00\00\0d\00\00\00\08\04\10\00\19\00\00\00\1d\06\00\00!\00\00\00failed to write whole buffer\84\04\10\00\1c\00\00\00\17\00\00\00*\00\00\00\0c\00\00\00\04\00\00\00+\00\00\00,\00\00\00-\00\00\00formatter error\00\c4\04\10\00\0f\00\00\00(\00\00\00*\00\00\00\0c\00\00\00\04\00\00\00.\00\00\00/\00\00\000\00\00\00input must be utf-8\00\f8\04\10\00\13\00\00\00(\00\00\00library/std/src/os/fd/owned.rslibrary/std/src/panic.rs\00\006\05\10\00\18\00\00\00\f3\00\00\00\12\00\00\00library/std/src/sync/once.rs`\05\10\00\1c\00\00\00\95\00\00\002\00\00\00`\05\10\00\1c\00\00\00\d0\00\00\00\14\00\00\00`\05\10\00\1c\00\00\00\d0\00\00\001\00\00\00lock count overflow in reentrant mutexlibrary/std/src/sync/remutex.rs\00\00\00\d2\05\10\00\1f\00\00\00\91\00\00\00\0e\00\00\00file name contained an unexpected NUL byte\00\00\04\06\10\00*\00\00\00\14\00\00\00\00\00\00\00\02\00\00\000\06\10\00stack backtrace:\0a\00\00\00H\06\10\00\11\00\00\00note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.\0ad\06\10\00X\00\00\00library/std/src/sys_common/thread_info.rs\00\00\00\c4\06\10\00)\00\00\00\16\00\00\003\00\00\00\c4\06\10\00)\00\00\00+\00\00\00+\00\00\00assertion failed: thread_info.is_none()\00\10\07\10\00'\00\00\00memory allocation of  bytes failed\00\00@\07\10\00\15\00\00\00U\07\10\00\0d\00\00\00library/std/src/alloc.rst\07\10\00\18\00\00\00U\01\00\00\09\00\00\00 bytes failed\0a\00\00@\07\10\00\15\00\00\00\9c\07\10\00\0e\00\00\00library/std/src/panicking.rsBox<dyn Any><unnamed>\00\00\00\17\00\00\00\00\00\00\00\01\00\00\001\00\00\002\00\00\003\00\00\004\00\00\005\00\00\006\00\00\007\00\00\00\10\00\00\00\0c\00\00\00\04\00\00\008\00\00\009\00\00\00:\00\00\00;\00\00\00<\00\00\00=\00\00\00>\00\00\00\bc\07\10\00\1c\00\00\00\f7\00\00\00$\00\00\00thread '' panicked at '', \00\00P\08\10\00\08\00\00\00X\08\10\00\0f\00\00\00g\08\10\00\03\00\00\00\15\02\10\00\01\00\00\00note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace\0a\00\00\8c\08\10\00N\00\00\00\bc\07\10\00\1c\00\00\00A\02\00\00\1e\00\00\00\bc\07\10\00\1c\00\00\00@\02\00\00\1f\00\00\00\10\00\00\00\0c\00\00\00\04\00\00\00?\00\00\00\17\00\00\00\08\00\00\00\04\00\00\00@\00\00\00A\00\00\00\10\00\00\00\04\00\00\00B\00\00\00C\00\00\00\17\00\00\00\08\00\00\00\04\00\00\00D\00\00\00E\00\00\00thread panicked while processing panic. aborting.\0a\00\00L\09\10\002\00\00\00\17\00\00\00\00\00\00\00\01\00\00\00F\00\00\00\0apanicked after panic::always_abort(), aborting.\0a\00\00\00\00\01\10\00\00\00\00\00\98\09\10\001\00\00\00thread caused non-unwinding panic. aborting.\0a\00\00\00\dc\09\10\00-\00\00\00thread panicked while panicking. aborting.\0a\00\14\0a\10\00+\00\00\00failed to initiate panic, error H\0a\10\00 \00\00\00UnsupportedCustomerror\00\00\17\00\00\00\04\00\00\00\04\00\00\00G\00\00\00UncategorizedOtherOutOfMemoryUnexpectedEofInterruptedArgumentListTooLongInvalidFilenameTooManyLinksCrossesDevicesDeadlockExecutableFileBusyResourceBusyFileTooLargeFilesystemQuotaExceededNotSeekableStorageFullWriteZeroTimedOutInvalidDataInvalidInputStaleNetworkFileHandleFilesystemLoopReadOnlyFilesystemDirectoryNotEmptyIsADirectoryNotADirectoryWouldBlockAlreadyExistsBrokenPipeNetworkDownAddrNotAvailableAddrInUseNotConnectedConnectionAbortedNetworkUnreachableHostUnreachableConnectionResetConnectionRefusedPermissionDeniedNotFound\00\ff\ff\ff\ff\18\05\10\00\1e\00\00\00\a1\00\00\00\09\00\00\00failed to find a pre-opened file descriptor through which  could be opened\00\00\c0\0c\10\00:\00\00\00\fa\0c\10\00\10\00\00\00advancing IoSlice beyond its length\00\1c\0d\10\00#\00\00\00library/std/src/sys/wasi/io.rs\00\00H\0d\10\00\1e\00\00\00\17\00\00\00\0d\00\00\00cannot recursively acquire mutexx\0d\10\00 \00\00\00library/std/src/sys/wasi/../unsupported/locks/mutex.rs\00\00\a0\0d\10\006\00\00\00\14\00\00\00\09\00\00\00rwlock locked for writing\00\00\00\e8\0d\10\00\19\00\00\00one-time initialization may not be performed recursively\0c\0e\10\008\00\00\00Once instance has previously been poisoned\00\00L\0e\10\00*\00\00\00strerror_r failure\00\00\80\0e\10\00\12\00\00\00library/std/src/sys/wasi/os.rs\00\00\9c\0e\10\00\1e\00\00\00A\00\00\00\0d\00\00\00\17\00\00\00\08\00\00\00\04\00\00\00H\00\00\00\9c\0e\10\00\1e\00\00\00C\00\00\006\00\00\00\08\00\0e\00\0f\00?\00\02\00@\005\00\0d\00\04\00\03\00,\00\1b\00\1c\00I\00\14\00\06\004\000\00\08\00\00\00\10\00\00\00\11\00\00\00\0f\00\00\00\0f\00\00\00\12\00\00\00\11\00\00\00\0c\00\00\00\09\00\00\00\10\00\00\00\0b\00\00\00\0a\00\00\00\0d\00\00\00\0a\00\00\00\0d\00\00\00\0c\00\00\00\11\00\00\00\12\00\00\00\0e\00\00\00\16\00\00\00\0c\00\00\00\0b\00\00\00\08\00\00\00\09\00\00\00\0b\00\00\00\0b\00\00\00\17\00\00\00\0c\00\00\00\0c\00\00\00\12\00\00\00\08\00\00\00\0e\00\00\00\0c\00\00\00\0f\00\00\00\13\00\00\00\0b\00\00\00\0b\00\00\00\0d\00\00\00\0b\00\00\00\05\00\00\00\0d\00\00\00\a3\0c\10\00\93\0c\10\00\82\0c\10\00s\0c\10\00d\0c\10\00R\0c\10\00A\0c\10\005\0c\10\00,\0c\10\00\1c\0c\10\00\11\0c\10\00\07\0c\10\00\fa\0b\10\00\f0\0b\10\00\e3\0b\10\00\d7\0b\10\00\c6\0b\10\00\b4\0b\10\00\a6\0b\10\00\90\0b\10\00\84\0b\10\00y\0b\10\00q\0b\10\00h\0b\10\00]\0b\10\00R\0b\10\00;\0b\10\00/\0b\10\00#\0b\10\00\11\0b\10\00\09\0b\10\00\fb\0a\10\00\ef\0a\10\00\e0\0a\10\00\cd\0a\10\00\c2\0a\10\00p\0a\10\00\b5\0a\10\00\aa\0a\10\00\a5\0a\10\00\98\0a\10\00/\00.\00.\00./\00Success\00Illegal byte sequence\00Domain error\00Result not representable\00Not a tty\00Permission denied\00Operation not permitted\00No such file or directory\00No such process\00File exists\00Value too large for data type\00No space left on device\00Out of memory\00Resource busy\00Interrupted system call\00Resource temporarily unavailable\00Invalid seek\00Cross-device link\00Read-only file system\00Directory not empty\00Connection reset by peer\00Operation timed out\00Connection refused\00Host is unreachable\00Address in use\00Broken pipe\00I/O error\00No such device or address\00No such device\00Not a directory\00Is a directory\00Text file busy\00Exec format error\00Invalid argument\00Argument list too long\00Symbolic link loop\00Filename too long\00Too many open files in system\00No file descriptors available\00Bad file descriptor\00No child process\00Bad address\00File too large\00Too many links\00No locks available\00Resource deadlock would occur\00State not recoverable\00Previous owner died\00Operation canceled\00Function not implemented\00No message of desired type\00Identifier removed\00Link has been severed\00Protocol error\00Bad message\00Not a socket\00Destination address required\00Message too large\00Protocol wrong type for socket\00Protocol not available\00Protocol not supported\00Not supported\00Address family not supported by protocol\00Address not available\00Network is down\00Network unreachable\00Connection reset by network\00Connection aborted\00No buffer space available\00Socket is connected\00Socket not connected\00Operation already in progress\00Operation in progress\00Stale file handle\00Quota exceeded\00Multihop attempted\00Capabilities insufficient\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00u\02N\00\d6\01\e2\04\b9\04\18\01\8e\05\ed\02\16\04\f2\00\97\03\01\038\05\af\01\82\01O\03/\04\1e\00\d4\05\a2\00\12\03\1e\03\c2\01\de\03\08\00\ac\05\00\01d\02\f1\01e\054\02\8c\02\cf\02-\03L\04\e3\05\9f\02\f8\04\1c\05\08\05\b1\02K\05\15\02x\00R\02<\03\f1\03\e4\00\c3\03}\04\cc\00\aa\03y\05$\02n\01m\03\22\04\ab\04D\00\fb\01\ae\00\83\03`\00\e5\01\07\04\94\04^\04+\00X\019\01\92\00\c2\05\9b\01C\02F\01\f6\05\00\00J\00\00\00\04\00\00\00\04\00\00\00K\00\00\00L\00\00\00M\00\00\00J\00\00\00\04\00\00\00\04\00\00\00N\00\00\00library/alloc/src/raw_vec.rscapacity overflow\00\00\00`\17\10\00\11\00\00\00D\17\10\00\1c\00\00\00\0c\02\00\00\05\00\00\00called `Option::unwrap()` on a `None` valuelibrary/alloc/src/ffi/c_str.rs\00\00\00\b7\17\10\00\1e\00\00\00\1b\01\00\007\00\00\00a formatting trait implementation returned an error\00J\00\00\00\00\00\00\00\01\00\00\00O\00\00\00library/alloc/src/fmt.rs,\18\10\00\18\00\00\00d\02\00\00 \00\00\00called `Result::unwrap()` on an `Err` value\00J\00\00\00\00\00\00\00\01\00\00\00P\00\00\00library/alloc/src/sync.rs\00\00\00\90\18\10\00\19\00\00\00[\01\00\002\00\00\00NulErrorJ\00\00\00\04\00\00\00\04\00\00\00Q\00\00\00J\00\00\00\04\00\00\00\04\00\00\00R\00\00\00called `Option::unwrap()` on a `None` value\00)..\00\11\19\10\00\02\00\00\000123456789abcdeflibrary/core/src/ascii.rs\00\00\00,\19\10\00\19\00\00\00t\00\00\00#\00\00\00BorrowMutError\22\00f\19\10\00\01\00\00\00f\19\10\00\01\00\00\00\10\19\10\00\00\00\00\00:[\00\00\10\19\10\00\00\00\00\00\80\19\10\00\01\00\00\00\80\19\10\00\01\00\00\00panicked at '', \a8\19\10\00\01\00\00\00\a9\19\10\00\03\00\00\00[\00\00\00\00\00\00\00\01\00\00\00\5c\00\00\00index out of bounds: the len is  but the index is \00\00\cc\19\10\00 \00\00\00\ec\19\10\00\12\00\00\00matches!===assertion failed: `(left  right)`\0a  left: ``,\0a right: ``: \00\00\00\1b\1a\10\00\19\00\00\004\1a\10\00\12\00\00\00F\1a\10\00\0c\00\00\00R\1a\10\00\03\00\00\00`\00\00\00\1b\1a\10\00\19\00\00\004\1a\10\00\12\00\00\00F\1a\10\00\0c\00\00\00x\1a\10\00\01\00\00\00: \00\00\10\19\10\00\00\00\00\00\9c\1a\10\00\02\00\00\00[\00\00\00\0c\00\00\00\04\00\00\00]\00\00\00^\00\00\00_\00\00\00     {\0a,\0a,  { } }(\0a(,\0a\00\00[\00\00\00\04\00\00\00\04\00\00\00`\00\00\00]library/core/src/fmt/num.rs\f1\1a\10\00\1b\00\00\00e\00\00\00\14\00\00\000x00010203040506070809101112131415161718192021222324252627282930313233343536373839404142434445464748495051525354555657585960616263646566676869707172737475767778798081828384858687888990919293949596979899\00\00[\00\00\00\04\00\00\00\04\00\00\00a\00\00\00b\00\00\00c\00\00\00library/core/src/fmt/mod.rstruefalse\00\1c\10\00\1b\00\00\00\a8\09\00\00\1e\00\00\00\00\1c\10\00\1b\00\00\00\af\09\00\00\16\00\00\00library/core/src/slice/memchr.rsD\1c\10\00 \00\00\00q\00\00\00'\00\00\00range start index  out of range for slice of length t\1c\10\00\12\00\00\00\86\1c\10\00\22\00\00\00range end index \b8\1c\10\00\10\00\00\00\86\1c\10\00\22\00\00\00slice index starts at  but ends at \00\d8\1c\10\00\16\00\00\00\ee\1c\10\00\0d\00\00\00\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\03\03\03\03\03\03\03\03\03\03\03\03\03\03\03\03\04\04\04\04\04\00\00\00\00\00\00\00\00\00\00\00[...]byte index  is out of bounds of `\00\00\11\1e\10\00\0b\00\00\00\1c\1e\10\00\16\00\00\00x\1a\10\00\01\00\00\00begin <= end ( <= ) when slicing `\00\00L\1e\10\00\0e\00\00\00Z\1e\10\00\04\00\00\00^\1e\10\00\10\00\00\00x\1a\10\00\01\00\00\00 is not a char boundary; it is inside  (bytes ) of `\11\1e\10\00\0b\00\00\00\90\1e\10\00&\00\00\00\b6\1e\10\00\08\00\00\00\be\1e\10\00\06\00\00\00x\1a\10\00\01\00\00\00library/core/src/str/mod.rs\00\ec\1e\10\00\1b\00\00\00\07\01\00\00\1d\00\00\00library/core/src/unicode/printable.rs\00\00\00\18\1f\10\00%\00\00\00\0a\00\00\00\1c\00\00\00\18\1f\10\00%\00\00\00\1a\00\00\006\00\00\00\00\01\03\05\05\06\06\02\07\06\08\07\09\11\0a\1c\0b\19\0c\1a\0d\10\0e\0c\0f\04\10\03\12\12\13\09\16\01\17\04\18\01\19\03\1a\07\1b\01\1c\02\1f\16 \03+\03-\0b.\010\031\022\01\a7\02\a9\02\aa\04\ab\08\fa\02\fb\05\fd\02\fe\03\ff\09\adxy\8b\8d\a20WX\8b\8c\90\1c\dd\0e\0fKL\fb\fc./?\5c]_\e2\84\8d\8e\91\92\a9\b1\ba\bb\c5\c6\c9\ca\de\e4\e5\ff\00\04\11\12)147:;=IJ]\84\8e\92\a9\b1\b4\ba\bb\c6\ca\ce\cf\e4\e5\00\04\0d\0e\11\12)14:;EFIJ^de\84\91\9b\9d\c9\ce\cf\0d\11):;EIW[\5c^_de\8d\91\a9\b4\ba\bb\c5\c9\df\e4\e5\f0\0d\11EIde\80\84\b2\bc\be\bf\d5\d7\f0\f1\83\85\8b\a4\a6\be\bf\c5\c7\cf\da\dbH\98\bd\cd\c6\ce\cfINOWY^_\89\8e\8f\b1\b6\b7\bf\c1\c6\c7\d7\11\16\17[\5c\f6\f7\fe\ff\80mq\de\df\0e\1fno\1c\1d_}~\ae\af\7f\bb\bc\16\17\1e\1fFGNOXZ\5c^~\7f\b5\c5\d4\d5\dc\f0\f1\f5rs\8ftu\96&./\a7\af\b7\bf\c7\cf\d7\df\9a@\97\980\8f\1f\d2\d4\ce\ffNOZ[\07\08\0f\10'/\ee\efno7=?BE\90\91Sgu\c8\c9\d0\d1\d8\d9\e7\fe\ff\00 _\22\82\df\04\82D\08\1b\04\06\11\81\ac\0e\80\ab\05\1f\09\81\1b\03\19\08\01\04/\044\04\07\03\01\07\06\07\11\0aP\0f\12\07U\07\03\04\1c\0a\09\03\08\03\07\03\02\03\03\03\0c\04\05\03\0b\06\01\0e\15\05N\07\1b\07W\07\02\06\17\0cP\04C\03-\03\01\04\11\06\0f\0c:\04\1d%_ m\04j%\80\c8\05\82\b0\03\1a\06\82\fd\03Y\07\16\09\18\09\14\0c\14\0cj\06\0a\06\1a\06Y\07+\05F\0a,\04\0c\04\01\031\0b,\04\1a\06\0b\03\80\ac\06\0a\06/1M\03\80\a4\08<\03\0f\03<\078\08+\05\82\ff\11\18\08/\11-\03!\0f!\0f\80\8c\04\82\97\19\0b\15\88\94\05/\05;\07\02\0e\18\09\80\be\22t\0c\80\d6\1a\0c\05\80\ff\05\80\df\0c\f2\9d\037\09\81\5c\14\80\b8\08\80\cb\05\0a\18;\03\0a\068\08F\08\0c\06t\0b\1e\03Z\04Y\09\80\83\18\1c\0a\16\09L\04\80\8a\06\ab\a4\0c\17\041\a1\04\81\da&\07\0c\05\05\80\a6\10\81\f5\07\01 *\06L\04\80\8d\04\80\be\03\1b\03\0f\0d\00\06\01\01\03\01\04\02\05\07\07\02\08\08\09\02\0a\05\0b\02\0e\04\10\01\11\02\12\05\13\11\14\01\15\02\17\02\19\0d\1c\05\1d\08\1f\01$\01j\04k\02\af\03\b1\02\bc\02\cf\02\d1\02\d4\0c\d5\09\d6\02\d7\02\da\01\e0\05\e1\02\e7\04\e8\02\ee \f0\04\f8\02\fa\03\fb\01\0c';>NO\8f\9e\9e\9f{\8b\93\96\a2\b2\ba\86\b1\06\07\096=>V\f3\d0\d1\04\14\1867VW\7f\aa\ae\af\bd5\e0\12\87\89\8e\9e\04\0d\0e\11\12)14:EFIJNOde\5c\b6\b7\1b\1c\07\08\0a\0b\14\1769:\a8\a9\d8\d9\097\90\91\a8\07\0a;>fi\8f\92\11o_\bf\ee\efZb\f4\fc\ffST\9a\9b./'(U\9d\a0\a1\a3\a4\a7\a8\ad\ba\bc\c4\06\0b\0c\15\1d:?EQ\a6\a7\cc\cd\a0\07\19\1a\22%>?\e7\ec\ef\ff\c5\c6\04 #%&(38:HJLPSUVXZ\5c^`cefksx}\7f\8a\a4\aa\af\b0\c0\d0\ae\afno\be\93^\22{\05\03\04-\03f\03\01/.\80\82\1d\031\0f\1c\04$\09\1e\05+\05D\04\0e*\80\aa\06$\04$\04(\084\0bNC\817\09\16\0a\08\18;E9\03c\08\090\16\05!\03\1b\05\01@8\04K\05/\04\0a\07\09\07@ '\04\0c\096\03:\05\1a\07\04\0c\07PI73\0d3\07.\08\0a\81&RK+\08*\16\1a&\1c\14\17\09N\04$\09D\0d\19\07\0a\06H\08'\09u\0bB>*\06;\05\0a\06Q\06\01\05\10\03\05\80\8bb\1eH\08\0a\80\a6^\22E\0b\0a\06\0d\13:\06\0a6,\04\17\80\b9<dS\0cH\09\0aFE\1bH\08S\0dI\07\0a\80\f6F\0a\1d\03GI7\03\0e\08\0a\069\07\0a\816\19\07;\03\1cV\01\0f2\0d\83\9bfu\0b\80\c4\8aLc\0d\840\10\16\8f\aa\82G\a1\b9\829\07*\04\5c\06&\0aF\0a(\05\13\82\b0[eK\049\07\11@\05\0b\02\0e\97\f8\08\84\d6*\09\a2\e7\813\0f\01\1d\06\0e\04\08\81\8c\89\04k\05\0d\03\09\07\10\92`G\09t<\80\f6\0as\08p\15Fz\14\0c\14\0cW\09\19\80\87\81G\03\85B\0f\15\84P\1f\06\06\80\d5+\05>!\01p-\03\1a\04\02\81@\1f\11:\05\01\81\d0*\82\e6\80\f7)L\04\0a\04\02\83\11DL=\80\c2<\06\01\04U\05\1b4\02\81\0e,\04d\0cV\0a\80\ae8\1d\0d,\04\09\07\02\0e\06\80\9a\83\d8\04\11\03\0d\03w\04_\06\0c\04\01\0f\0c\048\08\0a\06(\08\22N\81T\0c\1d\03\09\076\08\0e\04\09\07\09\07\80\cb%\0a\84\06library/core/src/unicode/unicode_data.rsSomeNone[\00\00\00\04\00\00\00\04\00\00\00d\00\00\00ErrorUtf8Errorvalid_up_toerror_len\00\00[\00\00\00\04\00\00\00\04\00\00\00e\00\00\00\dc$\10\00(\00\00\00P\00\00\00(\00\00\00\dc$\10\00(\00\00\00\5c\00\00\00\16\00\00\00\00\03\00\00\83\04 \00\91\05`\00]\13\a0\00\12\17 \1f\0c `\1f\ef,\a0+*0 ,o\a6\e0,\02\a8`-\1e\fb`.\00\fe 6\9e\ff`6\fd\01\e16\01\0a!7$\0d\e17\ab\0ea9/\18\a190\1caH\f3\1e\a1L@4aP\f0j\a1QOo!R\9d\bc\a1R\00\cfaSe\d1\a1S\00\da!T\00\e0\e1U\ae\e2aW\ec\e4!Y\d0\e8\a1Y \00\eeY\f0\01\7fZ\00p\00\07\00-\01\01\01\02\01\02\01\01H\0b0\15\10\01e\07\02\06\02\02\01\04#\01\1e\1b[\0b:\09\09\01\18\04\01\09\01\03\01\05+\03<\08*\18\01 7\01\01\01\04\08\04\01\03\07\0a\02\1d\01:\01\01\01\02\04\08\01\09\01\0a\02\1a\01\02\029\01\04\02\04\02\02\03\03\01\1e\02\03\01\0b\029\01\04\05\01\02\04\01\14\02\16\06\01\01:\01\01\02\01\04\08\01\07\03\0a\02\1e\01;\01\01\01\0c\01\09\01(\01\03\017\01\01\03\05\03\01\04\07\02\0b\02\1d\01:\01\02\01\02\01\03\01\05\02\07\02\0b\02\1c\029\02\01\01\02\04\08\01\09\01\0a\02\1d\01H\01\04\01\02\03\01\01\08\01Q\01\02\07\0c\08b\01\02\09\0b\07I\02\1b\01\01\01\01\017\0e\01\05\01\02\05\0b\01$\09\01f\04\01\06\01\02\02\02\19\02\04\03\10\04\0d\01\02\02\06\01\0f\01\00\03\00\03\1d\02\1e\02\1e\02@\02\01\07\08\01\02\0b\09\01-\03\01\01u\02\22\01v\03\04\02\09\01\06\03\db\02\02\01:\01\01\07\01\01\01\01\02\08\06\0a\02\010\1f1\040\07\01\01\05\01(\09\0c\02 \04\02\02\01\038\01\01\02\03\01\01\03:\08\02\02\98\03\01\0d\01\07\04\01\06\01\03\02\c6@\00\01\c3!\00\03\8d\01` \00\06i\02\00\04\01\0a \02P\02\00\01\03\01\04\01\19\02\05\01\97\02\1a\12\0d\01&\08\19\0b.\030\01\02\04\02\02'\01C\06\02\02\02\02\0c\01\08\01/\013\01\01\03\02\02\05\02\01\01*\02\08\01\ee\01\02\01\04\01\00\01\00\10\10\10\00\02\00\01\e2\01\95\05\00\03\01\02\05\04(\03\04\01\a5\02\00\04\00\02P\03F\0b1\04{\016\0f)\01\02\02\0a\031\04\02\02\07\01=\03$\05\01\08>\01\0c\024\09\0a\04\02\01_\03\02\01\01\02\06\01\02\01\9d\01\03\08\15\029\02\01\01\01\01\16\01\0e\07\03\05\c3\08\02\03\01\01\17\01Q\01\02\06\01\01\02\01\01\02\01\02\eb\01\02\04\06\02\01\02\1b\02U\08\02\01\01\02j\01\01\01\02\06\01\01e\03\02\04\01\05\00\09\01\02\f5\01\0a\02\01\01\04\01\90\04\02\02\04\01 \0a(\06\02\04\08\01\09\06\02\03.\0d\01\02\00\07\01\06\01\01R\16\02\07\01\02\01\02z\06\03\01\01\02\01\07\01\01H\02\03\01\01\01\00\02\0b\024\05\05\01\01\01\00\01\06\0f\00\05;\07\00\01?\04Q\01\00\02\00.\02\17\00\01\01\03\04\05\08\08\02\07\1e\04\94\03\007\042\08\01\0e\01\16\05\01\0f\00\07\01\11\02\07\01\02\01\05d\01\a0\07\00\01=\04\00\04\00\07m\07\00`\80\f0\00LayoutError")
  (data $.data (i32.const 1059032) "\01\00\00\00\ff\ff\ff\ffX\10\10\00"))
