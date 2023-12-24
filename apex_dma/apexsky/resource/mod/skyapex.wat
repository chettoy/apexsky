(module
  (type (;0;) (func (param i32)))
  (type (;1;) (func (param i32 i32)))
  (type (;2;) (func (param i32 i32) (result i32)))
  (type (;3;) (func (param i32 i32 i32)))
  (type (;4;) (func (param i32 i32 i32) (result i32)))
  (type (;5;) (func (param i32 i32 i32 i32) (result i32)))
  (type (;6;) (func))
  (type (;7;) (func (param i32 i32 i32 i32)))
  (type (;8;) (func (result i32)))
  (type (;9;) (func (param i32) (result i32)))
  (type (;10;) (func (param i32 i32 i32 i32 i32 i32)))
  (type (;11;) (func (param i32 i32 i32 i32 i32) (result i32)))
  (type (;12;) (func (param i32 i32 i32 i32 i32)))
  (type (;13;) (func (param i32 i32 i32 i32 i32 i32 i32)))
  (type (;14;) (func (param i32 i32 i32 i32 i32 i32) (result i32)))
  (type (;15;) (func (param i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32) (result i32)))
  (type (;16;) (func (param i64 i32 i32) (result i32)))
  (import "wasi_snapshot_preview1" "fd_write" (func $_ZN4wasi13lib_generated22wasi_snapshot_preview18fd_write17h9f1c185719eccf6aE (;0;) (type 5)))
  (import "wasi_snapshot_preview1" "environ_get" (func $__imported_wasi_snapshot_preview1_environ_get (;1;) (type 2)))
  (import "wasi_snapshot_preview1" "environ_sizes_get" (func $__imported_wasi_snapshot_preview1_environ_sizes_get (;2;) (type 2)))
  (import "wasi_snapshot_preview1" "proc_exit" (func $__imported_wasi_snapshot_preview1_proc_exit (;3;) (type 0)))
  (func $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h4f9db6b1bee3dfa0E (;4;) (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 0
    i32.load offset=4
    local.get 1
    call $_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17hbfdf58a1d1fe1e23E
  )
  (func $_ZN6obfstr4xref5inner17hc8431023c40b57dbE (;5;) (type 2) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.const -418714061
    i32.mul
    i32.const -784755689
    i32.xor
    i32.const 328965658
    i32.mul
    local.tee 1
    i32.const 6
    i32.rotl
    local.get 1
    i32.xor
    i32.const 65535
    i32.and
    i32.add
  )
  (func $load (;6;) (type 6))
  (func $add (;7;) (type 2) (param i32 i32) (result i32)
    local.get 1
    local.get 0
    i32.add
  )
  (func $print_run_as_root (;8;) (type 6)
    (local i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 64
    i32.sub
    local.tee 0
    global.set $__stack_pointer
    local.get 0
    i32.const 994061
    i32.store offset=4
    local.get 0
    i32.const 4
    i32.add
    local.set 1
    local.get 0
    i32.load offset=4
    local.set 2
    local.get 0
    i32.const 1678799685
    i32.store offset=4
    local.get 0
    i32.load offset=4
    local.set 1
    local.get 0
    i32.const 16
    i32.add
    i64.const 1
    i64.store align=4
    local.get 2
    local.get 1
    call $_ZN6obfstr4xref5inner17hc8431023c40b57dbE
    local.tee 1
    i32.load align=1
    local.set 2
    local.get 1
    i32.load offset=4 align=1
    local.set 3
    local.get 1
    i32.load offset=8 align=1
    local.set 4
    local.get 1
    i32.load offset=12 align=1
    local.set 5
    local.get 0
    i32.const 61
    i32.add
    local.get 1
    i32.load16_u offset=16 align=1
    local.tee 6
    i32.const 8
    i32.shr_u
    i32.const 134
    i32.xor
    i32.store8
    local.get 0
    i32.const 19
    i32.store offset=40
    local.get 0
    i32.const 1
    i32.store offset=32
    local.get 0
    i32.const 2
    i32.store offset=8
    local.get 0
    i32.const 1048580
    i32.store offset=4
    local.get 0
    local.get 2
    i32.const 514936862
    i32.xor
    i32.store offset=44
    local.get 0
    local.get 3
    i32.const -1562131801
    i32.xor
    i32.store offset=48
    local.get 0
    local.get 4
    i32.const -1939951492
    i32.xor
    i32.store offset=52
    local.get 0
    local.get 5
    i32.const 275953620
    i32.xor
    i32.store offset=56
    local.get 0
    local.get 6
    i32.const 144
    i32.xor
    i32.store8 offset=60
    local.get 0
    local.get 1
    i32.const 18
    i32.add
    i32.load8_u
    i32.const 138
    i32.xor
    i32.store8 offset=62
    local.get 0
    local.get 0
    i32.const 44
    i32.add
    i32.store offset=36
    local.get 0
    local.get 0
    i32.const 36
    i32.add
    i32.store offset=28
    local.get 0
    local.get 0
    i32.const 28
    i32.add
    i32.store offset=12
    local.get 0
    i32.const 4
    i32.add
    call $_ZN3std2io5stdio6_print17hda456e4879685b1fE
    local.get 0
    i32.const 64
    i32.add
    global.set $__stack_pointer
  )
  (func $__rust_alloc (;9;) (type 2) (param i32 i32) (result i32)
    (local i32)
    local.get 0
    local.get 1
    call $__rdl_alloc
    local.set 2
    local.get 2
    return
  )
  (func $__rust_dealloc (;10;) (type 3) (param i32 i32 i32)
    local.get 0
    local.get 1
    local.get 2
    call $__rdl_dealloc
    return
  )
  (func $__rust_realloc (;11;) (type 5) (param i32 i32 i32 i32) (result i32)
    (local i32)
    local.get 0
    local.get 1
    local.get 2
    local.get 3
    call $__rdl_realloc
    local.set 4
    local.get 4
    return
  )
  (func $__rust_alloc_error_handler (;12;) (type 1) (param i32 i32)
    local.get 0
    local.get 1
    call $__rg_oom
    return
  )
  (func $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h1118ca1ed69c577bE (;13;) (type 1) (param i32 i32)
    local.get 0
    i64.const -163230743173927068
    i64.store offset=8
    local.get 0
    i64.const -4493808902380553279
    i64.store
  )
  (func $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h142b3648bf87ee57E (;14;) (type 1) (param i32 i32)
    local.get 0
    i64.const -5054324659421563287
    i64.store offset=8
    local.get 0
    i64.const 144823174312744008
    i64.store
  )
  (func $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h59be72fef6d3ef6eE (;15;) (type 1) (param i32 i32)
    local.get 0
    i64.const 834069368081456416
    i64.store offset=8
    local.get 0
    i64.const 607490806584822236
    i64.store
  )
  (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17ha103d6456042d7d4E (;16;) (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 1
    call $_ZN43_$LT$bool$u20$as$u20$core..fmt..Display$GT$3fmt17hfba4e42b73edb2ecE
  )
  (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17had20c8f17260966eE (;17;) (type 2) (param i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        local.get 0
        i32.load
        local.tee 0
        i32.load8_u
        br_if 0 (;@2;)
        local.get 1
        i32.const 1049012
        i32.const 4
        call $_ZN4core3fmt9Formatter9write_str17h8cfbd807d1bba582E
        local.set 0
        br 1 (;@1;)
      end
      local.get 2
      local.get 0
      i32.const 1
      i32.add
      i32.store offset=12
      local.get 1
      i32.const 1049016
      i32.const 4
      local.get 2
      i32.const 12
      i32.add
      i32.const 1048616
      call $_ZN4core3fmt9Formatter25debug_tuple_field1_finish17hc9e36c10b865cf5dE
      local.set 0
    end
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 0
  )
  (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hcc84fdff291ecc01E (;18;) (type 2) (param i32 i32) (result i32)
    (local i32)
    local.get 0
    i32.load
    local.set 0
    block ;; label = @1
      local.get 1
      i32.load offset=28
      local.tee 2
      i32.const 16
      i32.and
      br_if 0 (;@1;)
      block ;; label = @2
        local.get 2
        i32.const 32
        i32.and
        br_if 0 (;@2;)
        local.get 0
        local.get 1
        call $_ZN4core3fmt3num3imp51_$LT$impl$u20$core..fmt..Display$u20$for$u20$u8$GT$3fmt17h27012b48f7c972baE
        return
      end
      local.get 0
      local.get 1
      call $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i8$GT$3fmt17h06b29036c0557900E
      return
    end
    local.get 0
    local.get 1
    call $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i8$GT$3fmt17hb1f2095a70886ad5E
  )
  (func $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h48cca9c67568fdc3E (;19;) (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 0
    i32.load offset=4
    local.get 1
    call $_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17hbfdf58a1d1fe1e23E
  )
  (func $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hd38fd9b0b0ba4b2eE (;20;) (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 1
    call $_ZN70_$LT$core..panic..location..Location$u20$as$u20$core..fmt..Display$GT$3fmt17h6e0a5b25ce921e81E
  )
  (func $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hdcca3a40c8b93508E (;21;) (type 2) (param i32 i32) (result i32)
    (local i32)
    block ;; label = @1
      local.get 1
      i32.load offset=28
      local.tee 2
      i32.const 16
      i32.and
      br_if 0 (;@1;)
      block ;; label = @2
        local.get 2
        i32.const 32
        i32.and
        br_if 0 (;@2;)
        local.get 0
        local.get 1
        call $_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17hbb0e20284f86c495E
        return
      end
      local.get 0
      local.get 1
      call $_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i32$GT$3fmt17hcc3212d2d4ae287dE
      return
    end
    local.get 0
    local.get 1
    call $_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i32$GT$3fmt17he478be80e254b577E
  )
  (func $_ZN4core3fmt5Write10write_char17h87516656917da6b5E (;22;) (type 2) (param i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    i32.const 0
    i32.store offset=12
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            local.get 1
            i32.const 128
            i32.lt_u
            br_if 0 (;@4;)
            local.get 1
            i32.const 2048
            i32.lt_u
            br_if 1 (;@3;)
            local.get 1
            i32.const 65536
            i32.ge_u
            br_if 2 (;@2;)
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
            br 3 (;@1;)
          end
          local.get 2
          local.get 1
          i32.store8 offset=12
          i32.const 1
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
    end
    local.get 0
    local.get 2
    i32.const 12
    i32.add
    local.get 1
    call $_ZN80_$LT$std..io..Write..write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17hdfe706db334d8d1fE
    local.set 1
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 1
  )
  (func $_ZN80_$LT$std..io..Write..write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17hdfe706db334d8d1fE (;23;) (type 4) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    i32.const 0
    local.set 4
    block ;; label = @1
      block ;; label = @2
        local.get 2
        i32.eqz
        br_if 0 (;@2;)
        block ;; label = @3
          loop ;; label = @4
            local.get 3
            local.get 2
            i32.store offset=4
            local.get 3
            local.get 1
            i32.store
            local.get 3
            i32.const 8
            i32.add
            i32.const 2
            local.get 3
            i32.const 1
            call $_ZN4wasi13lib_generated8fd_write17h5d845406ea9346aaE
            block ;; label = @5
              local.get 3
              i32.load16_u offset=8
              br_if 0 (;@5;)
              block ;; label = @6
                local.get 3
                i32.load offset=12
                local.tee 5
                br_if 0 (;@6;)
                i32.const 2
                local.set 2
                i32.const 1050560
                local.set 5
                br 3 (;@3;)
              end
              local.get 2
              local.get 5
              i32.lt_u
              br_if 4 (;@1;)
              local.get 1
              local.get 5
              i32.add
              local.set 1
              local.get 2
              local.get 5
              i32.sub
              local.tee 2
              br_if 1 (;@4;)
              br 3 (;@2;)
            end
            block ;; label = @5
              local.get 3
              i32.load16_u offset=10
              local.tee 5
              i32.const 27
              i32.ne
              br_if 0 (;@5;)
              local.get 2
              br_if 1 (;@4;)
              br 3 (;@2;)
            end
          end
          i32.const 0
          local.set 2
        end
        local.get 0
        i32.load offset=4
        local.set 4
        block ;; label = @3
          block ;; label = @4
            local.get 0
            i32.load8_u
            local.tee 1
            i32.const 4
            i32.gt_u
            br_if 0 (;@4;)
            local.get 1
            i32.const 3
            i32.ne
            br_if 1 (;@3;)
          end
          local.get 4
          i32.load
          local.tee 6
          local.get 4
          i32.const 4
          i32.add
          i32.load
          local.tee 1
          i32.load
          call_indirect (type 0)
          block ;; label = @4
            local.get 1
            i32.load offset=4
            local.tee 7
            i32.eqz
            br_if 0 (;@4;)
            local.get 6
            local.get 7
            local.get 1
            i32.load offset=8
            call $__rust_dealloc
          end
          local.get 4
          i32.const 12
          i32.const 4
          call $__rust_dealloc
        end
        local.get 0
        local.get 5
        i32.store offset=4
        local.get 0
        local.get 2
        i32.store
        i32.const 1
        local.set 4
      end
      local.get 3
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 4
      return
    end
    local.get 5
    local.get 2
    i32.const 1050572
    call $_ZN4core5slice5index26slice_start_index_len_fail17h46dad5e8fa8e060bE
    unreachable
  )
  (func $_ZN4core3fmt5Write10write_char17hb08d0817fd21c14dE (;24;) (type 2) (param i32 i32) (result i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    i32.const 0
    i32.store offset=12
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            local.get 1
            i32.const 128
            i32.lt_u
            br_if 0 (;@4;)
            local.get 1
            i32.const 2048
            i32.lt_u
            br_if 1 (;@3;)
            local.get 1
            i32.const 65536
            i32.ge_u
            br_if 2 (;@2;)
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
            br 3 (;@1;)
          end
          local.get 2
          local.get 1
          i32.store8 offset=12
          i32.const 1
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
    end
    block ;; label = @1
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
      call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17hb112330d3b6a189cE
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
    i32.const 0
  )
  (func $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17hb112330d3b6a189cE (;25;) (type 3) (param i32 i32 i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
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
        block ;; label = @3
          block ;; label = @4
            local.get 1
            br_if 0 (;@4;)
            local.get 3
            i32.const 0
            i32.store offset=24
            br 1 (;@3;)
          end
          local.get 3
          local.get 1
          i32.store offset=28
          local.get 3
          i32.const 1
          i32.store offset=24
          local.get 3
          local.get 0
          i32.load offset=4
          i32.store offset=20
        end
        local.get 3
        i32.const 8
        i32.add
        local.get 4
        local.get 2
        local.get 3
        i32.const 20
        i32.add
        call $_ZN5alloc7raw_vec11finish_grow17hb30d734150aed491E
        local.get 3
        i32.load offset=12
        local.set 1
        block ;; label = @3
          local.get 3
          i32.load offset=8
          br_if 0 (;@3;)
          local.get 0
          local.get 2
          i32.store
          local.get 0
          local.get 1
          i32.store offset=4
          br 2 (;@1;)
        end
        local.get 1
        i32.const -2147483647
        i32.eq
        br_if 1 (;@1;)
        local.get 1
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        local.get 3
        i32.const 16
        i32.add
        i32.load
        call $_ZN5alloc5alloc18handle_alloc_error17h76ec24bcd8f509d8E
        unreachable
      end
      call $_ZN5alloc7raw_vec17capacity_overflow17h4ae6daed6ccaada6E
      unreachable
    end
    local.get 3
    i32.const 32
    i32.add
    global.set $__stack_pointer
  )
  (func $_ZN4core3fmt5Write10write_char17hb4f3cb455ce5e67cE (;26;) (type 2) (param i32 i32) (result i32)
    (local i32 i32 i64 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    i32.const 0
    i32.store offset=4
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            local.get 1
            i32.const 128
            i32.lt_u
            br_if 0 (;@4;)
            local.get 1
            i32.const 2048
            i32.lt_u
            br_if 1 (;@3;)
            local.get 1
            i32.const 65536
            i32.ge_u
            br_if 2 (;@2;)
            local.get 2
            local.get 1
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=6
            local.get 2
            local.get 1
            i32.const 12
            i32.shr_u
            i32.const 224
            i32.or
            i32.store8 offset=4
            local.get 2
            local.get 1
            i32.const 6
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=5
            i32.const 3
            local.set 1
            br 3 (;@1;)
          end
          local.get 2
          local.get 1
          i32.store8 offset=4
          i32.const 1
          local.set 1
          br 2 (;@1;)
        end
        local.get 2
        local.get 1
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=5
        local.get 2
        local.get 1
        i32.const 6
        i32.shr_u
        i32.const 192
        i32.or
        i32.store8 offset=4
        i32.const 2
        local.set 1
        br 1 (;@1;)
      end
      local.get 2
      local.get 1
      i32.const 63
      i32.and
      i32.const 128
      i32.or
      i32.store8 offset=7
      local.get 2
      local.get 1
      i32.const 6
      i32.shr_u
      i32.const 63
      i32.and
      i32.const 128
      i32.or
      i32.store8 offset=6
      local.get 2
      local.get 1
      i32.const 12
      i32.shr_u
      i32.const 63
      i32.and
      i32.const 128
      i32.or
      i32.store8 offset=5
      local.get 2
      local.get 1
      i32.const 18
      i32.shr_u
      i32.const 7
      i32.and
      i32.const 240
      i32.or
      i32.store8 offset=4
      i32.const 4
      local.set 1
    end
    local.get 2
    i32.const 8
    i32.add
    local.get 0
    i32.load offset=8
    local.get 2
    i32.const 4
    i32.add
    local.get 1
    call $_ZN61_$LT$std..io..stdio..StdoutLock$u20$as$u20$std..io..Write$GT$9write_all17h0816e60e5bb42170E
    block ;; label = @1
      local.get 2
      i32.load8_u offset=8
      local.tee 1
      i32.const 4
      i32.eq
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=4
      local.set 3
      local.get 2
      i64.load offset=8
      local.set 4
      block ;; label = @2
        block ;; label = @3
          local.get 0
          i32.load8_u
          local.tee 5
          i32.const 4
          i32.gt_u
          br_if 0 (;@3;)
          local.get 5
          i32.const 3
          i32.ne
          br_if 1 (;@2;)
        end
        local.get 3
        i32.load
        local.tee 6
        local.get 3
        i32.const 4
        i32.add
        i32.load
        local.tee 5
        i32.load
        call_indirect (type 0)
        block ;; label = @3
          local.get 5
          i32.load offset=4
          local.tee 7
          i32.eqz
          br_if 0 (;@3;)
          local.get 6
          local.get 7
          local.get 5
          i32.load offset=8
          call $__rust_dealloc
        end
        local.get 3
        i32.const 12
        i32.const 4
        call $__rust_dealloc
      end
      local.get 0
      local.get 4
      i64.store align=4
    end
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 1
    i32.const 4
    i32.ne
  )
  (func $_ZN61_$LT$std..io..stdio..StdoutLock$u20$as$u20$std..io..Write$GT$9write_all17h0816e60e5bb42170E (;27;) (type 7) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                local.get 1
                i32.load
                local.tee 5
                i32.load offset=8
                br_if 0 (;@6;)
                local.get 5
                i32.const -1
                i32.store offset=8
                local.get 4
                i32.const 8
                i32.add
                i32.const 10
                local.get 2
                local.get 3
                call $_ZN4core5slice6memchr7memrchr17h6febb542c92544afE
                local.get 4
                i32.load offset=8
                br_if 1 (;@5;)
                block ;; label = @7
                  local.get 5
                  i32.const 20
                  i32.add
                  i32.load
                  local.tee 6
                  br_if 0 (;@7;)
                  i32.const 0
                  local.set 6
                  br 4 (;@3;)
                end
                local.get 6
                local.get 5
                i32.const 16
                i32.add
                i32.load
                local.tee 7
                i32.add
                i32.const -1
                i32.add
                i32.load8_u
                i32.const 10
                i32.ne
                br_if 3 (;@3;)
                i32.const 0
                local.set 1
                loop ;; label = @7
                  local.get 4
                  local.get 6
                  local.get 1
                  i32.sub
                  local.tee 8
                  i32.store offset=20
                  local.get 4
                  local.get 7
                  local.get 1
                  i32.add
                  local.tee 9
                  i32.store offset=16
                  local.get 4
                  i32.const 24
                  i32.add
                  i32.const 1
                  local.get 4
                  i32.const 16
                  i32.add
                  i32.const 1
                  call $_ZN4wasi13lib_generated8fd_write17h5d845406ea9346aaE
                  block ;; label = @8
                    block ;; label = @9
                      block ;; label = @10
                        block ;; label = @11
                          block ;; label = @12
                            local.get 4
                            i32.load16_u offset=24
                            br_if 0 (;@12;)
                            local.get 4
                            i32.load offset=28
                            local.set 10
                            br 1 (;@11;)
                          end
                          local.get 8
                          local.set 10
                          local.get 4
                          i32.load16_u offset=26
                          local.tee 11
                          i32.const 8
                          i32.eq
                          br_if 0 (;@11;)
                          local.get 5
                          i32.const 0
                          i32.store8 offset=24
                          local.get 11
                          i32.const 27
                          i32.eq
                          br_if 3 (;@8;)
                          i32.const 0
                          local.set 6
                          br 1 (;@10;)
                        end
                        local.get 5
                        i32.const 0
                        i32.store8 offset=24
                        local.get 10
                        br_if 1 (;@9;)
                        i32.const 1049356
                        local.set 11
                        i32.const 2
                        local.set 6
                      end
                      block ;; label = @10
                        local.get 1
                        i32.eqz
                        br_if 0 (;@10;)
                        local.get 7
                        local.get 9
                        local.get 8
                        call $memmove
                        drop
                        local.get 5
                        local.get 8
                        i32.store offset=20
                      end
                      local.get 0
                      i32.const 0
                      i32.store16 offset=1 align=1
                      local.get 0
                      local.get 11
                      i32.store offset=4
                      local.get 0
                      local.get 6
                      i32.store8
                      local.get 0
                      i32.const 3
                      i32.add
                      i32.const 0
                      i32.store8
                      br 7 (;@2;)
                    end
                    local.get 10
                    local.get 1
                    i32.add
                    local.set 1
                  end
                  local.get 1
                  local.get 6
                  i32.ge_u
                  br_if 3 (;@4;)
                  br 0 (;@7;)
                end
              end
              i32.const 1050256
              call $_ZN4core4cell22panic_already_borrowed17ha086677c15c4e605E
              unreachable
            end
            block ;; label = @5
              local.get 3
              local.get 4
              i32.load offset=12
              i32.const 1
              i32.add
              local.tee 12
              i32.lt_u
              br_if 0 (;@5;)
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    local.get 5
                    i32.const 20
                    i32.add
                    i32.load
                    local.tee 1
                    br_if 0 (;@8;)
                    local.get 12
                    i32.eqz
                    br_if 1 (;@7;)
                    local.get 2
                    local.set 6
                    local.get 12
                    local.set 1
                    loop ;; label = @9
                      local.get 4
                      local.get 1
                      i32.store offset=20
                      local.get 4
                      local.get 6
                      i32.store offset=16
                      local.get 4
                      i32.const 24
                      i32.add
                      i32.const 1
                      local.get 4
                      i32.const 16
                      i32.add
                      i32.const 1
                      call $_ZN4wasi13lib_generated8fd_write17h5d845406ea9346aaE
                      block ;; label = @10
                        block ;; label = @11
                          block ;; label = @12
                            local.get 4
                            i32.load16_u offset=24
                            br_if 0 (;@12;)
                            block ;; label = @13
                              local.get 4
                              i32.load offset=28
                              local.tee 10
                              br_if 0 (;@13;)
                              i32.const 1050560
                              local.set 10
                              i32.const 2
                              local.set 1
                              br 7 (;@6;)
                            end
                            local.get 1
                            local.get 10
                            i32.lt_u
                            br_if 1 (;@11;)
                            local.get 6
                            local.get 10
                            i32.add
                            local.set 6
                            local.get 1
                            local.get 10
                            i32.sub
                            local.set 1
                            br 2 (;@10;)
                          end
                          local.get 4
                          i32.load16_u offset=26
                          local.tee 10
                          i32.const 27
                          i32.eq
                          br_if 1 (;@10;)
                          local.get 10
                          i32.const 8
                          i32.eq
                          br_if 4 (;@7;)
                          i32.const 0
                          local.set 1
                          br 5 (;@6;)
                        end
                        local.get 10
                        local.get 1
                        i32.const 1050572
                        call $_ZN4core5slice5index26slice_start_index_len_fail17h46dad5e8fa8e060bE
                        unreachable
                      end
                      local.get 1
                      br_if 0 (;@9;)
                      br 2 (;@7;)
                    end
                  end
                  block ;; label = @8
                    block ;; label = @9
                      local.get 5
                      i32.load offset=12
                      local.get 1
                      i32.sub
                      local.get 12
                      i32.le_u
                      br_if 0 (;@9;)
                      local.get 5
                      i32.const 16
                      i32.add
                      i32.load
                      local.get 1
                      i32.add
                      local.get 2
                      local.get 12
                      call $memcpy
                      drop
                      local.get 5
                      local.get 1
                      local.get 12
                      i32.add
                      local.tee 10
                      i32.store offset=20
                      br 1 (;@8;)
                    end
                    local.get 4
                    i32.const 24
                    i32.add
                    local.get 5
                    i32.const 12
                    i32.add
                    local.get 2
                    local.get 12
                    call $_ZN3std2io8buffered9bufwriter18BufWriter$LT$W$GT$14write_all_cold17h5afadb4f0a3ab020E
                    block ;; label = @9
                      local.get 4
                      i32.load8_u offset=24
                      local.tee 1
                      i32.const 4
                      i32.eq
                      br_if 0 (;@9;)
                      local.get 0
                      local.get 4
                      i32.load offset=25 align=1
                      i32.store offset=1 align=1
                      local.get 0
                      i32.const 4
                      i32.add
                      local.get 4
                      i32.const 24
                      i32.add
                      i32.const 4
                      i32.add
                      i32.load align=1
                      i32.store align=1
                      local.get 0
                      local.get 1
                      i32.store8
                      br 7 (;@2;)
                    end
                    local.get 5
                    i32.load offset=20
                    local.set 10
                  end
                  local.get 10
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 5
                  i32.const 16
                  i32.add
                  i32.load
                  local.set 7
                  i32.const 0
                  local.set 1
                  loop ;; label = @8
                    local.get 4
                    local.get 10
                    local.get 1
                    i32.sub
                    local.tee 8
                    i32.store offset=20
                    local.get 4
                    local.get 7
                    local.get 1
                    i32.add
                    local.tee 9
                    i32.store offset=16
                    local.get 4
                    i32.const 24
                    i32.add
                    i32.const 1
                    local.get 4
                    i32.const 16
                    i32.add
                    i32.const 1
                    call $_ZN4wasi13lib_generated8fd_write17h5d845406ea9346aaE
                    block ;; label = @9
                      block ;; label = @10
                        block ;; label = @11
                          block ;; label = @12
                            block ;; label = @13
                              local.get 4
                              i32.load16_u offset=24
                              br_if 0 (;@13;)
                              local.get 4
                              i32.load offset=28
                              local.set 6
                              br 1 (;@12;)
                            end
                            local.get 8
                            local.set 6
                            local.get 4
                            i32.load16_u offset=26
                            local.tee 11
                            i32.const 8
                            i32.eq
                            br_if 0 (;@12;)
                            local.get 5
                            i32.const 0
                            i32.store8 offset=24
                            local.get 11
                            i32.const 27
                            i32.eq
                            br_if 3 (;@9;)
                            i32.const 0
                            local.set 6
                            br 1 (;@11;)
                          end
                          local.get 5
                          i32.const 0
                          i32.store8 offset=24
                          local.get 6
                          br_if 1 (;@10;)
                          i32.const 1049356
                          local.set 11
                          i32.const 2
                          local.set 6
                        end
                        block ;; label = @11
                          local.get 1
                          i32.eqz
                          br_if 0 (;@11;)
                          local.get 7
                          local.get 9
                          local.get 8
                          call $memmove
                          drop
                          local.get 5
                          local.get 8
                          i32.store offset=20
                        end
                        local.get 0
                        i32.const 0
                        i32.store16 offset=1 align=1
                        local.get 0
                        local.get 11
                        i32.store offset=4
                        local.get 0
                        local.get 6
                        i32.store8
                        local.get 0
                        i32.const 3
                        i32.add
                        i32.const 0
                        i32.store8
                        br 8 (;@2;)
                      end
                      local.get 6
                      local.get 1
                      i32.add
                      local.set 1
                    end
                    local.get 1
                    local.get 10
                    i32.lt_u
                    br_if 0 (;@8;)
                  end
                  block ;; label = @8
                    local.get 10
                    local.get 1
                    i32.lt_u
                    br_if 0 (;@8;)
                    local.get 5
                    i32.const 0
                    i32.store offset=20
                    br 1 (;@7;)
                  end
                  local.get 1
                  local.get 10
                  i32.const 1048932
                  call $_ZN4core5slice5index24slice_end_index_len_fail17hdd28afa5458bc752E
                  unreachable
                end
                local.get 2
                local.get 12
                i32.add
                local.set 10
                block ;; label = @7
                  local.get 5
                  i32.load offset=12
                  local.get 5
                  i32.load offset=20
                  local.tee 6
                  i32.sub
                  local.get 3
                  local.get 12
                  i32.sub
                  local.tee 1
                  i32.gt_u
                  br_if 0 (;@7;)
                  local.get 0
                  local.get 5
                  i32.const 12
                  i32.add
                  local.get 10
                  local.get 1
                  call $_ZN3std2io8buffered9bufwriter18BufWriter$LT$W$GT$14write_all_cold17h5afadb4f0a3ab020E
                  br 5 (;@2;)
                end
                local.get 5
                i32.const 16
                i32.add
                i32.load
                local.get 6
                i32.add
                local.get 10
                local.get 1
                call $memcpy
                drop
                local.get 0
                i32.const 4
                i32.store8
                local.get 5
                local.get 6
                local.get 1
                i32.add
                i32.store offset=20
                br 4 (;@2;)
              end
              local.get 0
              i32.const 0
              i32.store16 offset=1 align=1
              local.get 0
              local.get 10
              i32.store offset=4
              local.get 0
              local.get 1
              i32.store8
              local.get 0
              i32.const 3
              i32.add
              i32.const 0
              i32.store8
              br 3 (;@2;)
            end
            i32.const 1049285
            i32.const 35
            i32.const 1049424
            call $_ZN4core9panicking5panic17h89e93701a34d6dc7E
            unreachable
          end
          local.get 6
          local.get 1
          i32.lt_u
          br_if 2 (;@1;)
          i32.const 0
          local.set 6
          local.get 5
          i32.const 0
          i32.store offset=20
        end
        block ;; label = @3
          local.get 5
          i32.load offset=12
          local.get 6
          i32.sub
          local.get 3
          i32.gt_u
          br_if 0 (;@3;)
          local.get 0
          local.get 5
          i32.const 12
          i32.add
          local.get 2
          local.get 3
          call $_ZN3std2io8buffered9bufwriter18BufWriter$LT$W$GT$14write_all_cold17h5afadb4f0a3ab020E
          br 1 (;@2;)
        end
        local.get 5
        i32.const 16
        i32.add
        i32.load
        local.get 6
        i32.add
        local.get 2
        local.get 3
        call $memcpy
        drop
        local.get 0
        i32.const 4
        i32.store8
        local.get 5
        local.get 6
        local.get 3
        i32.add
        i32.store offset=20
      end
      local.get 5
      local.get 5
      i32.load offset=8
      i32.const 1
      i32.add
      i32.store offset=8
      local.get 4
      i32.const 32
      i32.add
      global.set $__stack_pointer
      return
    end
    local.get 1
    local.get 6
    i32.const 1048932
    call $_ZN4core5slice5index24slice_end_index_len_fail17hdd28afa5458bc752E
    unreachable
  )
  (func $_ZN4core3fmt5Write9write_fmt17h027f44fd7eec0aa4E (;28;) (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.const 1049092
    local.get 1
    call $_ZN4core3fmt5write17h2d8708cc7ff40a92E
  )
  (func $_ZN4core3fmt5Write9write_fmt17h5539445f3fc48d14E (;29;) (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.const 1049068
    local.get 1
    call $_ZN4core3fmt5write17h2d8708cc7ff40a92E
  )
  (func $_ZN4core3fmt5Write9write_fmt17h742432808baa5dd2E (;30;) (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.const 1049044
    local.get 1
    call $_ZN4core3fmt5write17h2d8708cc7ff40a92E
  )
  (func $_ZN4core3fmt5Write9write_fmt17h7e18207dce1cf897E (;31;) (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.const 1049020
    local.get 1
    call $_ZN4core3fmt5write17h2d8708cc7ff40a92E
  )
  (func $_ZN3std9panicking12default_hook17h37c9d1490f380811E (;32;) (type 0) (param i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 112
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        local.get 0
        i32.load8_u offset=17
        br_if 0 (;@2;)
        block ;; label = @3
          i32.const 0
          i32.load offset=1055504
          i32.const 1
          i32.gt_u
          br_if 0 (;@3;)
          local.get 1
          call $_ZN3std5panic19get_backtrace_style17he6d3ee8cbcc9f914E
          i32.store8 offset=35
          br 2 (;@1;)
        end
        local.get 1
        i32.const 1
        i32.store8 offset=35
        br 1 (;@1;)
      end
      local.get 1
      i32.const 3
      i32.store8 offset=35
    end
    local.get 1
    local.get 0
    i32.load offset=12
    i32.store offset=36
    i32.const 12
    local.set 2
    local.get 1
    i32.const 16
    i32.add
    local.get 0
    i32.load
    local.tee 3
    local.get 0
    i32.load offset=4
    i32.const 12
    i32.add
    local.tee 0
    i32.load
    call_indirect (type 1)
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 1
          i64.load offset=16
          i64.const -4493808902380553279
          i64.xor
          local.get 1
          i32.const 16
          i32.add
          i32.const 8
          i32.add
          i64.load
          i64.const -163230743173927068
          i64.xor
          i64.or
          i64.eqz
          br_if 0 (;@3;)
          local.get 1
          local.get 3
          local.get 0
          i32.load
          call_indirect (type 1)
          i32.const 1051260
          local.set 0
          local.get 1
          i64.load
          i64.const 144823174312744008
          i64.xor
          local.get 1
          i32.const 8
          i32.add
          i64.load
          i64.const -5054324659421563287
          i64.xor
          i64.or
          i64.eqz
          i32.eqz
          br_if 2 (;@1;)
          local.get 3
          i32.const 8
          i32.add
          local.set 0
          local.get 3
          i32.const 4
          i32.add
          local.set 3
          br 1 (;@2;)
        end
        local.get 3
        i32.const 4
        i32.add
        local.set 0
      end
      local.get 0
      i32.load
      local.set 2
      local.get 3
      i32.load
      local.set 0
    end
    local.get 1
    local.get 2
    i32.store offset=44
    local.get 1
    local.get 0
    i32.store offset=40
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  i32.const 0
                  i32.load offset=1055512
                  local.tee 0
                  br_if 0 (;@7;)
                  call $_ZN4core4cell4once17OnceCell$LT$T$GT$15get_or_try_init13outlined_call17he51936b7cd3d2a1dE
                  local.set 0
                  i32.const 0
                  i32.load offset=1055512
                  br_if 1 (;@6;)
                  i32.const 0
                  local.get 0
                  i32.store offset=1055512
                end
                local.get 0
                local.get 0
                i32.load
                local.tee 3
                i32.const 1
                i32.add
                i32.store
                local.get 3
                i32.const -1
                i32.le_s
                br_if 1 (;@5;)
                local.get 1
                local.get 0
                i32.store offset=48
                block ;; label = @7
                  block ;; label = @8
                    local.get 0
                    i32.const 16
                    i32.add
                    i32.load
                    local.tee 3
                    br_if 0 (;@8;)
                    i32.const 9
                    local.set 2
                    i32.const 1051272
                    local.set 3
                    br 1 (;@7;)
                  end
                  local.get 0
                  i32.const 20
                  i32.add
                  i32.load
                  i32.const -1
                  i32.add
                  local.set 2
                end
                local.get 1
                local.get 2
                i32.store offset=56
                local.get 1
                local.get 3
                i32.store offset=52
                local.get 1
                local.get 1
                i32.const 35
                i32.add
                i32.store offset=72
                local.get 1
                local.get 1
                i32.const 40
                i32.add
                i32.store offset=68
                local.get 1
                local.get 1
                i32.const 36
                i32.add
                i32.store offset=64
                local.get 1
                local.get 1
                i32.const 52
                i32.add
                i32.store offset=60
                block ;; label = @7
                  i32.const 0
                  i32.load8_u offset=1055426
                  br_if 0 (;@7;)
                  local.get 1
                  i32.const 0
                  i32.store offset=76
                  br 4 (;@3;)
                end
                i32.const 0
                i32.const 1
                i32.store8 offset=1055426
                block ;; label = @7
                  i32.const 0
                  i32.load8_u offset=1055520
                  br_if 0 (;@7;)
                  i32.const 0
                  i32.const 1
                  i32.store8 offset=1055520
                  i32.const 0
                  i32.const 0
                  i32.store offset=1055524
                  local.get 1
                  i32.const 0
                  i32.store offset=76
                  br 4 (;@3;)
                end
                local.get 1
                i32.const 0
                i32.load offset=1055524
                local.tee 3
                i32.store offset=76
                i32.const 0
                i32.const 0
                i32.store offset=1055524
                local.get 3
                i32.eqz
                br_if 3 (;@3;)
                local.get 3
                i32.load8_u offset=8
                local.set 0
                local.get 3
                i32.const 1
                i32.store8 offset=8
                local.get 1
                local.get 0
                i32.store8 offset=83
                local.get 0
                br_if 2 (;@4;)
                block ;; label = @7
                  block ;; label = @8
                    block ;; label = @9
                      i32.const 0
                      i32.load offset=1055492
                      i32.const 2147483647
                      i32.and
                      br_if 0 (;@9;)
                      local.get 1
                      i32.const 60
                      i32.add
                      local.get 3
                      i32.const 12
                      i32.add
                      i32.const 1051284
                      call $_ZN3std9panicking12default_hook28_$u7b$$u7b$closure$u7d$$u7d$17h3542a0b08d22dec8E
                      local.get 3
                      i32.const 9
                      i32.add
                      local.set 0
                      br 1 (;@8;)
                    end
                    call $_ZN3std9panicking11panic_count17is_zero_slow_path17hb2a28c6a6d133550E
                    local.set 0
                    local.get 1
                    i32.const 60
                    i32.add
                    local.get 3
                    i32.const 12
                    i32.add
                    i32.const 1051284
                    call $_ZN3std9panicking12default_hook28_$u7b$$u7b$closure$u7d$$u7d$17h3542a0b08d22dec8E
                    local.get 0
                    i32.eqz
                    br_if 1 (;@7;)
                    local.get 3
                    i32.const 9
                    i32.add
                    local.set 0
                  end
                  i32.const 0
                  i32.load offset=1055492
                  i32.const 2147483647
                  i32.and
                  i32.eqz
                  br_if 0 (;@7;)
                  call $_ZN3std9panicking11panic_count17is_zero_slow_path17hb2a28c6a6d133550E
                  br_if 0 (;@7;)
                  local.get 0
                  i32.const 1
                  i32.store8
                end
                local.get 3
                i32.const 0
                i32.store8 offset=8
                i32.const 0
                i32.const 1
                i32.store8 offset=1055426
                block ;; label = @7
                  block ;; label = @8
                    i32.const 0
                    i32.load8_u offset=1055520
                    br_if 0 (;@8;)
                    i32.const 0
                    local.get 3
                    i32.store offset=1055524
                    i32.const 0
                    i32.const 1
                    i32.store8 offset=1055520
                    br 1 (;@7;)
                  end
                  i32.const 0
                  i32.load offset=1055524
                  local.set 0
                  i32.const 0
                  local.get 3
                  i32.store offset=1055524
                  local.get 1
                  local.get 0
                  i32.store offset=84
                  local.get 0
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 0
                  local.get 0
                  i32.load
                  local.tee 3
                  i32.const -1
                  i32.add
                  i32.store
                  local.get 3
                  i32.const 1
                  i32.ne
                  br_if 0 (;@7;)
                  local.get 1
                  i32.const 84
                  i32.add
                  call $_ZN5alloc4sync16Arc$LT$T$C$A$GT$9drop_slow17hb57cee88e32714f6E
                end
                i32.const 1
                local.set 3
                local.get 1
                i32.load offset=48
                local.tee 0
                i32.eqz
                br_if 5 (;@1;)
                br 4 (;@2;)
              end
              local.get 1
              i32.const 96
              i32.add
              i64.const 0
              i64.store align=4
              local.get 1
              i32.const 1
              i32.store offset=88
              local.get 1
              i32.const 1048648
              i32.store offset=84
              local.get 1
              i32.const 1048632
              i32.store offset=92
              local.get 1
              i32.const 84
              i32.add
              i32.const 1048736
              call $_ZN4core9panicking9panic_fmt17h2298343e5f8a8826E
              unreachable
            end
            unreachable
            unreachable
          end
          local.get 1
          i64.const 0
          i64.store offset=96 align=4
          local.get 1
          i32.const 1048632
          i32.store offset=92
          local.get 1
          i32.const 1
          i32.store offset=88
          local.get 1
          i32.const 1050692
          i32.store offset=84
          local.get 1
          i32.const 83
          i32.add
          local.get 1
          i32.const 84
          i32.add
          call $_ZN4core9panicking13assert_failed17h36d5dfceec7301c8E
          unreachable
        end
        local.get 1
        i32.const 60
        i32.add
        local.get 1
        i32.const 111
        i32.add
        i32.const 1051324
        call $_ZN3std9panicking12default_hook28_$u7b$$u7b$closure$u7d$$u7d$17h3542a0b08d22dec8E
        i32.const 0
        local.set 3
      end
      local.get 0
      local.get 0
      i32.load
      local.tee 2
      i32.const -1
      i32.add
      i32.store
      block ;; label = @2
        local.get 2
        i32.const 1
        i32.ne
        br_if 0 (;@2;)
        local.get 1
        i32.const 48
        i32.add
        call $_ZN5alloc4sync16Arc$LT$T$C$A$GT$9drop_slow17h2aca64a119bc7c84E
      end
      local.get 3
      i32.const -1
      i32.xor
      local.get 1
      i32.load offset=76
      local.tee 0
      i32.const 0
      i32.ne
      i32.and
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      local.get 0
      i32.load
      local.tee 3
      i32.const -1
      i32.add
      i32.store
      local.get 3
      i32.const 1
      i32.ne
      br_if 0 (;@1;)
      local.get 1
      i32.const 76
      i32.add
      call $_ZN5alloc4sync16Arc$LT$T$C$A$GT$9drop_slow17hb57cee88e32714f6E
    end
    local.get 1
    i32.const 112
    i32.add
    global.set $__stack_pointer
  )
  (func $_ZN4core3ptr122drop_in_place$LT$$RF$alloc..boxed..Box$LT$dyn$u20$core..error..Error$u2b$core..marker..Sync$u2b$core..marker..Send$GT$$GT$17hf364da63891e5e4bE (;33;) (type 0) (param i32))
  (func $_ZN3std9panicking11panic_count17is_zero_slow_path17hb2a28c6a6d133550E (;34;) (type 8) (result i32)
    i32.const 0
    i32.load offset=1055504
    i32.eqz
  )
  (func $_ZN4core3ptr29drop_in_place$LT$$LP$$RP$$GT$17h7fb52925df5f1f21E (;35;) (type 0) (param i32))
  (func $_ZN5alloc4sync16Arc$LT$T$C$A$GT$9drop_slow17h2aca64a119bc7c84E (;36;) (type 0) (param i32)
    (local i32 i32)
    block ;; label = @1
      local.get 0
      i32.load
      local.tee 0
      i32.const 16
      i32.add
      i32.load
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.const 20
      i32.add
      i32.load
      local.set 2
      local.get 1
      i32.const 0
      i32.store8
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      local.get 2
      i32.const 1
      call $__rust_dealloc
    end
    block ;; label = @1
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
      i32.const 8
      call $__rust_dealloc
    end
  )
  (func $_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h349d8f91f5ab28e4E (;37;) (type 0) (param i32)
    (local i32)
    block ;; label = @1
      local.get 0
      i32.load
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=4
      local.get 1
      i32.const 1
      call $__rust_dealloc
    end
  )
  (func $_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17ha807bba75500b076E (;38;) (type 0) (param i32)
    (local i32)
    block ;; label = @1
      local.get 0
      i32.load
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=4
      local.get 1
      i32.const 1
      call $__rust_dealloc
    end
  )
  (func $_ZN4core3ptr77drop_in_place$LT$std..panicking..begin_panic_handler..FormatStringPayload$GT$17h70cb05dd1c565003E (;39;) (type 0) (param i32)
    (local i32)
    block ;; label = @1
      local.get 0
      i32.load
      local.tee 1
      i32.const -2147483648
      i32.or
      i32.const -2147483648
      i32.eq
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=4
      local.get 1
      i32.const 1
      call $__rust_dealloc
    end
  )
  (func $_ZN4core3ptr81drop_in_place$LT$core..result..Result$LT$$LP$$RP$$C$std..io..error..Error$GT$$GT$17h124ae7e4b5e3d03eE (;40;) (type 1) (param i32 i32)
    (local i32 i32)
    block ;; label = @1
      block ;; label = @2
        local.get 0
        i32.const 255
        i32.and
        local.tee 0
        i32.const 4
        i32.gt_u
        br_if 0 (;@2;)
        local.get 0
        i32.const 3
        i32.ne
        br_if 1 (;@1;)
      end
      local.get 1
      i32.load
      local.tee 2
      local.get 1
      i32.const 4
      i32.add
      i32.load
      local.tee 0
      i32.load
      call_indirect (type 0)
      block ;; label = @2
        local.get 0
        i32.load offset=4
        local.tee 3
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        local.get 3
        local.get 0
        i32.load offset=8
        call $__rust_dealloc
      end
      local.get 1
      i32.const 12
      i32.const 4
      call $__rust_dealloc
    end
  )
  (func $_ZN4core3ptr88drop_in_place$LT$std..io..Write..write_fmt..Adapter$LT$alloc..vec..Vec$LT$u8$GT$$GT$$GT$17hcdb72680c98ebe9dE (;41;) (type 0) (param i32)
    (local i32 i32 i32)
    local.get 0
    i32.load offset=4
    local.set 1
    block ;; label = @1
      block ;; label = @2
        local.get 0
        i32.load8_u
        local.tee 0
        i32.const 4
        i32.gt_u
        br_if 0 (;@2;)
        local.get 0
        i32.const 3
        i32.ne
        br_if 1 (;@1;)
      end
      local.get 1
      i32.load
      local.tee 2
      local.get 1
      i32.const 4
      i32.add
      i32.load
      local.tee 0
      i32.load
      call_indirect (type 0)
      block ;; label = @2
        local.get 0
        i32.load offset=4
        local.tee 3
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        local.get 3
        local.get 0
        i32.load offset=8
        call $__rust_dealloc
      end
      local.get 1
      i32.const 12
      i32.const 4
      call $__rust_dealloc
    end
  )
  (func $_ZN4core4cell4once17OnceCell$LT$T$GT$15get_or_try_init13outlined_call17he51936b7cd3d2a1dE (;42;) (type 8) (result i32)
    (local i32 i32 i32 i32 i64 i64 i64)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 0
    global.set $__stack_pointer
    local.get 0
    i32.const 8
    i32.add
    i32.const 8
    i32.const 16
    call $_ZN5alloc4sync32arcinner_layout_for_value_layout17h3fb142d6d262ede9E
    local.get 0
    i32.load offset=8
    local.set 1
    block ;; label = @1
      block ;; label = @2
        local.get 0
        i32.load offset=12
        local.tee 2
        br_if 0 (;@2;)
        local.get 1
        local.set 3
        br 1 (;@1;)
      end
      i32.const 0
      i32.load8_u offset=1055425
      drop
      local.get 2
      local.get 1
      call $__rust_alloc
      local.set 3
    end
    block ;; label = @1
      block ;; label = @2
        local.get 3
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        i64.const 4294967297
        i64.store
        local.get 3
        i32.const 16
        i32.add
        i32.const 0
        i32.store
        i32.const 0
        i64.load offset=1055496
        local.set 4
        loop ;; label = @3
          local.get 4
          i64.const 1
          i64.add
          local.tee 5
          i64.eqz
          br_if 2 (;@1;)
          i32.const 0
          local.get 5
          i32.const 0
          i64.load offset=1055496
          local.tee 6
          local.get 6
          local.get 4
          i64.eq
          local.tee 1
          select
          i64.store offset=1055496
          local.get 6
          local.set 4
          local.get 1
          i32.eqz
          br_if 0 (;@3;)
        end
        local.get 3
        local.get 5
        i64.store offset=8
        local.get 0
        i32.const 16
        i32.add
        global.set $__stack_pointer
        local.get 3
        return
      end
      local.get 1
      local.get 2
      call $_ZN5alloc5alloc18handle_alloc_error17h76ec24bcd8f509d8E
      unreachable
    end
    call $_ZN3std6thread8ThreadId3new9exhausted17hf16bddfb3b6d3400E
    unreachable
  )
  (func $_ZN3std6thread8ThreadId3new9exhausted17hf16bddfb3b6d3400E (;43;) (type 6)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 0
    global.set $__stack_pointer
    local.get 0
    i32.const 20
    i32.add
    i64.const 0
    i64.store align=4
    local.get 0
    i32.const 1
    i32.store offset=12
    local.get 0
    i32.const 1049204
    i32.store offset=8
    local.get 0
    i32.const 1048632
    i32.store offset=16
    local.get 0
    i32.const 8
    i32.add
    i32.const 1049212
    call $_ZN4core9panicking9panic_fmt17h2298343e5f8a8826E
    unreachable
  )
  (func $_ZN4core9panicking13assert_failed17h36d5dfceec7301c8E (;44;) (type 1) (param i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    i32.const 1048752
    i32.store offset=12
    local.get 2
    local.get 0
    i32.store offset=8
    i32.const 0
    local.get 2
    i32.const 8
    i32.add
    i32.const 1048756
    local.get 2
    i32.const 12
    i32.add
    i32.const 1048756
    local.get 1
    i32.const 1050756
    call $_ZN4core9panicking19assert_failed_inner17h1836e5e3a2907691E
    unreachable
  )
  (func $_ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Write$GT$10write_char17h828e0b2d8491c700E (;45;) (type 2) (param i32 i32) (result i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            local.get 1
            i32.const 128
            i32.lt_u
            br_if 0 (;@4;)
            local.get 2
            i32.const 0
            i32.store offset=12
            local.get 1
            i32.const 2048
            i32.lt_u
            br_if 1 (;@3;)
            block ;; label = @5
              local.get 1
              i32.const 65536
              i32.ge_u
              br_if 0 (;@5;)
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
              br 3 (;@2;)
            end
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
          block ;; label = @4
            local.get 0
            i32.load offset=8
            local.tee 3
            local.get 0
            i32.load
            i32.ne
            br_if 0 (;@4;)
            local.get 0
            local.get 3
            call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17h6961f3d73b538d0eE
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
      end
      block ;; label = @2
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
        call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17hb112330d3b6a189cE
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
    i32.const 0
  )
  (func $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17h6961f3d73b538d0eE (;46;) (type 1) (param i32 i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
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
        block ;; label = @3
          block ;; label = @4
            local.get 3
            br_if 0 (;@4;)
            local.get 2
            i32.const 0
            i32.store offset=24
            br 1 (;@3;)
          end
          local.get 2
          local.get 3
          i32.store offset=28
          local.get 2
          i32.const 1
          i32.store offset=24
          local.get 2
          local.get 0
          i32.load offset=4
          i32.store offset=20
        end
        local.get 2
        i32.const 8
        i32.add
        local.get 4
        local.get 1
        local.get 2
        i32.const 20
        i32.add
        call $_ZN5alloc7raw_vec11finish_grow17hb30d734150aed491E
        local.get 2
        i32.load offset=12
        local.set 3
        block ;; label = @3
          local.get 2
          i32.load offset=8
          br_if 0 (;@3;)
          local.get 0
          local.get 1
          i32.store
          local.get 0
          local.get 3
          i32.store offset=4
          br 2 (;@1;)
        end
        local.get 3
        i32.const -2147483647
        i32.eq
        br_if 1 (;@1;)
        local.get 3
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        local.get 2
        i32.const 16
        i32.add
        i32.load
        call $_ZN5alloc5alloc18handle_alloc_error17h76ec24bcd8f509d8E
        unreachable
      end
      call $_ZN5alloc7raw_vec17capacity_overflow17h4ae6daed6ccaada6E
      unreachable
    end
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
  )
  (func $_ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Write$GT$9write_str17h4f30c535d16fff94E (;47;) (type 4) (param i32 i32 i32) (result i32)
    (local i32)
    block ;; label = @1
      local.get 0
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
      call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17hb112330d3b6a189cE
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
    i32.const 0
  )
  (func $_ZN5alloc4sync16Arc$LT$T$C$A$GT$9drop_slow17hb57cee88e32714f6E (;48;) (type 0) (param i32)
    (local i32)
    block ;; label = @1
      local.get 0
      i32.load
      local.tee 0
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
    block ;; label = @1
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
    end
  )
  (func $_ZN5alloc7raw_vec11finish_grow17hb30d734150aed491E (;49;) (type 7) (param i32 i32 i32 i32)
    (local i32)
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 1
          i32.eqz
          br_if 0 (;@3;)
          local.get 2
          i32.const -1
          i32.le_s
          br_if 1 (;@2;)
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                local.get 3
                i32.load offset=4
                i32.eqz
                br_if 0 (;@6;)
                block ;; label = @7
                  local.get 3
                  i32.const 8
                  i32.add
                  i32.load
                  local.tee 4
                  br_if 0 (;@7;)
                  block ;; label = @8
                    local.get 2
                    br_if 0 (;@8;)
                    local.get 1
                    local.set 3
                    br 4 (;@4;)
                  end
                  i32.const 0
                  i32.load8_u offset=1055425
                  drop
                  br 2 (;@5;)
                end
                local.get 3
                i32.load
                local.get 4
                local.get 1
                local.get 2
                call $__rust_realloc
                local.set 3
                br 2 (;@4;)
              end
              block ;; label = @6
                local.get 2
                br_if 0 (;@6;)
                local.get 1
                local.set 3
                br 2 (;@4;)
              end
              i32.const 0
              i32.load8_u offset=1055425
              drop
            end
            local.get 2
            local.get 1
            call $__rust_alloc
            local.set 3
          end
          block ;; label = @4
            local.get 3
            i32.eqz
            br_if 0 (;@4;)
            local.get 0
            local.get 3
            i32.store offset=4
            local.get 0
            i32.const 8
            i32.add
            local.get 2
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
          br 2 (;@1;)
        end
        local.get 0
        i32.const 0
        i32.store offset=4
        local.get 0
        i32.const 8
        i32.add
        local.get 2
        i32.store
        br 1 (;@1;)
      end
      local.get 0
      i32.const 0
      i32.store offset=4
    end
    local.get 0
    i32.const 1
    i32.store
  )
  (func $_ZN60_$LT$alloc..string..String$u20$as$u20$core..fmt..Display$GT$3fmt17hf5f49f3ddec890d5E (;50;) (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load offset=4
    local.get 0
    i32.load offset=8
    local.get 1
    call $_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17hbfdf58a1d1fe1e23E
  )
  (func $_ZN64_$LT$core..str..error..Utf8Error$u20$as$u20$core..fmt..Debug$GT$3fmt17hec1ed09a2b6964b9E (;51;) (type 2) (param i32 i32) (result i32)
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
    i32.const 1048964
    i32.const 9
    i32.const 1048973
    i32.const 11
    local.get 0
    i32.const 1048948
    i32.const 1048984
    i32.const 9
    local.get 2
    i32.const 12
    i32.add
    i32.const 1048996
    call $_ZN4core3fmt9Formatter26debug_struct_field2_finish17h355b9e13692e645bE
    local.set 0
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 0
  )
  (func $_ZN3std2io5Write9write_fmt17hb2f06b1b320117c2E (;52;) (type 3) (param i32 i32 i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    i32.const 4
    i32.store8
    local.get 3
    local.get 1
    i32.store offset=8
    block ;; label = @1
      block ;; label = @2
        local.get 3
        i32.const 1049044
        local.get 2
        call $_ZN4core3fmt5write17h2d8708cc7ff40a92E
        i32.eqz
        br_if 0 (;@2;)
        block ;; label = @3
          local.get 3
          i32.load8_u
          i32.const 4
          i32.ne
          br_if 0 (;@3;)
          local.get 0
          i32.const 1050604
          i32.store offset=4
          local.get 0
          i32.const 2
          i32.store8
          br 2 (;@1;)
        end
        local.get 0
        local.get 3
        i64.load
        i64.store align=4
        br 1 (;@1;)
      end
      local.get 0
      i32.const 4
      i32.store8
      local.get 3
      i32.load offset=4
      local.set 1
      block ;; label = @2
        local.get 3
        i32.load8_u
        local.tee 0
        i32.const 4
        i32.gt_u
        br_if 0 (;@2;)
        local.get 0
        i32.const 3
        i32.ne
        br_if 1 (;@1;)
      end
      local.get 1
      i32.load
      local.tee 2
      local.get 1
      i32.const 4
      i32.add
      i32.load
      local.tee 0
      i32.load
      call_indirect (type 0)
      block ;; label = @2
        local.get 0
        i32.load offset=4
        local.tee 4
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        local.get 4
        local.get 0
        i32.load offset=8
        call $__rust_dealloc
      end
      local.get 1
      i32.const 12
      i32.const 4
      call $__rust_dealloc
    end
    local.get 3
    i32.const 16
    i32.add
    global.set $__stack_pointer
  )
  (func $_ZN3std3sys4wasi14abort_internal17hc2157ea3f05ea364E (;53;) (type 6)
    call $abort
    unreachable
  )
  (func $_ZN3std3env11current_dir17hc84cb768eaa2cd85E (;54;) (type 0) (param i32)
    (local i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    i32.const 0
    i32.load8_u offset=1055425
    drop
    i32.const 512
    local.set 2
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            i32.const 512
            i32.const 1
            call $__rust_alloc
            local.tee 3
            i32.eqz
            br_if 0 (;@4;)
            local.get 1
            local.get 3
            i32.store offset=8
            local.get 1
            i32.const 512
            i32.store offset=4
            local.get 3
            i32.const 512
            call $getcwd
            br_if 1 (;@3;)
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  i32.const 0
                  i32.load offset=1056024
                  local.tee 2
                  i32.const 68
                  i32.ne
                  br_if 0 (;@7;)
                  i32.const 512
                  local.set 2
                  br 1 (;@6;)
                end
                local.get 0
                i64.const 2147483648
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
              loop ;; label = @6
                local.get 1
                local.get 2
                i32.store offset=12
                local.get 1
                i32.const 4
                i32.add
                local.get 2
                i32.const 1
                call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17hb112330d3b6a189cE
                local.get 1
                i32.load offset=8
                local.tee 3
                local.get 1
                i32.load offset=4
                local.tee 2
                call $getcwd
                br_if 3 (;@3;)
                i32.const 0
                i32.load offset=1056024
                local.tee 4
                i32.const 68
                i32.eq
                br_if 0 (;@6;)
              end
              local.get 0
              i64.const 2147483648
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
          i32.const 1
          i32.const 512
          call $_ZN5alloc5alloc18handle_alloc_error17h76ec24bcd8f509d8E
          unreachable
        end
        local.get 1
        local.get 3
        call $strlen
        local.tee 4
        i32.store offset=12
        block ;; label = @3
          local.get 2
          local.get 4
          i32.le_u
          br_if 0 (;@3;)
          block ;; label = @4
            block ;; label = @5
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
          i32.store offset=4
          local.get 1
          local.get 5
          i32.store offset=8
        end
        local.get 0
        local.get 1
        i64.load offset=4 align=4
        i64.store align=4
        local.get 0
        i32.const 8
        i32.add
        local.get 1
        i32.const 4
        i32.add
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
    i32.const 1
    local.get 4
    call $_ZN5alloc5alloc18handle_alloc_error17h76ec24bcd8f509d8E
    unreachable
  )
  (func $_ZN3std3env7_var_os17hea1412083d7ac7a6E (;55;) (type 3) (param i32 i32 i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 416
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            local.get 2
            i32.const 383
            i32.gt_u
            br_if 0 (;@4;)
            local.get 3
            i32.const 20
            i32.add
            local.get 1
            local.get 2
            call $memcpy
            drop
            local.get 3
            i32.const 20
            i32.add
            local.get 2
            i32.add
            i32.const 0
            i32.store8
            local.get 3
            i32.const 404
            i32.add
            local.get 3
            i32.const 20
            i32.add
            local.get 2
            i32.const 1
            i32.add
            call $_ZN4core3ffi5c_str4CStr19from_bytes_with_nul17h4468beb2bd7d2757E
            block ;; label = @5
              local.get 3
              i32.load offset=404
              br_if 0 (;@5;)
              block ;; label = @6
                local.get 3
                i32.load offset=408
                call $getenv
                local.tee 1
                br_if 0 (;@6;)
                i32.const -2147483648
                local.set 2
                br 5 (;@1;)
              end
              block ;; label = @6
                block ;; label = @7
                  local.get 1
                  call $strlen
                  local.tee 2
                  br_if 0 (;@7;)
                  i32.const 1
                  local.set 4
                  br 1 (;@6;)
                end
                local.get 2
                i32.const -1
                i32.le_s
                br_if 3 (;@3;)
                i32.const 0
                i32.load8_u offset=1055425
                drop
                local.get 2
                i32.const 1
                call $__rust_alloc
                local.tee 4
                i32.eqz
                br_if 4 (;@2;)
              end
              local.get 4
              local.get 1
              local.get 2
              call $memcpy
              local.set 1
              local.get 3
              local.get 2
              i32.store offset=16
              local.get 3
              local.get 1
              i32.store offset=12
              br 4 (;@1;)
            end
            local.get 3
            i32.const 0
            i64.load offset=1050976
            i64.store offset=12 align=4
            i32.const -2147483647
            local.set 2
            br 3 (;@1;)
          end
          local.get 3
          i32.const 8
          i32.add
          local.get 1
          local.get 2
          call $_ZN3std3sys6common14small_c_string24run_with_cstr_allocating17he5b9f330c6694c39E
          local.get 3
          i32.load offset=8
          local.set 2
          br 2 (;@1;)
        end
        call $_ZN5alloc7raw_vec17capacity_overflow17h4ae6daed6ccaada6E
        unreachable
      end
      i32.const 1
      local.get 2
      call $_ZN5alloc5alloc18handle_alloc_error17h76ec24bcd8f509d8E
      unreachable
    end
    block ;; label = @1
      block ;; label = @2
        local.get 2
        i32.const -2147483647
        i32.ne
        br_if 0 (;@2;)
        block ;; label = @3
          local.get 3
          i32.load8_u offset=12
          i32.const 3
          i32.ne
          br_if 0 (;@3;)
          local.get 3
          i32.const 16
          i32.add
          i32.load
          local.tee 2
          i32.load
          local.tee 4
          local.get 2
          i32.const 4
          i32.add
          i32.load
          local.tee 1
          i32.load
          call_indirect (type 0)
          block ;; label = @4
            local.get 1
            i32.load offset=4
            local.tee 5
            i32.eqz
            br_if 0 (;@4;)
            local.get 4
            local.get 5
            local.get 1
            i32.load offset=8
            call $__rust_dealloc
          end
          local.get 2
          i32.const 12
          i32.const 4
          call $__rust_dealloc
        end
        local.get 0
        i32.const -2147483648
        i32.store
        br 1 (;@1;)
      end
      local.get 0
      local.get 3
      i64.load offset=12 align=4
      i64.store offset=4 align=4
      local.get 0
      local.get 2
      i32.store
    end
    local.get 3
    i32.const 416
    i32.add
    global.set $__stack_pointer
  )
  (func $_ZN3std3sys6common14small_c_string24run_with_cstr_allocating17he5b9f330c6694c39E (;56;) (type 3) (param i32 i32 i32)
    (local i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    local.get 1
    local.get 2
    call $_ZN72_$LT$$RF$str$u20$as$u20$alloc..ffi..c_str..CString..new..SpecNewImpl$GT$13spec_new_impl17hc178f76b63054b4aE
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            local.get 3
            i32.load
            local.tee 2
            i32.const -2147483648
            i32.ne
            br_if 0 (;@4;)
            local.get 3
            i32.const 8
            i32.add
            i32.load
            local.set 1
            block ;; label = @5
              block ;; label = @6
                local.get 3
                i32.load offset=4
                local.tee 4
                call $getenv
                local.tee 5
                br_if 0 (;@6;)
                local.get 0
                i32.const -2147483648
                i32.store
                br 1 (;@5;)
              end
              block ;; label = @6
                block ;; label = @7
                  local.get 5
                  call $strlen
                  local.tee 2
                  br_if 0 (;@7;)
                  i32.const 1
                  local.set 6
                  br 1 (;@6;)
                end
                local.get 2
                i32.const -1
                i32.le_s
                br_if 3 (;@3;)
                i32.const 0
                i32.load8_u offset=1055425
                drop
                local.get 2
                i32.const 1
                call $__rust_alloc
                local.tee 6
                i32.eqz
                br_if 4 (;@2;)
              end
              local.get 6
              local.get 5
              local.get 2
              call $memcpy
              local.set 5
              local.get 0
              local.get 2
              i32.store offset=8
              local.get 0
              local.get 5
              i32.store offset=4
              local.get 0
              local.get 2
              i32.store
            end
            local.get 4
            i32.const 0
            i32.store8
            local.get 1
            i32.eqz
            br_if 3 (;@1;)
            local.get 4
            local.get 1
            i32.const 1
            call $__rust_dealloc
            br 3 (;@1;)
          end
          local.get 0
          i32.const -2147483647
          i32.store
          local.get 0
          i32.const 0
          i64.load offset=1050976
          i64.store offset=4 align=4
          local.get 2
          i32.eqz
          br_if 2 (;@1;)
          local.get 3
          i32.load offset=4
          local.get 2
          i32.const 1
          call $__rust_dealloc
          br 2 (;@1;)
        end
        call $_ZN5alloc7raw_vec17capacity_overflow17h4ae6daed6ccaada6E
        unreachable
      end
      i32.const 1
      local.get 2
      call $_ZN5alloc5alloc18handle_alloc_error17h76ec24bcd8f509d8E
      unreachable
    end
    local.get 3
    i32.const 16
    i32.add
    global.set $__stack_pointer
  )
  (func $_ZN60_$LT$std..io..error..Error$u20$as$u20$core..fmt..Display$GT$3fmt17h6c02f8ed609e15d3E (;57;) (type 2) (param i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 64
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              local.get 0
              i32.load8_u
              br_table 0 (;@5;) 1 (;@4;) 2 (;@3;) 3 (;@2;) 0 (;@5;)
            end
            local.get 2
            local.get 0
            i32.load offset=4
            local.tee 0
            i32.store offset=8
            local.get 2
            i32.const 12
            i32.add
            local.get 0
            call $_ZN3std3sys4wasi2os12error_string17hd1d1a0a878770456E
            local.get 2
            i32.const 40
            i32.add
            i32.const 12
            i32.add
            i64.const 2
            i64.store align=4
            local.get 2
            i32.const 24
            i32.add
            i32.const 12
            i32.add
            i32.const 2
            i32.store
            local.get 2
            i32.const 3
            i32.store offset=44
            local.get 2
            i32.const 1050204
            i32.store offset=40
            local.get 2
            i32.const 3
            i32.store offset=28
            local.get 2
            local.get 2
            i32.const 24
            i32.add
            i32.store offset=48
            local.get 2
            local.get 2
            i32.const 8
            i32.add
            i32.store offset=32
            local.get 2
            local.get 2
            i32.const 12
            i32.add
            i32.store offset=24
            local.get 1
            local.get 2
            i32.const 40
            i32.add
            call $_ZN4core3fmt9Formatter9write_fmt17h4aae3abbd0cb752aE
            local.set 0
            local.get 2
            i32.load offset=12
            local.tee 1
            i32.eqz
            br_if 3 (;@1;)
            local.get 2
            i32.load offset=16
            local.get 1
            i32.const 1
            call $__rust_dealloc
            br 3 (;@1;)
          end
          local.get 0
          i32.load8_u offset=1
          local.set 0
          local.get 2
          i32.const 52
          i32.add
          i64.const 1
          i64.store align=4
          local.get 2
          i32.const 1
          i32.store offset=44
          local.get 2
          i32.const 1049368
          i32.store offset=40
          local.get 2
          i32.const 4
          i32.store offset=16
          local.get 2
          local.get 0
          i32.const 2
          i32.shl
          local.tee 0
          i32.const 1052096
          i32.add
          i32.load
          i32.store offset=28
          local.get 2
          local.get 0
          i32.const 1052260
          i32.add
          i32.load
          i32.store offset=24
          local.get 2
          local.get 2
          i32.const 12
          i32.add
          i32.store offset=48
          local.get 2
          local.get 2
          i32.const 24
          i32.add
          i32.store offset=12
          local.get 1
          local.get 2
          i32.const 40
          i32.add
          call $_ZN4core3fmt9Formatter9write_fmt17h4aae3abbd0cb752aE
          local.set 0
          br 2 (;@1;)
        end
        local.get 0
        i32.load offset=4
        local.tee 0
        i32.load
        local.get 0
        i32.load offset=4
        local.get 1
        call $_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17hbfdf58a1d1fe1e23E
        local.set 0
        br 1 (;@1;)
      end
      local.get 0
      i32.load offset=4
      local.tee 0
      i32.load
      local.get 1
      local.get 0
      i32.load offset=4
      i32.load offset=16
      call_indirect (type 2)
      local.set 0
    end
    local.get 2
    i32.const 64
    i32.add
    global.set $__stack_pointer
    local.get 0
  )
  (func $_ZN3std2io8buffered9bufwriter18BufWriter$LT$W$GT$14write_all_cold17h5afadb4f0a3ab020E (;58;) (type 7) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i64 i64)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 1
          i32.load
          local.tee 5
          local.get 1
          i32.load offset=8
          local.tee 6
          i32.sub
          local.get 3
          i32.ge_u
          br_if 0 (;@3;)
          block ;; label = @4
            local.get 6
            br_if 0 (;@4;)
            i32.const 0
            local.set 6
            br 1 (;@3;)
          end
          local.get 1
          i32.load offset=4
          local.set 7
          i32.const 0
          local.set 8
          loop ;; label = @4
            local.get 4
            local.get 6
            local.get 8
            i32.sub
            local.tee 9
            i32.store offset=4
            local.get 4
            local.get 7
            local.get 8
            i32.add
            local.tee 10
            i32.store
            local.get 4
            i32.const 8
            i32.add
            i32.const 1
            local.get 4
            i32.const 1
            call $_ZN4wasi13lib_generated8fd_write17h5d845406ea9346aaE
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    block ;; label = @9
                      local.get 4
                      i32.load16_u offset=8
                      br_if 0 (;@9;)
                      local.get 4
                      i32.load offset=12
                      local.set 11
                      br 1 (;@8;)
                    end
                    local.get 9
                    local.set 11
                    local.get 4
                    i32.load16_u offset=10
                    local.tee 12
                    i32.const 8
                    i32.eq
                    br_if 0 (;@8;)
                    local.get 1
                    i32.const 0
                    i32.store8 offset=12
                    local.get 12
                    i32.const 27
                    i32.eq
                    br_if 3 (;@5;)
                    i32.const 0
                    local.set 3
                    br 1 (;@7;)
                  end
                  local.get 1
                  i32.const 0
                  i32.store8 offset=12
                  local.get 11
                  br_if 1 (;@6;)
                  i32.const 1049356
                  local.set 12
                  i32.const 2
                  local.set 3
                end
                block ;; label = @7
                  local.get 8
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 7
                  local.get 10
                  local.get 9
                  call $memmove
                  drop
                  local.get 1
                  local.get 9
                  i32.store offset=8
                end
                local.get 0
                i32.const 0
                i32.store16 offset=1 align=1
                local.get 0
                local.get 12
                i32.store offset=4
                local.get 0
                local.get 3
                i32.store8
                local.get 0
                i32.const 3
                i32.add
                i32.const 0
                i32.store8
                br 4 (;@2;)
              end
              local.get 11
              local.get 8
              i32.add
              local.set 8
            end
            local.get 8
            local.get 6
            i32.lt_u
            br_if 0 (;@4;)
          end
          local.get 6
          local.get 8
          i32.lt_u
          br_if 2 (;@1;)
          i32.const 0
          local.set 6
          local.get 1
          i32.const 0
          i32.store offset=8
        end
        block ;; label = @3
          local.get 5
          local.get 3
          i32.le_u
          br_if 0 (;@3;)
          local.get 1
          i32.load offset=4
          local.get 6
          i32.add
          local.get 2
          local.get 3
          call $memcpy
          drop
          local.get 0
          i32.const 4
          i32.store8
          local.get 1
          local.get 6
          local.get 3
          i32.add
          i32.store offset=8
          br 1 (;@2;)
        end
        i64.const 4
        local.set 13
        block ;; label = @3
          block ;; label = @4
            local.get 3
            br_if 0 (;@4;)
            i64.const 0
            local.set 14
            br 1 (;@3;)
          end
          loop ;; label = @4
            local.get 4
            local.get 3
            i32.store offset=4
            local.get 4
            local.get 2
            i32.store
            local.get 4
            i32.const 8
            i32.add
            i32.const 1
            local.get 4
            i32.const 1
            call $_ZN4wasi13lib_generated8fd_write17h5d845406ea9346aaE
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  local.get 4
                  i32.load16_u offset=8
                  br_if 0 (;@7;)
                  block ;; label = @8
                    local.get 4
                    i32.load offset=12
                    local.tee 8
                    br_if 0 (;@8;)
                    i32.const 1050560
                    i64.extend_i32_u
                    local.set 14
                    i64.const 2
                    local.set 13
                    br 5 (;@3;)
                  end
                  local.get 3
                  local.get 8
                  i32.lt_u
                  br_if 1 (;@6;)
                  local.get 2
                  local.get 8
                  i32.add
                  local.set 2
                  local.get 3
                  local.get 8
                  i32.sub
                  local.set 3
                  br 2 (;@5;)
                end
                local.get 4
                i32.load16_u offset=10
                local.tee 8
                i32.const 27
                i32.eq
                br_if 1 (;@5;)
                i64.const 0
                local.set 14
                i64.const 4
                local.set 13
                local.get 8
                i32.const 8
                i32.eq
                br_if 3 (;@3;)
                local.get 8
                i64.extend_i32_u
                local.set 14
                i64.const 0
                local.set 13
                br 3 (;@3;)
              end
              local.get 8
              local.get 3
              i32.const 1050572
              call $_ZN4core5slice5index26slice_start_index_len_fail17h46dad5e8fa8e060bE
              unreachable
            end
            local.get 3
            br_if 0 (;@4;)
          end
          i32.const 1050560
          i64.extend_i32_u
          local.set 14
        end
        local.get 1
        i32.const 0
        i32.store8 offset=12
        local.get 0
        local.get 14
        i64.const 32
        i64.shl
        local.get 13
        i64.or
        i64.store align=4
      end
      local.get 4
      i32.const 16
      i32.add
      global.set $__stack_pointer
      return
    end
    local.get 8
    local.get 6
    i32.const 1048932
    call $_ZN4core5slice5index24slice_end_index_len_fail17hdd28afa5458bc752E
    unreachable
  )
  (func $_ZN3std3sys4wasi2os12error_string17hd1d1a0a878770456E (;59;) (type 1) (param i32 i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 1056
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
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
          call $strlen
          call $_ZN4core3str8converts9from_utf817h2f7de8ce7fc1ff13E
          block ;; label = @4
            local.get 2
            i32.load offset=1024
            br_if 0 (;@4;)
            local.get 2
            i32.load offset=1028
            local.set 3
            block ;; label = @5
              block ;; label = @6
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
              i32.le_s
              br_if 3 (;@2;)
              i32.const 0
              i32.load8_u offset=1055425
              drop
              local.get 1
              i32.const 1
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
          i32.const 1049242
          i32.const 43
          local.get 2
          i32.const 1048
          i32.add
          i32.const 1051868
          i32.const 1051916
          call $_ZN4core6result13unwrap_failed17h3354cd59a5a00fe5E
          unreachable
        end
        local.get 2
        i32.const 1036
        i32.add
        i64.const 0
        i64.store align=4
        local.get 2
        i32.const 1
        i32.store offset=1028
        local.get 2
        i32.const 1051952
        i32.store offset=1024
        local.get 2
        i32.const 1048632
        i32.store offset=1032
        local.get 2
        i32.const 1024
        i32.add
        i32.const 1051960
        call $_ZN4core9panicking9panic_fmt17h2298343e5f8a8826E
        unreachable
      end
      call $_ZN5alloc7raw_vec17capacity_overflow17h4ae6daed6ccaada6E
      unreachable
    end
    i32.const 1
    local.get 1
    call $_ZN5alloc5alloc18handle_alloc_error17h76ec24bcd8f509d8E
    unreachable
  )
  (func $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$5write17h833ff2b5e1425b8eE (;60;) (type 7) (param i32 i32 i32 i32)
    (local i32)
    block ;; label = @1
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
      call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17hb112330d3b6a189cE
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
    i32.store8
  )
  (func $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$14write_vectored17hb19441c5f2107b2aE (;61;) (type 7) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32)
    block ;; label = @1
      block ;; label = @2
        local.get 3
        br_if 0 (;@2;)
        i32.const 0
        local.set 4
        br 1 (;@1;)
      end
      local.get 3
      i32.const 3
      i32.and
      local.set 5
      block ;; label = @2
        block ;; label = @3
          local.get 3
          i32.const 4
          i32.ge_u
          br_if 0 (;@3;)
          i32.const 0
          local.set 4
          i32.const 0
          local.set 6
          br 1 (;@2;)
        end
        local.get 2
        i32.const 28
        i32.add
        local.set 7
        local.get 3
        i32.const -4
        i32.and
        local.set 8
        i32.const 0
        local.set 4
        i32.const 0
        local.set 6
        loop ;; label = @3
          local.get 7
          i32.load
          local.get 7
          i32.const -8
          i32.add
          i32.load
          local.get 7
          i32.const -16
          i32.add
          i32.load
          local.get 7
          i32.const -24
          i32.add
          i32.load
          local.get 4
          i32.add
          i32.add
          i32.add
          i32.add
          local.set 4
          local.get 7
          i32.const 32
          i32.add
          local.set 7
          local.get 8
          local.get 6
          i32.const 4
          i32.add
          local.tee 6
          i32.ne
          br_if 0 (;@3;)
        end
      end
      block ;; label = @2
        local.get 5
        i32.eqz
        br_if 0 (;@2;)
        local.get 6
        i32.const 3
        i32.shl
        local.get 2
        i32.add
        i32.const 4
        i32.add
        local.set 7
        loop ;; label = @3
          local.get 7
          i32.load
          local.get 4
          i32.add
          local.set 4
          local.get 7
          i32.const 8
          i32.add
          local.set 7
          local.get 5
          i32.const -1
          i32.add
          local.tee 5
          br_if 0 (;@3;)
        end
      end
      block ;; label = @2
        local.get 1
        i32.load
        local.get 1
        i32.load offset=8
        local.tee 7
        i32.sub
        local.get 4
        i32.ge_u
        br_if 0 (;@2;)
        local.get 1
        local.get 7
        local.get 4
        call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17hb112330d3b6a189cE
      end
      local.get 3
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      local.get 3
      i32.const 3
      i32.shl
      i32.add
      local.set 8
      local.get 1
      i32.load offset=8
      local.set 7
      loop ;; label = @2
        local.get 2
        i32.load
        local.set 6
        block ;; label = @3
          local.get 1
          i32.load
          local.get 7
          i32.sub
          local.get 2
          i32.const 4
          i32.add
          i32.load
          local.tee 5
          i32.ge_u
          br_if 0 (;@3;)
          local.get 1
          local.get 7
          local.get 5
          call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17hb112330d3b6a189cE
          local.get 1
          i32.load offset=8
          local.set 7
        end
        local.get 1
        i32.load offset=4
        local.get 7
        i32.add
        local.get 6
        local.get 5
        call $memcpy
        drop
        local.get 1
        local.get 7
        local.get 5
        i32.add
        local.tee 7
        i32.store offset=8
        local.get 2
        i32.const 8
        i32.add
        local.tee 2
        local.get 8
        i32.ne
        br_if 0 (;@2;)
      end
    end
    local.get 0
    i32.const 4
    i32.store8
    local.get 0
    local.get 4
    i32.store offset=4
  )
  (func $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$17is_write_vectored17h937f916ed6b57ed4E (;62;) (type 9) (param i32) (result i32)
    i32.const 1
  )
  (func $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$9write_all17h2e325221fa2d7c27E (;63;) (type 7) (param i32 i32 i32 i32)
    (local i32)
    block ;; label = @1
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
      call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17hb112330d3b6a189cE
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
    i32.store offset=8
  )
  (func $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$5flush17hb1cbd1fa24903956E (;64;) (type 1) (param i32 i32)
    local.get 0
    i32.const 4
    i32.store8
  )
  (func $_ZN3std2io5Write18write_all_vectored17hb7d4ca586d7e3005E (;65;) (type 7) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              local.get 3
              i32.eqz
              br_if 0 (;@5;)
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
              i32.const 0
              local.set 9
              block ;; label = @6
                loop ;; label = @7
                  local.get 9
                  local.get 5
                  i32.load
                  local.tee 10
                  i32.lt_u
                  br_if 1 (;@6;)
                  local.get 5
                  i32.const 8
                  i32.add
                  local.set 5
                  local.get 8
                  i32.const 1
                  i32.add
                  local.set 8
                  local.get 9
                  local.get 10
                  i32.sub
                  local.set 9
                  local.get 6
                  i32.const -8
                  i32.add
                  local.tee 6
                  br_if 0 (;@7;)
                end
                local.get 7
                local.set 8
              end
              local.get 3
              local.get 8
              i32.lt_u
              br_if 3 (;@2;)
              local.get 2
              local.get 8
              i32.const 3
              i32.shl
              local.tee 5
              i32.add
              local.set 7
              block ;; label = @6
                block ;; label = @7
                  local.get 3
                  local.get 8
                  i32.ne
                  br_if 0 (;@7;)
                  local.get 9
                  i32.eqz
                  br_if 1 (;@6;)
                  local.get 4
                  i32.const 20
                  i32.add
                  i64.const 0
                  i64.store align=4
                  local.get 4
                  i32.const 1
                  i32.store offset=12
                  local.get 4
                  i32.const 1050416
                  i32.store offset=8
                  local.get 4
                  i32.const 1048632
                  i32.store offset=16
                  local.get 4
                  i32.const 8
                  i32.add
                  i32.const 1050424
                  call $_ZN4core9panicking9panic_fmt17h2298343e5f8a8826E
                  unreachable
                end
                local.get 2
                local.get 5
                i32.add
                local.tee 5
                i32.load offset=4
                local.tee 6
                local.get 9
                i32.lt_u
                br_if 3 (;@3;)
                local.get 5
                i32.const 4
                i32.add
                local.get 6
                local.get 9
                i32.sub
                i32.store
                local.get 7
                local.get 7
                i32.load
                local.get 9
                i32.add
                i32.store
              end
              local.get 3
              local.get 8
              i32.sub
              local.tee 3
              i32.eqz
              br_if 0 (;@5;)
              loop ;; label = @6
                local.get 4
                i32.const 8
                i32.add
                i32.const 2
                local.get 7
                local.get 3
                call $_ZN4wasi13lib_generated8fd_write17h5d845406ea9346aaE
                block ;; label = @7
                  block ;; label = @8
                    block ;; label = @9
                      local.get 4
                      i32.load16_u offset=8
                      br_if 0 (;@9;)
                      block ;; label = @10
                        local.get 4
                        i32.load offset=12
                        local.tee 9
                        br_if 0 (;@10;)
                        local.get 0
                        i32.const 1050560
                        i32.store offset=4
                        local.get 0
                        i32.const 2
                        i32.store8
                        br 9 (;@1;)
                      end
                      local.get 7
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
                      local.set 2
                      i32.const 0
                      local.set 8
                      loop ;; label = @10
                        local.get 9
                        local.get 5
                        i32.load
                        local.tee 10
                        i32.lt_u
                        br_if 2 (;@8;)
                        local.get 5
                        i32.const 8
                        i32.add
                        local.set 5
                        local.get 8
                        i32.const 1
                        i32.add
                        local.set 8
                        local.get 9
                        local.get 10
                        i32.sub
                        local.set 9
                        local.get 6
                        i32.const -8
                        i32.add
                        local.tee 6
                        br_if 0 (;@10;)
                      end
                      local.get 2
                      local.set 8
                      br 1 (;@8;)
                    end
                    local.get 4
                    i32.load16_u offset=10
                    local.tee 9
                    i32.const 27
                    i32.eq
                    br_if 1 (;@7;)
                    local.get 0
                    local.get 9
                    i32.store offset=4
                    local.get 0
                    i32.const 0
                    i32.store
                    br 7 (;@1;)
                  end
                  local.get 3
                  local.get 8
                  i32.lt_u
                  br_if 3 (;@4;)
                  local.get 3
                  local.get 8
                  i32.sub
                  local.set 6
                  local.get 7
                  local.get 8
                  i32.const 3
                  i32.shl
                  local.tee 10
                  i32.add
                  local.set 5
                  block ;; label = @8
                    local.get 3
                    local.get 8
                    i32.ne
                    br_if 0 (;@8;)
                    local.get 5
                    local.set 7
                    local.get 6
                    local.set 3
                    local.get 9
                    i32.eqz
                    br_if 1 (;@7;)
                    local.get 4
                    i32.const 20
                    i32.add
                    i64.const 0
                    i64.store align=4
                    local.get 4
                    i32.const 1
                    i32.store offset=12
                    local.get 4
                    i32.const 1050416
                    i32.store offset=8
                    local.get 4
                    i32.const 1048632
                    i32.store offset=16
                    local.get 4
                    i32.const 8
                    i32.add
                    i32.const 1050424
                    call $_ZN4core9panicking9panic_fmt17h2298343e5f8a8826E
                    unreachable
                  end
                  block ;; label = @8
                    local.get 7
                    local.get 10
                    i32.add
                    local.tee 8
                    i32.load offset=4
                    local.tee 10
                    local.get 9
                    i32.lt_u
                    br_if 0 (;@8;)
                    local.get 8
                    i32.const 4
                    i32.add
                    local.get 10
                    local.get 9
                    i32.sub
                    i32.store
                    local.get 5
                    local.get 5
                    i32.load
                    local.get 9
                    i32.add
                    i32.store
                    local.get 5
                    local.set 7
                    local.get 6
                    local.set 3
                    br 1 (;@7;)
                  end
                  local.get 4
                  i32.const 20
                  i32.add
                  i64.const 0
                  i64.store align=4
                  local.get 4
                  i32.const 1
                  i32.store offset=12
                  local.get 4
                  i32.const 1050476
                  i32.store offset=8
                  local.get 4
                  i32.const 1048632
                  i32.store offset=16
                  local.get 4
                  i32.const 8
                  i32.add
                  i32.const 1050516
                  call $_ZN4core9panicking9panic_fmt17h2298343e5f8a8826E
                  unreachable
                end
                local.get 3
                br_if 0 (;@6;)
              end
            end
            local.get 0
            i32.const 4
            i32.store8
            br 3 (;@1;)
          end
          local.get 8
          local.get 3
          i32.const 1050360
          call $_ZN4core5slice5index26slice_start_index_len_fail17h46dad5e8fa8e060bE
          unreachable
        end
        local.get 4
        i32.const 20
        i32.add
        i64.const 0
        i64.store align=4
        local.get 4
        i32.const 1
        i32.store offset=12
        local.get 4
        i32.const 1050476
        i32.store offset=8
        local.get 4
        i32.const 1048632
        i32.store offset=16
        local.get 4
        i32.const 8
        i32.add
        i32.const 1050516
        call $_ZN4core9panicking9panic_fmt17h2298343e5f8a8826E
        unreachable
      end
      local.get 8
      local.get 3
      i32.const 1050360
      call $_ZN4core5slice5index26slice_start_index_len_fail17h46dad5e8fa8e060bE
      unreachable
    end
    local.get 4
    i32.const 32
    i32.add
    global.set $__stack_pointer
  )
  (func $_ZN3std4sync9once_lock17OnceLock$LT$T$GT$10initialize17hdd0e7addc178a8ebE (;66;) (type 6)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 0
    global.set $__stack_pointer
    block ;; label = @1
      i32.const 0
      i32.load8_u offset=1055460
      i32.const 3
      i32.eq
      br_if 0 (;@1;)
      local.get 0
      i32.const 1055428
      i32.store offset=4
      local.get 0
      local.get 0
      i32.const 15
      i32.add
      i32.store offset=8
      local.get 0
      i32.const 4
      i32.add
      call $_ZN3std3sys4wasi4once4Once4call17h59c9a6ba0f648d2cE
    end
    local.get 0
    i32.const 16
    i32.add
    global.set $__stack_pointer
  )
  (func $_ZN61_$LT$$RF$std..io..stdio..Stdout$u20$as$u20$std..io..Write$GT$9write_fmt17hcc7ecae8d634ba62E (;67;) (type 3) (param i32 i32 i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            local.get 1
            i32.load
            i32.load
            local.tee 1
            i32.load
            i32.const 1055516
            i32.eq
            br_if 0 (;@4;)
            local.get 1
            i32.load8_u offset=28
            local.set 4
            local.get 1
            i32.const 1
            i32.store8 offset=28
            local.get 3
            local.get 4
            i32.store8 offset=4
            local.get 4
            i32.eqz
            br_if 1 (;@3;)
            local.get 3
            i64.const 0
            i64.store offset=20 align=4
            local.get 3
            i32.const 1048632
            i32.store offset=16
            local.get 3
            i32.const 1
            i32.store offset=12
            local.get 3
            i32.const 1050692
            i32.store offset=8
            local.get 3
            i32.const 4
            i32.add
            local.get 3
            i32.const 8
            i32.add
            call $_ZN4core9panicking13assert_failed17h36d5dfceec7301c8E
            unreachable
          end
          local.get 1
          i32.load offset=4
          i32.const 1
          i32.add
          local.tee 4
          i32.eqz
          br_if 2 (;@1;)
          local.get 1
          local.get 4
          i32.store offset=4
          br 1 (;@2;)
        end
        local.get 1
        i32.const 1
        i32.store offset=4
        local.get 1
        i32.const 1055516
        i32.store
      end
      local.get 3
      local.get 1
      i32.store offset=4
      local.get 3
      i32.const 4
      i32.store8 offset=8
      local.get 3
      local.get 3
      i32.const 4
      i32.add
      i32.store offset=16
      block ;; label = @2
        block ;; label = @3
          local.get 3
          i32.const 8
          i32.add
          i32.const 1049092
          local.get 2
          call $_ZN4core3fmt5write17h2d8708cc7ff40a92E
          i32.eqz
          br_if 0 (;@3;)
          block ;; label = @4
            local.get 3
            i32.load8_u offset=8
            i32.const 4
            i32.ne
            br_if 0 (;@4;)
            local.get 0
            i32.const 1050604
            i32.store offset=4
            local.get 0
            i32.const 2
            i32.store8
            br 2 (;@2;)
          end
          local.get 0
          local.get 3
          i64.load offset=8
          i64.store align=4
          br 1 (;@2;)
        end
        local.get 0
        i32.const 4
        i32.store8
        local.get 3
        i32.load offset=12
        local.set 0
        block ;; label = @3
          local.get 3
          i32.load8_u offset=8
          local.tee 1
          i32.const 4
          i32.gt_u
          br_if 0 (;@3;)
          local.get 1
          i32.const 3
          i32.ne
          br_if 1 (;@2;)
        end
        local.get 0
        i32.load
        local.tee 2
        local.get 0
        i32.const 4
        i32.add
        i32.load
        local.tee 1
        i32.load
        call_indirect (type 0)
        block ;; label = @3
          local.get 1
          i32.load offset=4
          local.tee 4
          i32.eqz
          br_if 0 (;@3;)
          local.get 2
          local.get 4
          local.get 1
          i32.load offset=8
          call $__rust_dealloc
        end
        local.get 0
        i32.const 12
        i32.const 4
        call $__rust_dealloc
      end
      local.get 3
      i32.load offset=4
      local.tee 1
      local.get 1
      i32.load offset=4
      i32.const -1
      i32.add
      local.tee 0
      i32.store offset=4
      block ;; label = @2
        local.get 0
        br_if 0 (;@2;)
        local.get 1
        i32.const 0
        i32.store8 offset=28
        local.get 1
        i32.const 0
        i32.store
      end
      local.get 3
      i32.const 32
      i32.add
      global.set $__stack_pointer
      return
    end
    i32.const 1050832
    i32.const 38
    i32.const 1050904
    call $_ZN4core6option13expect_failed17hed265c0c5be592b4E
    unreachable
  )
  (func $_ZN3std2io5stdio31print_to_buffer_if_capture_used17h7afb81e085932b05E (;68;) (type 9) (param i32) (result i32)
    (local i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    i32.const 0
    local.set 2
    block ;; label = @1
      block ;; label = @2
        i32.const 0
        i32.load8_u offset=1055426
        i32.eqz
        br_if 0 (;@2;)
        i32.const 0
        local.set 2
        block ;; label = @3
          i32.const 0
          i32.load8_u offset=1055520
          br_if 0 (;@3;)
          i32.const 0
          i32.const 1
          i32.store8 offset=1055520
          i32.const 0
          i32.const 0
          i32.store offset=1055524
          br 1 (;@2;)
        end
        i32.const 0
        local.set 2
        i32.const 0
        i32.load offset=1055524
        local.set 3
        i32.const 0
        i32.const 0
        i32.store offset=1055524
        local.get 3
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        i32.load8_u offset=8
        local.set 2
        i32.const 1
        local.set 4
        local.get 3
        i32.const 1
        i32.store8 offset=8
        local.get 1
        local.get 2
        i32.store8 offset=7
        local.get 2
        br_if 1 (;@1;)
        block ;; label = @3
          i32.const 0
          i32.load offset=1055492
          i32.const 2147483647
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          call $_ZN3std9panicking11panic_count17is_zero_slow_path17hb2a28c6a6d133550E
          local.set 4
        end
        local.get 1
        i32.const 4
        i32.store8 offset=8
        local.get 1
        local.get 3
        i32.const 12
        i32.add
        i32.store offset=16
        local.get 1
        i32.const 8
        i32.add
        i32.const 1049068
        local.get 0
        call $_ZN4core3fmt5write17h2d8708cc7ff40a92E
        local.set 0
        local.get 1
        i32.load8_u offset=8
        local.set 2
        block ;; label = @3
          block ;; label = @4
            local.get 0
            i32.eqz
            br_if 0 (;@4;)
            local.get 2
            i32.const 4
            i32.eq
            br_if 1 (;@3;)
            local.get 1
            i32.load offset=12
            local.set 0
            block ;; label = @5
              local.get 1
              i32.load8_u offset=8
              local.tee 2
              i32.const 4
              i32.gt_u
              br_if 0 (;@5;)
              local.get 2
              i32.const 3
              i32.ne
              br_if 2 (;@3;)
            end
            local.get 0
            i32.load
            local.tee 5
            local.get 0
            i32.const 4
            i32.add
            i32.load
            local.tee 2
            i32.load
            call_indirect (type 0)
            block ;; label = @5
              local.get 2
              i32.load offset=4
              local.tee 6
              i32.eqz
              br_if 0 (;@5;)
              local.get 5
              local.get 6
              local.get 2
              i32.load offset=8
              call $__rust_dealloc
            end
            local.get 0
            i32.const 12
            i32.const 4
            call $__rust_dealloc
            br 1 (;@3;)
          end
          local.get 1
          i32.load offset=12
          local.set 0
          block ;; label = @4
            local.get 2
            i32.const 4
            i32.gt_u
            br_if 0 (;@4;)
            local.get 2
            i32.const 3
            i32.ne
            br_if 1 (;@3;)
          end
          local.get 0
          i32.load
          local.tee 5
          local.get 0
          i32.const 4
          i32.add
          i32.load
          local.tee 2
          i32.load
          call_indirect (type 0)
          block ;; label = @4
            local.get 2
            i32.load offset=4
            local.tee 6
            i32.eqz
            br_if 0 (;@4;)
            local.get 5
            local.get 6
            local.get 2
            i32.load offset=8
            call $__rust_dealloc
          end
          local.get 0
          i32.const 12
          i32.const 4
          call $__rust_dealloc
        end
        block ;; label = @3
          local.get 4
          i32.eqz
          br_if 0 (;@3;)
          i32.const 0
          i32.load offset=1055492
          i32.const 2147483647
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          call $_ZN3std9panicking11panic_count17is_zero_slow_path17hb2a28c6a6d133550E
          br_if 0 (;@3;)
          local.get 3
          i32.const 1
          i32.store8 offset=9
        end
        local.get 3
        i32.const 0
        i32.store8 offset=8
        i32.const 0
        i32.load offset=1055524
        local.set 2
        i32.const 0
        local.get 3
        i32.store offset=1055524
        local.get 1
        local.get 2
        i32.store offset=8
        block ;; label = @3
          local.get 2
          i32.eqz
          br_if 0 (;@3;)
          local.get 2
          local.get 2
          i32.load
          local.tee 3
          i32.const -1
          i32.add
          i32.store
          local.get 3
          i32.const 1
          i32.ne
          br_if 0 (;@3;)
          local.get 1
          i32.const 8
          i32.add
          call $_ZN5alloc4sync16Arc$LT$T$C$A$GT$9drop_slow17hb57cee88e32714f6E
        end
        i32.const 1
        local.set 2
      end
      local.get 1
      i32.const 32
      i32.add
      global.set $__stack_pointer
      local.get 2
      return
    end
    local.get 1
    i64.const 0
    i64.store offset=20 align=4
    local.get 1
    i32.const 1048632
    i32.store offset=16
    local.get 1
    i32.const 1
    i32.store offset=12
    local.get 1
    i32.const 1050692
    i32.store offset=8
    local.get 1
    i32.const 7
    i32.add
    local.get 1
    i32.const 8
    i32.add
    call $_ZN4core9panicking13assert_failed17h36d5dfceec7301c8E
    unreachable
  )
  (func $_ZN3std2io5stdio6_print17hda456e4879685b1fE (;69;) (type 0) (param i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 80
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    local.get 1
    i32.const 6
    i32.store offset=12
    local.get 1
    i32.const 1050328
    i32.store offset=8
    block ;; label = @1
      block ;; label = @2
        local.get 0
        call $_ZN3std2io5stdio31print_to_buffer_if_capture_used17h7afb81e085932b05E
        br_if 0 (;@2;)
        block ;; label = @3
          i32.const 0
          i32.load8_u offset=1055460
          i32.const 3
          i32.eq
          br_if 0 (;@3;)
          call $_ZN3std4sync9once_lock17OnceLock$LT$T$GT$10initialize17hdd0e7addc178a8ebE
        end
        local.get 1
        i32.const 1055428
        i32.store offset=28
        local.get 1
        local.get 1
        i32.const 28
        i32.add
        i32.store offset=40
        local.get 1
        i32.const 16
        i32.add
        local.get 1
        i32.const 40
        i32.add
        local.get 0
        call $_ZN61_$LT$$RF$std..io..stdio..Stdout$u20$as$u20$std..io..Write$GT$9write_fmt17hcc7ecae8d634ba62E
        local.get 1
        i32.load8_u offset=16
        i32.const 4
        i32.ne
        br_if 1 (;@1;)
      end
      local.get 1
      i32.const 80
      i32.add
      global.set $__stack_pointer
      return
    end
    local.get 1
    local.get 1
    i64.load offset=16
    i64.store offset=32
    local.get 1
    i32.const 40
    i32.add
    i32.const 12
    i32.add
    i64.const 2
    i64.store align=4
    local.get 1
    i32.const 64
    i32.add
    i32.const 12
    i32.add
    i32.const 5
    i32.store
    local.get 1
    i32.const 2
    i32.store offset=44
    local.get 1
    i32.const 1050296
    i32.store offset=40
    local.get 1
    i32.const 4
    i32.store offset=68
    local.get 1
    local.get 1
    i32.const 64
    i32.add
    i32.store offset=48
    local.get 1
    local.get 1
    i32.const 32
    i32.add
    i32.store offset=72
    local.get 1
    local.get 1
    i32.const 8
    i32.add
    i32.store offset=64
    local.get 1
    i32.const 40
    i32.add
    i32.const 1050312
    call $_ZN4core9panicking9panic_fmt17h2298343e5f8a8826E
    unreachable
  )
  (func $_ZN3std2io5Write9write_all17h52b2f76405bd61e5E (;70;) (type 7) (param i32 i32 i32 i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            local.get 3
            i32.eqz
            br_if 0 (;@4;)
            loop ;; label = @5
              local.get 4
              local.get 3
              i32.store offset=4
              local.get 4
              local.get 2
              i32.store
              local.get 4
              i32.const 8
              i32.add
              i32.const 2
              local.get 4
              i32.const 1
              call $_ZN4wasi13lib_generated8fd_write17h5d845406ea9346aaE
              block ;; label = @6
                block ;; label = @7
                  local.get 4
                  i32.load16_u offset=8
                  br_if 0 (;@7;)
                  block ;; label = @8
                    local.get 4
                    i32.load offset=12
                    local.tee 5
                    br_if 0 (;@8;)
                    local.get 0
                    i32.const 1050560
                    i32.store offset=4
                    local.get 0
                    i32.const 2
                    i32.store8
                    br 7 (;@1;)
                  end
                  local.get 3
                  local.get 5
                  i32.lt_u
                  br_if 4 (;@3;)
                  local.get 2
                  local.get 5
                  i32.add
                  local.set 2
                  local.get 3
                  local.get 5
                  i32.sub
                  local.set 3
                  br 1 (;@6;)
                end
                local.get 4
                i32.load16_u offset=10
                local.tee 5
                i32.const 27
                i32.ne
                br_if 4 (;@2;)
              end
              local.get 3
              br_if 0 (;@5;)
            end
          end
          local.get 0
          i32.const 4
          i32.store8
          br 2 (;@1;)
        end
        local.get 5
        local.get 3
        i32.const 1050572
        call $_ZN4core5slice5index26slice_start_index_len_fail17h46dad5e8fa8e060bE
        unreachable
      end
      local.get 0
      local.get 5
      i32.store offset=4
      local.get 0
      i32.const 0
      i32.store
    end
    local.get 4
    i32.const 16
    i32.add
    global.set $__stack_pointer
  )
  (func $_ZN3std2io5Write18write_all_vectored17habcb74d67e8f3963E (;71;) (type 7) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                local.get 3
                i32.eqz
                br_if 0 (;@6;)
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
                i32.const 0
                local.set 9
                block ;; label = @7
                  loop ;; label = @8
                    local.get 9
                    local.get 5
                    i32.load
                    local.tee 10
                    i32.lt_u
                    br_if 1 (;@7;)
                    local.get 5
                    i32.const 8
                    i32.add
                    local.set 5
                    local.get 8
                    i32.const 1
                    i32.add
                    local.set 8
                    local.get 9
                    local.get 10
                    i32.sub
                    local.set 9
                    local.get 6
                    i32.const -8
                    i32.add
                    local.tee 6
                    br_if 0 (;@8;)
                  end
                  local.get 7
                  local.set 8
                end
                local.get 3
                local.get 8
                i32.lt_u
                br_if 4 (;@2;)
                local.get 2
                local.get 8
                i32.const 3
                i32.shl
                local.tee 5
                i32.add
                local.set 11
                block ;; label = @7
                  block ;; label = @8
                    local.get 3
                    local.get 8
                    i32.ne
                    br_if 0 (;@8;)
                    local.get 9
                    i32.eqz
                    br_if 1 (;@7;)
                    local.get 4
                    i32.const 20
                    i32.add
                    i64.const 0
                    i64.store align=4
                    local.get 4
                    i32.const 1
                    i32.store offset=12
                    local.get 4
                    i32.const 1050416
                    i32.store offset=8
                    local.get 4
                    i32.const 1048632
                    i32.store offset=16
                    local.get 4
                    i32.const 8
                    i32.add
                    i32.const 1050424
                    call $_ZN4core9panicking9panic_fmt17h2298343e5f8a8826E
                    unreachable
                  end
                  local.get 2
                  local.get 5
                  i32.add
                  local.tee 5
                  i32.load offset=4
                  local.tee 6
                  local.get 9
                  i32.lt_u
                  br_if 4 (;@3;)
                  local.get 5
                  i32.const 4
                  i32.add
                  local.get 6
                  local.get 9
                  i32.sub
                  i32.store
                  local.get 11
                  local.get 11
                  i32.load
                  local.get 9
                  i32.add
                  i32.store
                end
                local.get 3
                local.get 8
                i32.sub
                local.tee 7
                i32.eqz
                br_if 0 (;@6;)
                loop ;; label = @7
                  i32.const 0
                  local.set 8
                  i32.const 0
                  local.set 6
                  block ;; label = @8
                    local.get 7
                    i32.const -1
                    i32.add
                    local.tee 12
                    i32.const 3
                    i32.lt_u
                    br_if 0 (;@8;)
                    local.get 11
                    i32.const 28
                    i32.add
                    local.set 9
                    local.get 7
                    i32.const -4
                    i32.and
                    local.set 5
                    i32.const 0
                    local.set 8
                    i32.const 0
                    local.set 6
                    loop ;; label = @9
                      local.get 9
                      i32.load
                      local.get 9
                      i32.const -8
                      i32.add
                      i32.load
                      local.get 9
                      i32.const -16
                      i32.add
                      i32.load
                      local.get 9
                      i32.const -24
                      i32.add
                      i32.load
                      local.get 8
                      i32.add
                      i32.add
                      i32.add
                      i32.add
                      local.set 8
                      local.get 9
                      i32.const 32
                      i32.add
                      local.set 9
                      local.get 5
                      local.get 6
                      i32.const 4
                      i32.add
                      local.tee 6
                      i32.ne
                      br_if 0 (;@9;)
                    end
                  end
                  block ;; label = @8
                    local.get 7
                    i32.const 3
                    i32.and
                    local.tee 5
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 11
                    local.get 6
                    i32.const 3
                    i32.shl
                    i32.add
                    i32.const 4
                    i32.add
                    local.set 9
                    loop ;; label = @9
                      local.get 9
                      i32.load
                      local.get 8
                      i32.add
                      local.set 8
                      local.get 9
                      i32.const 8
                      i32.add
                      local.set 9
                      local.get 5
                      i32.const -1
                      i32.add
                      local.tee 5
                      br_if 0 (;@9;)
                    end
                  end
                  block ;; label = @8
                    local.get 1
                    i32.load
                    local.get 1
                    i32.load offset=8
                    local.tee 9
                    i32.sub
                    local.get 8
                    i32.ge_u
                    br_if 0 (;@8;)
                    local.get 1
                    local.get 9
                    local.get 8
                    call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17hb112330d3b6a189cE
                    local.get 1
                    i32.load offset=8
                    local.set 9
                  end
                  local.get 11
                  local.get 7
                  i32.const 3
                  i32.shl
                  local.tee 2
                  i32.add
                  local.set 3
                  local.get 11
                  local.set 5
                  loop ;; label = @8
                    local.get 5
                    i32.load
                    local.set 10
                    block ;; label = @9
                      local.get 1
                      i32.load
                      local.get 9
                      i32.sub
                      local.get 5
                      i32.const 4
                      i32.add
                      i32.load
                      local.tee 6
                      i32.ge_u
                      br_if 0 (;@9;)
                      local.get 1
                      local.get 9
                      local.get 6
                      call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17hb112330d3b6a189cE
                      local.get 1
                      i32.load offset=8
                      local.set 9
                    end
                    local.get 1
                    i32.load offset=4
                    local.get 9
                    i32.add
                    local.get 10
                    local.get 6
                    call $memcpy
                    drop
                    local.get 1
                    local.get 9
                    local.get 6
                    i32.add
                    local.tee 9
                    i32.store offset=8
                    local.get 5
                    i32.const 8
                    i32.add
                    local.tee 5
                    local.get 3
                    i32.ne
                    br_if 0 (;@8;)
                  end
                  block ;; label = @8
                    local.get 8
                    br_if 0 (;@8;)
                    local.get 0
                    i32.const 1050560
                    i32.store offset=4
                    local.get 0
                    i32.const 2
                    i32.store8
                    br 7 (;@1;)
                  end
                  local.get 11
                  i32.const 4
                  i32.add
                  local.set 9
                  local.get 12
                  i32.const 536870911
                  i32.and
                  i32.const 1
                  i32.add
                  local.set 10
                  i32.const 0
                  local.set 5
                  block ;; label = @8
                    loop ;; label = @9
                      local.get 8
                      local.get 9
                      i32.load
                      local.tee 6
                      i32.lt_u
                      br_if 1 (;@8;)
                      local.get 9
                      i32.const 8
                      i32.add
                      local.set 9
                      local.get 5
                      i32.const 1
                      i32.add
                      local.set 5
                      local.get 8
                      local.get 6
                      i32.sub
                      local.set 8
                      local.get 2
                      i32.const -8
                      i32.add
                      local.tee 2
                      br_if 0 (;@9;)
                    end
                    local.get 10
                    local.set 5
                  end
                  local.get 7
                  local.get 5
                  i32.lt_u
                  br_if 2 (;@5;)
                  local.get 11
                  local.get 5
                  i32.const 3
                  i32.shl
                  local.tee 6
                  i32.add
                  local.set 9
                  block ;; label = @8
                    block ;; label = @9
                      local.get 7
                      local.get 5
                      i32.ne
                      br_if 0 (;@9;)
                      local.get 8
                      i32.eqz
                      br_if 1 (;@8;)
                      local.get 4
                      i32.const 20
                      i32.add
                      i64.const 0
                      i64.store align=4
                      local.get 4
                      i32.const 1
                      i32.store offset=12
                      local.get 4
                      i32.const 1050416
                      i32.store offset=8
                      local.get 4
                      i32.const 1048632
                      i32.store offset=16
                      local.get 4
                      i32.const 8
                      i32.add
                      i32.const 1050424
                      call $_ZN4core9panicking9panic_fmt17h2298343e5f8a8826E
                      unreachable
                    end
                    local.get 11
                    local.get 6
                    i32.add
                    local.tee 6
                    i32.load offset=4
                    local.tee 10
                    local.get 8
                    i32.lt_u
                    br_if 4 (;@4;)
                    local.get 6
                    i32.const 4
                    i32.add
                    local.get 10
                    local.get 8
                    i32.sub
                    i32.store
                    local.get 9
                    local.get 9
                    i32.load
                    local.get 8
                    i32.add
                    i32.store
                  end
                  local.get 9
                  local.set 11
                  local.get 7
                  local.get 5
                  i32.sub
                  local.tee 7
                  br_if 0 (;@7;)
                end
              end
              local.get 0
              i32.const 4
              i32.store8
              br 4 (;@1;)
            end
            local.get 5
            local.get 7
            i32.const 1050360
            call $_ZN4core5slice5index26slice_start_index_len_fail17h46dad5e8fa8e060bE
            unreachable
          end
          local.get 4
          i32.const 20
          i32.add
          i64.const 0
          i64.store align=4
          local.get 4
          i32.const 1
          i32.store offset=12
          local.get 4
          i32.const 1050476
          i32.store offset=8
          local.get 4
          i32.const 1048632
          i32.store offset=16
          local.get 4
          i32.const 8
          i32.add
          i32.const 1050516
          call $_ZN4core9panicking9panic_fmt17h2298343e5f8a8826E
          unreachable
        end
        local.get 4
        i32.const 20
        i32.add
        i64.const 0
        i64.store align=4
        local.get 4
        i32.const 1
        i32.store offset=12
        local.get 4
        i32.const 1050476
        i32.store offset=8
        local.get 4
        i32.const 1048632
        i32.store offset=16
        local.get 4
        i32.const 8
        i32.add
        i32.const 1050516
        call $_ZN4core9panicking9panic_fmt17h2298343e5f8a8826E
        unreachable
      end
      local.get 8
      local.get 3
      i32.const 1050360
      call $_ZN4core5slice5index26slice_start_index_len_fail17h46dad5e8fa8e060bE
      unreachable
    end
    local.get 4
    i32.const 32
    i32.add
    global.set $__stack_pointer
  )
  (func $_ZN3std2io5Write9write_fmt17hf5db5a8e08f12dacE (;72;) (type 3) (param i32 i32 i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    i32.const 4
    i32.store8
    local.get 3
    local.get 1
    i32.store offset=8
    block ;; label = @1
      block ;; label = @2
        local.get 3
        i32.const 1049068
        local.get 2
        call $_ZN4core3fmt5write17h2d8708cc7ff40a92E
        i32.eqz
        br_if 0 (;@2;)
        block ;; label = @3
          local.get 3
          i32.load8_u
          i32.const 4
          i32.ne
          br_if 0 (;@3;)
          local.get 0
          i32.const 1050604
          i32.store offset=4
          local.get 0
          i32.const 2
          i32.store8
          br 2 (;@1;)
        end
        local.get 0
        local.get 3
        i64.load
        i64.store align=4
        br 1 (;@1;)
      end
      local.get 0
      i32.const 4
      i32.store8
      local.get 3
      i32.load offset=4
      local.set 1
      block ;; label = @2
        local.get 3
        i32.load8_u
        local.tee 0
        i32.const 4
        i32.gt_u
        br_if 0 (;@2;)
        local.get 0
        i32.const 3
        i32.ne
        br_if 1 (;@1;)
      end
      local.get 1
      i32.load
      local.tee 2
      local.get 1
      i32.const 4
      i32.add
      i32.load
      local.tee 0
      i32.load
      call_indirect (type 0)
      block ;; label = @2
        local.get 0
        i32.load offset=4
        local.tee 4
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        local.get 4
        local.get 0
        i32.load offset=8
        call $__rust_dealloc
      end
      local.get 1
      i32.const 12
      i32.const 4
      call $__rust_dealloc
    end
    local.get 3
    i32.const 16
    i32.add
    global.set $__stack_pointer
  )
  (func $_ZN80_$LT$std..io..Write..write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17h6768fb4712d63ad7E (;73;) (type 4) (param i32 i32 i32) (result i32)
    (local i32)
    block ;; label = @1
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
      call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17hb112330d3b6a189cE
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
    i32.const 0
  )
  (func $_ZN80_$LT$std..io..Write..write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17hc6c1bc1e398c2f1bE (;74;) (type 4) (param i32 i32 i32) (result i32)
    (local i32 i32 i64 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    i32.const 8
    i32.add
    local.get 0
    i32.load offset=8
    local.get 1
    local.get 2
    call $_ZN61_$LT$std..io..stdio..StdoutLock$u20$as$u20$std..io..Write$GT$9write_all17h0816e60e5bb42170E
    block ;; label = @1
      local.get 3
      i32.load8_u offset=8
      local.tee 2
      i32.const 4
      i32.eq
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=4
      local.set 4
      local.get 3
      i64.load offset=8
      local.set 5
      block ;; label = @2
        block ;; label = @3
          local.get 0
          i32.load8_u
          local.tee 1
          i32.const 4
          i32.gt_u
          br_if 0 (;@3;)
          local.get 1
          i32.const 3
          i32.ne
          br_if 1 (;@2;)
        end
        local.get 4
        i32.load
        local.tee 6
        local.get 4
        i32.const 4
        i32.add
        i32.load
        local.tee 1
        i32.load
        call_indirect (type 0)
        block ;; label = @3
          local.get 1
          i32.load offset=4
          local.tee 7
          i32.eqz
          br_if 0 (;@3;)
          local.get 6
          local.get 7
          local.get 1
          i32.load offset=8
          call $__rust_dealloc
        end
        local.get 4
        i32.const 12
        i32.const 4
        call $__rust_dealloc
      end
      local.get 0
      local.get 5
      i64.store align=4
    end
    local.get 3
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 2
    i32.const 4
    i32.ne
  )
  (func $_ZN3std5panic19get_backtrace_style17he6d3ee8cbcc9f914E (;75;) (type 8) (result i32)
    (local i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 0
    global.set $__stack_pointer
    i32.const 0
    local.set 1
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              i32.const 0
              i32.load offset=1055464
              br_table 3 (;@2;) 4 (;@1;) 2 (;@3;) 1 (;@4;) 0 (;@5;)
            end
            i32.const 1048815
            i32.const 40
            i32.const 1050640
            call $_ZN4core9panicking5panic17h89e93701a34d6dc7E
            unreachable
          end
          i32.const 2
          local.set 1
          br 2 (;@1;)
        end
        i32.const 1
        local.set 1
        br 1 (;@1;)
      end
      local.get 0
      i32.const 4
      i32.add
      i32.const 1049228
      i32.const 14
      call $_ZN3std3env7_var_os17hea1412083d7ac7a6E
      block ;; label = @2
        block ;; label = @3
          local.get 0
          i32.load offset=4
          local.tee 2
          i32.const -2147483648
          i32.eq
          br_if 0 (;@3;)
          i32.const 0
          local.set 1
          local.get 0
          i32.load offset=8
          local.set 3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                local.get 0
                i32.load offset=12
                i32.const -1
                i32.add
                br_table 0 (;@6;) 2 (;@4;) 2 (;@4;) 1 (;@5;) 2 (;@4;)
              end
              local.get 3
              i32.load8_u
              i32.const 48
              i32.eq
              i32.const 1
              i32.shl
              local.set 1
              br 1 (;@4;)
            end
            local.get 3
            i32.const 1050656
            i32.const 4
            call $memcmp
            i32.eqz
            local.set 1
          end
          block ;; label = @4
            local.get 2
            i32.eqz
            br_if 0 (;@4;)
            local.get 3
            local.get 2
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
      i32.store offset=1055464
    end
    local.get 0
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 1
  )
  (func $_ZN3std7process5abort17h8ff01d99bdc8af7eE (;76;) (type 6)
    call $_ZN3std3sys4wasi14abort_internal17hc2157ea3f05ea364E
    unreachable
  )
  (func $_ZN3std3sys4wasi4once4Once4call17h59c9a6ba0f648d2cE (;77;) (type 0) (param i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              i32.const 0
              i32.load8_u offset=1055460
              local.tee 2
              i32.const 2
              i32.lt_u
              br_if 0 (;@5;)
              local.get 2
              i32.const -2
              i32.add
              br_table 4 (;@1;) 1 (;@4;) 4 (;@1;)
            end
            i32.const 0
            i32.const 2
            i32.store8 offset=1055460
            local.get 0
            i32.load
            local.set 2
            local.get 0
            i32.const 0
            i32.store
            local.get 2
            i32.eqz
            br_if 1 (;@3;)
            i32.const 0
            i32.load8_u offset=1055425
            drop
            i32.const 1024
            i32.const 1
            call $__rust_alloc
            local.tee 0
            i32.eqz
            br_if 2 (;@2;)
            local.get 2
            i32.const 0
            i32.store8 offset=28
            local.get 2
            i32.const 0
            i32.store8 offset=24
            local.get 2
            i32.const 0
            i32.store offset=20
            local.get 2
            local.get 0
            i32.store offset=16
            local.get 2
            i64.const 4398046511104
            i64.store offset=8 align=4
            local.get 2
            i64.const 0
            i64.store align=4
            i32.const 0
            i32.const 3
            i32.store8 offset=1055460
          end
          local.get 1
          i32.const 32
          i32.add
          global.set $__stack_pointer
          return
        end
        i32.const 1048772
        i32.const 43
        i32.const 1050816
        call $_ZN4core9panicking5panic17h89e93701a34d6dc7E
        unreachable
      end
      i32.const 1
      i32.const 1024
      call $_ZN5alloc5alloc18handle_alloc_error17h76ec24bcd8f509d8E
      unreachable
    end
    local.get 1
    i32.const 20
    i32.add
    i64.const 0
    i64.store align=4
    local.get 1
    i32.const 1
    i32.store offset=12
    local.get 1
    i32.const 1052088
    i32.store offset=8
    local.get 1
    i32.const 1048632
    i32.store offset=16
    local.get 1
    i32.const 8
    i32.add
    i32.const 1050800
    call $_ZN4core9panicking9panic_fmt17h2298343e5f8a8826E
    unreachable
  )
  (func $_ZN3std10sys_common9backtrace5print17h7410421ae2f94ef2E (;78;) (type 7) (param i32 i32 i32 i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    i32.const 0
    i32.load8_u offset=1055468
    local.set 5
    i32.const 1
    local.set 6
    i32.const 0
    i32.const 1
    i32.store8 offset=1055468
    local.get 4
    local.get 5
    i32.store8 offset=36
    block ;; label = @1
      local.get 5
      br_if 0 (;@1;)
      block ;; label = @2
        i32.const 0
        i32.load offset=1055492
        i32.const 2147483647
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        call $_ZN3std9panicking11panic_count17is_zero_slow_path17hb2a28c6a6d133550E
        local.set 6
      end
      local.get 2
      i32.const 36
      i32.add
      i32.load
      local.set 5
      local.get 4
      i32.const 24
      i32.add
      i64.const 1
      i64.store align=4
      local.get 4
      i32.const 1
      i32.store offset=16
      local.get 4
      i32.const 1049368
      i32.store offset=12
      local.get 4
      i32.const 6
      i32.store offset=40
      local.get 4
      local.get 3
      i32.store8 offset=47
      local.get 4
      local.get 4
      i32.const 36
      i32.add
      i32.store offset=20
      local.get 4
      local.get 4
      i32.const 47
      i32.add
      i32.store offset=36
      local.get 0
      local.get 1
      local.get 4
      i32.const 12
      i32.add
      local.get 5
      call_indirect (type 3)
      block ;; label = @2
        local.get 6
        i32.eqz
        br_if 0 (;@2;)
        i32.const 0
        i32.load offset=1055492
        i32.const 2147483647
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        call $_ZN3std9panicking11panic_count17is_zero_slow_path17hb2a28c6a6d133550E
        br_if 0 (;@2;)
        i32.const 0
        i32.const 1
        i32.store8 offset=1055469
      end
      i32.const 0
      i32.const 0
      i32.store8 offset=1055468
      local.get 4
      i32.const 48
      i32.add
      global.set $__stack_pointer
      return
    end
    local.get 4
    i64.const 0
    i64.store offset=24 align=4
    local.get 4
    i32.const 1048632
    i32.store offset=20
    local.get 4
    i32.const 1
    i32.store offset=16
    local.get 4
    i32.const 1050692
    i32.store offset=12
    local.get 4
    i32.const 36
    i32.add
    local.get 4
    i32.const 12
    i32.add
    call $_ZN4core9panicking13assert_failed17h36d5dfceec7301c8E
    unreachable
  )
  (func $_ZN91_$LT$std..sys_common..backtrace.._print..DisplayBacktrace$u20$as$u20$core..fmt..Display$GT$3fmt17ha679cf9b2fd90b85E (;79;) (type 2) (param i32 i32) (result i32)
    (local i32 i32 i64 i32 i32 i32 i32)
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
    call $_ZN3std3env11current_dir17hc84cb768eaa2cd85E
    local.get 2
    i64.load offset=12 align=4
    local.set 4
    block ;; label = @1
      local.get 2
      i32.load offset=8
      local.tee 5
      i32.const -2147483648
      i32.ne
      br_if 0 (;@1;)
      local.get 4
      i64.const 255
      i64.and
      i64.const 3
      i64.ne
      br_if 0 (;@1;)
      local.get 4
      i64.const 32
      i64.shr_u
      i32.wrap_i64
      local.tee 0
      i32.load
      local.tee 6
      local.get 0
      i32.const 4
      i32.add
      i32.load
      local.tee 7
      i32.load
      call_indirect (type 0)
      block ;; label = @2
        local.get 7
        i32.load offset=4
        local.tee 8
        i32.eqz
        br_if 0 (;@2;)
        local.get 6
        local.get 8
        local.get 7
        i32.load offset=8
        call $__rust_dealloc
      end
      local.get 0
      i32.const 12
      i32.const 4
      call $__rust_dealloc
    end
    local.get 2
    i32.const 20
    i32.add
    i64.const 0
    i64.store align=4
    i32.const 1
    local.set 0
    local.get 2
    i32.const 1
    i32.store offset=12
    local.get 2
    i32.const 1051004
    i32.store offset=8
    local.get 2
    i32.const 1048632
    i32.store offset=16
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 1
          local.get 2
          i32.const 8
          i32.add
          call $_ZN4core3fmt9Formatter9write_fmt17h4aae3abbd0cb752aE
          br_if 0 (;@3;)
          block ;; label = @4
            local.get 3
            i32.const 255
            i32.and
            br_if 0 (;@4;)
            local.get 2
            i32.const 20
            i32.add
            i64.const 0
            i64.store align=4
            local.get 2
            i32.const 1
            i32.store offset=12
            local.get 2
            i32.const 1051100
            i32.store offset=8
            local.get 2
            i32.const 1048632
            i32.store offset=16
            local.get 1
            local.get 2
            i32.const 8
            i32.add
            call $_ZN4core3fmt9Formatter9write_fmt17h4aae3abbd0cb752aE
            br_if 1 (;@3;)
          end
          i32.const 0
          local.set 0
          local.get 5
          i32.const -2147483648
          i32.or
          i32.const -2147483648
          i32.eq
          br_if 2 (;@1;)
          br 1 (;@2;)
        end
        local.get 5
        i32.const -2147483648
        i32.or
        i32.const -2147483648
        i32.eq
        br_if 1 (;@1;)
      end
      local.get 4
      i32.wrap_i64
      local.get 5
      i32.const 1
      call $__rust_dealloc
    end
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
    local.get 0
  )
  (func $_ZN3std10sys_common9backtrace26__rust_end_short_backtrace17he85c2c5ab877ce1bE (;80;) (type 0) (param i32)
    local.get 0
    call $_ZN3std9panicking19begin_panic_handler28_$u7b$$u7b$closure$u7d$$u7d$17h738b5516085eb0f6E
    unreachable
  )
  (func $_ZN3std9panicking19begin_panic_handler28_$u7b$$u7b$closure$u7d$$u7d$17h738b5516085eb0f6E (;81;) (type 0) (param i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    local.get 0
    i32.load
    local.tee 2
    i32.const 12
    i32.add
    i32.load
    local.set 3
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            local.get 2
            i32.load offset=4
            br_table 0 (;@4;) 1 (;@3;) 3 (;@1;)
          end
          local.get 3
          br_if 2 (;@1;)
          i32.const 1048632
          local.set 2
          i32.const 0
          local.set 3
          br 1 (;@2;)
        end
        local.get 3
        br_if 1 (;@1;)
        local.get 2
        i32.load
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
      i32.const 1051556
      local.get 0
      i32.load offset=4
      local.tee 2
      i32.load offset=8
      local.get 0
      i32.load offset=8
      local.get 2
      i32.load8_u offset=16
      local.get 2
      i32.load8_u offset=17
      call $_ZN3std9panicking20rust_panic_with_hook17h523d0e46b02eaef4E
      unreachable
    end
    local.get 1
    local.get 2
    i32.store offset=12
    local.get 1
    i32.const -2147483648
    i32.store
    local.get 1
    i32.const 1051576
    local.get 0
    i32.load offset=4
    local.tee 2
    i32.load offset=8
    local.get 0
    i32.load offset=8
    local.get 2
    i32.load8_u offset=16
    local.get 2
    i32.load8_u offset=17
    call $_ZN3std9panicking20rust_panic_with_hook17h523d0e46b02eaef4E
    unreachable
  )
  (func $_ZN3std5alloc24default_alloc_error_hook17h33eb21920aa52409E (;82;) (type 1) (param i32 i32)
    (local i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 64
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block ;; label = @1
      i32.const 0
      i32.load8_u offset=1055424
      br_if 0 (;@1;)
      local.get 2
      i32.const 24
      i32.add
      i64.const 1
      i64.store align=4
      local.get 2
      i32.const 2
      i32.store offset=16
      local.get 2
      i32.const 1051144
      i32.store offset=12
      local.get 2
      i32.const 7
      i32.store offset=40
      local.get 2
      local.get 1
      i32.store offset=44
      local.get 2
      local.get 2
      i32.const 36
      i32.add
      i32.store offset=20
      local.get 2
      local.get 2
      i32.const 44
      i32.add
      i32.store offset=36
      local.get 2
      i32.const 4
      i32.store8 offset=48
      local.get 2
      local.get 2
      i32.const 63
      i32.add
      i32.store offset=56
      local.get 2
      i32.const 48
      i32.add
      i32.const 1049044
      local.get 2
      i32.const 12
      i32.add
      call $_ZN4core3fmt5write17h2d8708cc7ff40a92E
      local.set 3
      local.get 2
      i32.load8_u offset=48
      local.set 1
      block ;; label = @2
        block ;; label = @3
          local.get 3
          i32.eqz
          br_if 0 (;@3;)
          local.get 1
          i32.const 4
          i32.eq
          br_if 1 (;@2;)
          local.get 2
          i32.load offset=52
          local.set 3
          block ;; label = @4
            local.get 2
            i32.load8_u offset=48
            local.tee 1
            i32.const 4
            i32.gt_u
            br_if 0 (;@4;)
            local.get 1
            i32.const 3
            i32.ne
            br_if 2 (;@2;)
          end
          local.get 3
          i32.load
          local.tee 4
          local.get 3
          i32.const 4
          i32.add
          i32.load
          local.tee 1
          i32.load
          call_indirect (type 0)
          block ;; label = @4
            local.get 1
            i32.load offset=4
            local.tee 5
            i32.eqz
            br_if 0 (;@4;)
            local.get 4
            local.get 5
            local.get 1
            i32.load offset=8
            call $__rust_dealloc
          end
          local.get 3
          i32.const 12
          i32.const 4
          call $__rust_dealloc
          br 1 (;@2;)
        end
        local.get 2
        i32.load offset=52
        local.set 3
        block ;; label = @3
          local.get 1
          i32.const 4
          i32.gt_u
          br_if 0 (;@3;)
          local.get 1
          i32.const 3
          i32.ne
          br_if 1 (;@2;)
        end
        local.get 3
        i32.load
        local.tee 4
        local.get 3
        i32.const 4
        i32.add
        i32.load
        local.tee 1
        i32.load
        call_indirect (type 0)
        block ;; label = @3
          local.get 1
          i32.load offset=4
          local.tee 5
          i32.eqz
          br_if 0 (;@3;)
          local.get 4
          local.get 5
          local.get 1
          i32.load offset=8
          call $__rust_dealloc
        end
        local.get 3
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
    i32.const 24
    i32.add
    i64.const 1
    i64.store align=4
    local.get 2
    i32.const 2
    i32.store offset=16
    local.get 2
    i32.const 1051176
    i32.store offset=12
    local.get 2
    i32.const 7
    i32.store offset=52
    local.get 2
    local.get 1
    i32.store offset=36
    local.get 2
    local.get 2
    i32.const 48
    i32.add
    i32.store offset=20
    local.get 2
    local.get 2
    i32.const 36
    i32.add
    i32.store offset=48
    local.get 2
    i32.const 12
    i32.add
    i32.const 1051216
    call $_ZN4core9panicking9panic_fmt17h2298343e5f8a8826E
    unreachable
  )
  (func $__rdl_alloc (;83;) (type 2) (param i32 i32) (result i32)
    (local i32)
    block ;; label = @1
      block ;; label = @2
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
      local.get 1
      local.get 0
      local.get 1
      i32.rem_u
      local.tee 2
      i32.sub
      i32.const 0
      local.get 2
      select
      local.get 0
      i32.add
      call $aligned_alloc
      return
    end
    local.get 0
    call $malloc
  )
  (func $__rdl_dealloc (;84;) (type 3) (param i32 i32 i32)
    local.get 0
    call $free
  )
  (func $__rdl_realloc (;85;) (type 5) (param i32 i32 i32 i32) (result i32)
    (local i32 i32)
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 2
          i32.const 8
          i32.gt_u
          br_if 0 (;@3;)
          local.get 2
          local.get 3
          i32.le_u
          br_if 1 (;@2;)
        end
        i32.const 0
        local.set 4
        local.get 2
        local.get 2
        local.get 3
        local.get 2
        i32.rem_u
        local.tee 5
        i32.sub
        i32.const 0
        local.get 5
        select
        local.get 3
        i32.add
        call $aligned_alloc
        local.tee 2
        i32.eqz
        br_if 1 (;@1;)
        local.get 2
        local.get 0
        local.get 1
        local.get 3
        local.get 1
        local.get 3
        i32.lt_u
        select
        call $memcpy
        local.set 2
        local.get 0
        call $free
        local.get 2
        return
      end
      local.get 0
      local.get 3
      call $realloc
      local.set 4
    end
    local.get 4
  )
  (func $_ZN3std9panicking12default_hook28_$u7b$$u7b$closure$u7d$$u7d$17h3542a0b08d22dec8E (;86;) (type 3) (param i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32)
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
    i64.const 3
    i64.store align=4
    local.get 3
    i32.const 60
    i32.add
    i32.const 4
    i32.store
    local.get 3
    i32.const 40
    i32.add
    i32.const 12
    i32.add
    i32.const 8
    i32.store
    local.get 3
    i32.const 1051388
    i32.store offset=16
    local.get 3
    i32.const 4
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
    i32.store offset=24
    local.get 3
    i32.const 4
    i32.store offset=20
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
    call_indirect (type 3)
    local.get 3
    i32.load offset=12
    local.set 5
    block ;; label = @1
      block ;; label = @2
        local.get 3
        i32.load8_u offset=8
        local.tee 6
        i32.const 4
        i32.gt_u
        br_if 0 (;@2;)
        local.get 6
        i32.const 3
        i32.ne
        br_if 1 (;@1;)
      end
      local.get 5
      i32.load
      local.tee 7
      local.get 5
      i32.const 4
      i32.add
      i32.load
      local.tee 6
      i32.load
      call_indirect (type 0)
      block ;; label = @2
        local.get 6
        i32.load offset=4
        local.tee 8
        i32.eqz
        br_if 0 (;@2;)
        local.get 7
        local.get 8
        local.get 6
        i32.load offset=8
        call $__rust_dealloc
      end
      local.get 5
      i32.const 12
      i32.const 4
      call $__rust_dealloc
    end
    block ;; label = @1
      local.get 0
      i32.load offset=12
      i32.load8_u
      local.tee 0
      i32.const 3
      i32.eq
      br_if 0 (;@1;)
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            local.get 0
            br_table 0 (;@4;) 1 (;@3;) 2 (;@2;) 0 (;@4;)
          end
          local.get 3
          i32.const 40
          i32.add
          local.get 1
          local.get 2
          i32.const 0
          call $_ZN3std10sys_common9backtrace5print17h7410421ae2f94ef2E
          local.get 3
          i32.load offset=44
          local.set 1
          block ;; label = @4
            local.get 3
            i32.load8_u offset=40
            local.tee 0
            i32.const 4
            i32.gt_u
            br_if 0 (;@4;)
            local.get 0
            i32.const 3
            i32.ne
            br_if 3 (;@1;)
          end
          local.get 1
          i32.load
          local.tee 2
          local.get 1
          i32.const 4
          i32.add
          i32.load
          local.tee 0
          i32.load
          call_indirect (type 0)
          block ;; label = @4
            local.get 0
            i32.load offset=4
            local.tee 6
            i32.eqz
            br_if 0 (;@4;)
            local.get 2
            local.get 6
            local.get 0
            i32.load offset=8
            call $__rust_dealloc
          end
          local.get 1
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
        call $_ZN3std10sys_common9backtrace5print17h7410421ae2f94ef2E
        local.get 3
        i32.load offset=44
        local.set 1
        block ;; label = @3
          local.get 3
          i32.load8_u offset=40
          local.tee 0
          i32.const 4
          i32.gt_u
          br_if 0 (;@3;)
          local.get 0
          i32.const 3
          i32.ne
          br_if 2 (;@1;)
        end
        local.get 1
        i32.load
        local.tee 2
        local.get 1
        i32.const 4
        i32.add
        i32.load
        local.tee 0
        i32.load
        call_indirect (type 0)
        block ;; label = @3
          local.get 0
          i32.load offset=4
          local.tee 6
          i32.eqz
          br_if 0 (;@3;)
          local.get 2
          local.get 6
          local.get 0
          i32.load offset=8
          call $__rust_dealloc
        end
        local.get 1
        i32.const 12
        i32.const 4
        call $__rust_dealloc
        br 1 (;@1;)
      end
      i32.const 0
      i32.load8_u offset=1055412
      local.set 0
      i32.const 0
      i32.const 0
      i32.store8 offset=1055412
      local.get 0
      i32.eqz
      br_if 0 (;@1;)
      local.get 3
      i32.const 52
      i32.add
      i64.const 0
      i64.store align=4
      local.get 3
      i32.const 1
      i32.store offset=44
      local.get 3
      i32.const 1051500
      i32.store offset=40
      local.get 3
      i32.const 1048632
      i32.store offset=48
      local.get 3
      i32.const 16
      i32.add
      local.get 1
      local.get 3
      i32.const 40
      i32.add
      local.get 4
      call_indirect (type 3)
      local.get 3
      i32.load offset=20
      local.set 1
      block ;; label = @2
        local.get 3
        i32.load8_u offset=16
        local.tee 0
        i32.const 4
        i32.gt_u
        br_if 0 (;@2;)
        local.get 0
        i32.const 3
        i32.ne
        br_if 1 (;@1;)
      end
      local.get 1
      i32.load
      local.tee 2
      local.get 1
      i32.const 4
      i32.add
      i32.load
      local.tee 0
      i32.load
      call_indirect (type 0)
      block ;; label = @2
        local.get 0
        i32.load offset=4
        local.tee 6
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        local.get 6
        local.get 0
        i32.load offset=8
        call $__rust_dealloc
      end
      local.get 1
      i32.const 12
      i32.const 4
      call $__rust_dealloc
    end
    local.get 3
    i32.const 64
    i32.add
    global.set $__stack_pointer
  )
  (func $rust_begin_unwind (;87;) (type 0) (param i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    block ;; label = @1
      local.get 0
      i32.load offset=8
      local.tee 2
      br_if 0 (;@1;)
      i32.const 1048772
      i32.const 43
      i32.const 1051508
      call $_ZN4core9panicking5panic17h89e93701a34d6dc7E
      unreachable
    end
    local.get 1
    local.get 0
    i32.load offset=12
    i32.store offset=12
    local.get 1
    local.get 0
    i32.store offset=8
    local.get 1
    local.get 2
    i32.store offset=4
    local.get 1
    i32.const 4
    i32.add
    call $_ZN3std10sys_common9backtrace26__rust_end_short_backtrace17he85c2c5ab877ce1bE
    unreachable
  )
  (func $_ZN102_$LT$std..panicking..begin_panic_handler..FormatStringPayload$u20$as$u20$core..panic..PanicPayload$GT$8take_box17he3eba7e0d8affceeE (;88;) (type 1) (param i32 i32)
    (local i32 i32 i32 i64)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block ;; label = @1
      local.get 1
      i32.load
      i32.const -2147483648
      i32.ne
      br_if 0 (;@1;)
      local.get 1
      i32.load offset=12
      local.set 3
      local.get 2
      i32.const 36
      i32.add
      i32.const 8
      i32.add
      local.tee 4
      i32.const 0
      i32.store
      local.get 2
      i64.const 4294967296
      i64.store offset=36 align=4
      local.get 2
      i32.const 36
      i32.add
      i32.const 1049020
      local.get 3
      call $_ZN4core3fmt5write17h2d8708cc7ff40a92E
      drop
      local.get 2
      i32.const 24
      i32.add
      i32.const 8
      i32.add
      local.get 4
      i32.load
      local.tee 3
      i32.store
      local.get 2
      local.get 2
      i64.load offset=36 align=4
      local.tee 5
      i64.store offset=24
      local.get 1
      i32.const 8
      i32.add
      local.get 3
      i32.store
      local.get 1
      local.get 5
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
    i32.const 0
    i32.load8_u offset=1055425
    drop
    local.get 2
    local.get 5
    i64.store offset=8
    block ;; label = @1
      i32.const 12
      i32.const 4
      call $__rust_alloc
      local.tee 1
      br_if 0 (;@1;)
      i32.const 4
      i32.const 12
      call $_ZN5alloc5alloc18handle_alloc_error17h76ec24bcd8f509d8E
      unreachable
    end
    local.get 1
    local.get 2
    i64.load offset=8
    i64.store align=4
    local.get 1
    i32.const 8
    i32.add
    local.get 3
    i32.load
    i32.store
    local.get 0
    i32.const 1051524
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store
    local.get 2
    i32.const 48
    i32.add
    global.set $__stack_pointer
  )
  (func $_ZN102_$LT$std..panicking..begin_panic_handler..FormatStringPayload$u20$as$u20$core..panic..PanicPayload$GT$3get17h16918555c6e4fc75E (;89;) (type 1) (param i32 i32)
    (local i32 i32 i32 i64)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block ;; label = @1
      local.get 1
      i32.load
      i32.const -2147483648
      i32.ne
      br_if 0 (;@1;)
      local.get 1
      i32.load offset=12
      local.set 3
      local.get 2
      i32.const 20
      i32.add
      i32.const 8
      i32.add
      local.tee 4
      i32.const 0
      i32.store
      local.get 2
      i64.const 4294967296
      i64.store offset=20 align=4
      local.get 2
      i32.const 20
      i32.add
      i32.const 1049020
      local.get 3
      call $_ZN4core3fmt5write17h2d8708cc7ff40a92E
      drop
      local.get 2
      i32.const 8
      i32.add
      i32.const 8
      i32.add
      local.get 4
      i32.load
      local.tee 3
      i32.store
      local.get 2
      local.get 2
      i64.load offset=20 align=4
      local.tee 5
      i64.store offset=8
      local.get 1
      i32.const 8
      i32.add
      local.get 3
      i32.store
      local.get 1
      local.get 5
      i64.store align=4
    end
    local.get 0
    i32.const 1051524
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
  )
  (func $_ZN99_$LT$std..panicking..begin_panic_handler..StaticStrPayload$u20$as$u20$core..panic..PanicPayload$GT$8take_box17hdb2e0d205360a4b1E (;90;) (type 1) (param i32 i32)
    (local i32 i32)
    i32.const 0
    i32.load8_u offset=1055425
    drop
    local.get 1
    i32.load offset=4
    local.set 2
    local.get 1
    i32.load
    local.set 3
    block ;; label = @1
      i32.const 8
      i32.const 4
      call $__rust_alloc
      local.tee 1
      br_if 0 (;@1;)
      i32.const 4
      i32.const 8
      call $_ZN5alloc5alloc18handle_alloc_error17h76ec24bcd8f509d8E
      unreachable
    end
    local.get 1
    local.get 2
    i32.store offset=4
    local.get 1
    local.get 3
    i32.store
    local.get 0
    i32.const 1051540
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store
  )
  (func $_ZN99_$LT$std..panicking..begin_panic_handler..StaticStrPayload$u20$as$u20$core..panic..PanicPayload$GT$3get17h96c1e5b81ffc4d9cE (;91;) (type 1) (param i32 i32)
    local.get 0
    i32.const 1051540
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store
  )
  (func $_ZN3std9panicking20rust_panic_with_hook17h523d0e46b02eaef4E (;92;) (type 10) (param i32 i32 i32 i32 i32 i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 80
    i32.sub
    local.tee 6
    global.set $__stack_pointer
    i32.const 0
    i32.const 0
    i32.load offset=1055492
    local.tee 7
    i32.const 1
    i32.add
    i32.store offset=1055492
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  local.get 7
                  i32.const 0
                  i32.lt_s
                  br_if 0 (;@7;)
                  i32.const 0
                  i32.load8_u offset=1055508
                  br_if 1 (;@6;)
                  i32.const 0
                  i32.const 1
                  i32.store8 offset=1055508
                  i32.const 0
                  i32.const 0
                  i32.load offset=1055504
                  i32.const 1
                  i32.add
                  i32.store offset=1055504
                  local.get 6
                  local.get 5
                  i32.store8 offset=33
                  local.get 6
                  local.get 4
                  i32.store8 offset=32
                  local.get 6
                  local.get 3
                  i32.store offset=28
                  local.get 6
                  local.get 2
                  i32.store offset=24
                  local.get 6
                  i32.const 1051596
                  i32.store offset=20
                  local.get 6
                  i32.const 1048632
                  i32.store offset=16
                  i32.const 0
                  i32.load offset=1055476
                  local.tee 7
                  i32.const -1
                  i32.le_s
                  br_if 5 (;@2;)
                  i32.const 0
                  local.get 7
                  i32.const 1
                  i32.add
                  i32.store offset=1055476
                  i32.const 0
                  i32.load offset=1055484
                  local.set 7
                  local.get 6
                  local.get 0
                  local.get 1
                  i32.load offset=16
                  call_indirect (type 1)
                  local.get 6
                  local.get 6
                  i64.load
                  i64.store offset=16 align=4
                  local.get 7
                  i32.eqz
                  br_if 3 (;@4;)
                  i32.const 0
                  i32.load offset=1055484
                  local.get 6
                  i32.const 16
                  i32.add
                  i32.const 0
                  i32.load offset=1055488
                  i32.load offset=20
                  call_indirect (type 1)
                  br 4 (;@3;)
                end
                local.get 6
                local.get 5
                i32.store8 offset=33
                local.get 6
                local.get 4
                i32.store8 offset=32
                local.get 6
                local.get 3
                i32.store offset=28
                local.get 6
                local.get 2
                i32.store offset=24
                local.get 6
                i32.const 1051596
                i32.store offset=20
                local.get 6
                i32.const 1048632
                i32.store offset=16
                local.get 6
                i32.const 52
                i32.add
                i64.const 1
                i64.store align=4
                local.get 6
                i32.const 2
                i32.store offset=44
                local.get 6
                i32.const 1051664
                i32.store offset=40
                local.get 6
                i32.const 9
                i32.store offset=12
                local.get 6
                local.get 6
                i32.const 8
                i32.add
                i32.store offset=48
                local.get 6
                local.get 6
                i32.const 16
                i32.add
                i32.store offset=8
                local.get 6
                i32.const 4
                i32.store8 offset=64
                local.get 6
                local.get 6
                i32.const 8
                i32.add
                i32.store offset=72
                local.get 6
                i32.const 64
                i32.add
                i32.const 1049044
                local.get 6
                i32.const 40
                i32.add
                call $_ZN4core3fmt5write17h2d8708cc7ff40a92E
                local.set 4
                local.get 6
                i32.load8_u offset=64
                local.set 7
                block ;; label = @7
                  local.get 4
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 7
                  i32.const 4
                  i32.eq
                  br_if 2 (;@5;)
                  local.get 6
                  i32.load offset=68
                  local.set 7
                  block ;; label = @8
                    local.get 6
                    i32.load8_u offset=64
                    local.tee 6
                    i32.const 4
                    i32.gt_u
                    br_if 0 (;@8;)
                    local.get 6
                    i32.const 3
                    i32.ne
                    br_if 3 (;@5;)
                  end
                  local.get 7
                  i32.load
                  local.tee 4
                  local.get 7
                  i32.const 4
                  i32.add
                  i32.load
                  local.tee 6
                  i32.load
                  call_indirect (type 0)
                  block ;; label = @8
                    local.get 6
                    i32.load offset=4
                    local.tee 5
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 4
                    local.get 5
                    local.get 6
                    i32.load offset=8
                    call $__rust_dealloc
                  end
                  local.get 7
                  i32.const 12
                  i32.const 4
                  call $__rust_dealloc
                  call $_ZN3std3sys4wasi14abort_internal17hc2157ea3f05ea364E
                  unreachable
                end
                local.get 6
                i32.load offset=68
                local.set 6
                block ;; label = @7
                  local.get 7
                  i32.const 4
                  i32.gt_u
                  br_if 0 (;@7;)
                  local.get 7
                  i32.const 3
                  i32.ne
                  br_if 2 (;@5;)
                end
                local.get 6
                i32.load
                local.tee 4
                local.get 6
                i32.const 4
                i32.add
                i32.load
                local.tee 7
                i32.load
                call_indirect (type 0)
                block ;; label = @7
                  local.get 7
                  i32.load offset=4
                  local.tee 5
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 4
                  local.get 5
                  local.get 7
                  i32.load offset=8
                  call $__rust_dealloc
                end
                local.get 6
                i32.const 12
                i32.const 4
                call $__rust_dealloc
                call $_ZN3std3sys4wasi14abort_internal17hc2157ea3f05ea364E
                unreachable
              end
              local.get 6
              i32.const 52
              i32.add
              i64.const 0
              i64.store align=4
              local.get 6
              i32.const 1
              i32.store offset=44
              local.get 6
              i32.const 1051732
              i32.store offset=40
              local.get 6
              i32.const 1048632
              i32.store offset=48
              local.get 6
              i32.const 4
              i32.store8 offset=16
              local.get 6
              local.get 6
              i32.const 8
              i32.add
              i32.store offset=24
              local.get 6
              i32.const 16
              i32.add
              i32.const 1049044
              local.get 6
              i32.const 40
              i32.add
              call $_ZN4core3fmt5write17h2d8708cc7ff40a92E
              local.set 4
              local.get 6
              i32.load8_u offset=16
              local.set 7
              block ;; label = @6
                local.get 4
                i32.eqz
                br_if 0 (;@6;)
                local.get 7
                i32.const 4
                i32.eq
                br_if 1 (;@5;)
                local.get 6
                i32.load offset=20
                local.set 7
                block ;; label = @7
                  local.get 6
                  i32.load8_u offset=16
                  local.tee 6
                  i32.const 4
                  i32.gt_u
                  br_if 0 (;@7;)
                  local.get 6
                  i32.const 3
                  i32.ne
                  br_if 2 (;@5;)
                end
                local.get 7
                i32.load
                local.tee 4
                local.get 7
                i32.const 4
                i32.add
                i32.load
                local.tee 6
                i32.load
                call_indirect (type 0)
                block ;; label = @7
                  local.get 6
                  i32.load offset=4
                  local.tee 5
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 4
                  local.get 5
                  local.get 6
                  i32.load offset=8
                  call $__rust_dealloc
                end
                local.get 7
                i32.const 12
                i32.const 4
                call $__rust_dealloc
                call $_ZN3std3sys4wasi14abort_internal17hc2157ea3f05ea364E
                unreachable
              end
              local.get 6
              i32.load offset=20
              local.set 6
              block ;; label = @6
                local.get 7
                i32.const 4
                i32.gt_u
                br_if 0 (;@6;)
                local.get 7
                i32.const 3
                i32.ne
                br_if 1 (;@5;)
              end
              local.get 6
              i32.load
              local.tee 4
              local.get 6
              i32.const 4
              i32.add
              i32.load
              local.tee 7
              i32.load
              call_indirect (type 0)
              block ;; label = @6
                local.get 7
                i32.load offset=4
                local.tee 5
                i32.eqz
                br_if 0 (;@6;)
                local.get 4
                local.get 5
                local.get 7
                i32.load offset=8
                call $__rust_dealloc
              end
              local.get 6
              i32.const 12
              i32.const 4
              call $__rust_dealloc
            end
            call $_ZN3std3sys4wasi14abort_internal17hc2157ea3f05ea364E
            unreachable
          end
          local.get 6
          i32.const 16
          i32.add
          call $_ZN3std9panicking12default_hook17h37c9d1490f380811E
        end
        i32.const 0
        i32.const 0
        i32.load offset=1055476
        i32.const -1
        i32.add
        i32.store offset=1055476
        i32.const 0
        i32.const 0
        i32.store8 offset=1055508
        local.get 4
        i32.eqz
        br_if 1 (;@1;)
        local.get 0
        local.get 1
        call $rust_panic
        unreachable
      end
      local.get 6
      i32.const 52
      i32.add
      i64.const 0
      i64.store align=4
      local.get 6
      i32.const 1
      i32.store offset=44
      local.get 6
      i32.const 1052024
      i32.store offset=40
      local.get 6
      local.get 6
      i32.const 8
      i32.add
      i32.store offset=48
      local.get 6
      i32.const 64
      i32.add
      local.get 6
      i32.const 8
      i32.add
      local.get 6
      i32.const 40
      i32.add
      call $_ZN3std2io5Write9write_fmt17hb2f06b1b320117c2E
      local.get 6
      i32.load8_u offset=64
      local.get 6
      i32.load offset=68
      call $_ZN4core3ptr81drop_in_place$LT$core..result..Result$LT$$LP$$RP$$C$std..io..error..Error$GT$$GT$17h124ae7e4b5e3d03eE
      call $_ZN3std3sys4wasi14abort_internal17hc2157ea3f05ea364E
      unreachable
    end
    local.get 6
    i32.const 52
    i32.add
    i64.const 0
    i64.store align=4
    local.get 6
    i32.const 1
    i32.store offset=44
    local.get 6
    i32.const 1051788
    i32.store offset=40
    local.get 6
    i32.const 1048632
    i32.store offset=48
    local.get 6
    i32.const 64
    i32.add
    local.get 6
    i32.const 8
    i32.add
    local.get 6
    i32.const 40
    i32.add
    call $_ZN3std2io5Write9write_fmt17hb2f06b1b320117c2E
    local.get 6
    i32.load8_u offset=64
    local.get 6
    i32.load offset=68
    call $_ZN4core3ptr81drop_in_place$LT$core..result..Result$LT$$LP$$RP$$C$std..io..error..Error$GT$$GT$17h124ae7e4b5e3d03eE
    call $_ZN3std3sys4wasi14abort_internal17hc2157ea3f05ea364E
    unreachable
  )
  (func $rust_panic (;93;) (type 1) (param i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    local.get 0
    local.get 1
    call $__rust_start_panic
    i32.store
    local.get 2
    i32.const 24
    i32.add
    i64.const 1
    i64.store align=4
    local.get 2
    i32.const 2
    i32.store offset=16
    local.get 2
    i32.const 1051852
    i32.store offset=12
    local.get 2
    i32.const 7
    i32.store offset=40
    local.get 2
    local.get 2
    i32.const 36
    i32.add
    i32.store offset=20
    local.get 2
    local.get 2
    i32.store offset=36
    local.get 2
    i32.const 4
    i32.add
    local.get 2
    i32.const 47
    i32.add
    local.get 2
    i32.const 12
    i32.add
    call $_ZN3std2io5Write9write_fmt17hb2f06b1b320117c2E
    local.get 2
    i32.load8_u offset=4
    local.get 2
    i32.load offset=8
    call $_ZN4core3ptr81drop_in_place$LT$core..result..Result$LT$$LP$$RP$$C$std..io..error..Error$GT$$GT$17h124ae7e4b5e3d03eE
    call $_ZN3std3sys4wasi14abort_internal17hc2157ea3f05ea364E
    unreachable
  )
  (func $_ZN64_$LT$std..sys..wasi..stdio..Stderr$u20$as$u20$std..io..Write$GT$5write17hbdd3c5ac45c1ded0E (;94;) (type 7) (param i32 i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    local.get 4
    local.get 3
    i32.store offset=4
    local.get 4
    local.get 2
    i32.store
    local.get 4
    i32.const 8
    i32.add
    i32.const 2
    local.get 4
    i32.const 1
    call $_ZN4wasi13lib_generated8fd_write17h5d845406ea9346aaE
    block ;; label = @1
      block ;; label = @2
        local.get 4
        i32.load16_u offset=8
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
      i64.load16_u offset=10
      i64.const 32
      i64.shl
      i64.store align=4
    end
    local.get 4
    i32.const 16
    i32.add
    global.set $__stack_pointer
  )
  (func $_ZN64_$LT$std..sys..wasi..stdio..Stderr$u20$as$u20$std..io..Write$GT$14write_vectored17h6387938e85325605E (;95;) (type 7) (param i32 i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    local.get 4
    i32.const 8
    i32.add
    i32.const 2
    local.get 2
    local.get 3
    call $_ZN4wasi13lib_generated8fd_write17h5d845406ea9346aaE
    block ;; label = @1
      block ;; label = @2
        local.get 4
        i32.load16_u offset=8
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
      i64.load16_u offset=10
      i64.const 32
      i64.shl
      i64.store align=4
    end
    local.get 4
    i32.const 16
    i32.add
    global.set $__stack_pointer
  )
  (func $_ZN64_$LT$std..sys..wasi..stdio..Stderr$u20$as$u20$std..io..Write$GT$17is_write_vectored17h360e1c632c87e1f7E (;96;) (type 9) (param i32) (result i32)
    i32.const 1
  )
  (func $_ZN64_$LT$std..sys..wasi..stdio..Stderr$u20$as$u20$std..io..Write$GT$5flush17h9046415f8d5293a1E (;97;) (type 1) (param i32 i32)
    local.get 0
    i32.const 4
    i32.store8
  )
  (func $_ZN3std5alloc8rust_oom17hc65343be3fcaec13E (;98;) (type 1) (param i32 i32)
    (local i32)
    local.get 0
    local.get 1
    i32.const 0
    i32.load offset=1055472
    local.tee 2
    i32.const 10
    local.get 2
    select
    call_indirect (type 1)
    call $_ZN3std7process5abort17h8ff01d99bdc8af7eE
    unreachable
  )
  (func $__rg_oom (;99;) (type 1) (param i32 i32)
    local.get 1
    local.get 0
    call $_ZN3std5alloc8rust_oom17hc65343be3fcaec13E
    unreachable
  )
  (func $__rust_start_panic (;100;) (type 2) (param i32 i32) (result i32)
    unreachable
    unreachable
  )
  (func $_ZN4wasi13lib_generated8fd_write17h5d845406ea9346aaE (;101;) (type 7) (param i32 i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        local.get 1
        local.get 2
        local.get 3
        local.get 4
        i32.const 12
        i32.add
        call $_ZN4wasi13lib_generated22wasi_snapshot_preview18fd_write17h9f1c185719eccf6aE
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
    global.set $__stack_pointer
  )
  (func $malloc (;102;) (type 9) (param i32) (result i32)
    local.get 0
    call $dlmalloc
  )
  (func $dlmalloc (;103;) (type 9) (param i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    block ;; label = @9
                      block ;; label = @10
                        block ;; label = @11
                          block ;; label = @12
                            i32.const 0
                            i32.load offset=1055552
                            local.tee 2
                            br_if 0 (;@12;)
                            block ;; label = @13
                              i32.const 0
                              i32.load offset=1056000
                              local.tee 3
                              br_if 0 (;@13;)
                              i32.const 0
                              i64.const -1
                              i64.store offset=1056012 align=4
                              i32.const 0
                              i64.const 281474976776192
                              i64.store offset=1056004 align=4
                              i32.const 0
                              local.get 1
                              i32.const 8
                              i32.add
                              i32.const -16
                              i32.and
                              i32.const 1431655768
                              i32.xor
                              local.tee 3
                              i32.store offset=1056000
                              i32.const 0
                              i32.const 0
                              i32.store offset=1056020
                              i32.const 0
                              i32.const 0
                              i32.store offset=1055972
                            end
                            i32.const 1114112
                            i32.const 1056064
                            i32.lt_u
                            br_if 1 (;@11;)
                            i32.const 0
                            local.set 2
                            i32.const 1114112
                            i32.const 1056064
                            i32.sub
                            i32.const 89
                            i32.lt_u
                            br_if 0 (;@12;)
                            i32.const 0
                            local.set 4
                            i32.const 0
                            i32.const 1056064
                            i32.store offset=1055976
                            i32.const 0
                            i32.const 1056064
                            i32.store offset=1055544
                            i32.const 0
                            local.get 3
                            i32.store offset=1055564
                            i32.const 0
                            i32.const -1
                            i32.store offset=1055560
                            i32.const 0
                            i32.const 1114112
                            i32.const 1056064
                            i32.sub
                            i32.store offset=1055980
                            loop ;; label = @13
                              local.get 4
                              i32.const 1055588
                              i32.add
                              local.get 4
                              i32.const 1055576
                              i32.add
                              local.tee 3
                              i32.store
                              local.get 3
                              local.get 4
                              i32.const 1055568
                              i32.add
                              local.tee 5
                              i32.store
                              local.get 4
                              i32.const 1055580
                              i32.add
                              local.get 5
                              i32.store
                              local.get 4
                              i32.const 1055596
                              i32.add
                              local.get 4
                              i32.const 1055584
                              i32.add
                              local.tee 5
                              i32.store
                              local.get 5
                              local.get 3
                              i32.store
                              local.get 4
                              i32.const 1055604
                              i32.add
                              local.get 4
                              i32.const 1055592
                              i32.add
                              local.tee 3
                              i32.store
                              local.get 3
                              local.get 5
                              i32.store
                              local.get 4
                              i32.const 1055600
                              i32.add
                              local.get 3
                              i32.store
                              local.get 4
                              i32.const 32
                              i32.add
                              local.tee 4
                              i32.const 256
                              i32.ne
                              br_if 0 (;@13;)
                            end
                            i32.const 1056064
                            i32.const -8
                            i32.const 1056064
                            i32.sub
                            i32.const 15
                            i32.and
                            i32.const 0
                            i32.const 1056064
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
                            i32.const 1114112
                            i32.const 1056064
                            i32.sub
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
                            i32.load offset=1056016
                            i32.store offset=1055556
                            i32.const 0
                            local.get 4
                            i32.store offset=1055540
                            i32.const 0
                            local.get 2
                            i32.store offset=1055552
                            local.get 3
                            i32.const 1056064
                            i32.add
                            i32.const 4
                            i32.add
                            i32.const 56
                            i32.store
                          end
                          block ;; label = @12
                            block ;; label = @13
                              local.get 0
                              i32.const 236
                              i32.gt_u
                              br_if 0 (;@13;)
                              block ;; label = @14
                                i32.const 0
                                i32.load offset=1055528
                                local.tee 6
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
                                local.tee 7
                                i32.const 3
                                i32.shr_u
                                local.tee 3
                                i32.shr_u
                                local.tee 4
                                i32.const 3
                                i32.and
                                i32.eqz
                                br_if 0 (;@14;)
                                block ;; label = @15
                                  block ;; label = @16
                                    local.get 4
                                    i32.const 1
                                    i32.and
                                    local.get 3
                                    i32.or
                                    i32.const 1
                                    i32.xor
                                    local.tee 5
                                    i32.const 3
                                    i32.shl
                                    local.tee 3
                                    i32.const 1055568
                                    i32.add
                                    local.tee 4
                                    local.get 3
                                    i32.const 1055576
                                    i32.add
                                    i32.load
                                    local.tee 3
                                    i32.load offset=8
                                    local.tee 7
                                    i32.ne
                                    br_if 0 (;@16;)
                                    i32.const 0
                                    local.get 6
                                    i32.const -2
                                    local.get 5
                                    i32.rotl
                                    i32.and
                                    i32.store offset=1055528
                                    br 1 (;@15;)
                                  end
                                  local.get 4
                                  local.get 7
                                  i32.store offset=8
                                  local.get 7
                                  local.get 4
                                  i32.store offset=12
                                end
                                local.get 3
                                i32.const 8
                                i32.add
                                local.set 4
                                local.get 3
                                local.get 5
                                i32.const 3
                                i32.shl
                                local.tee 5
                                i32.const 3
                                i32.or
                                i32.store offset=4
                                local.get 3
                                local.get 5
                                i32.add
                                local.tee 3
                                local.get 3
                                i32.load offset=4
                                i32.const 1
                                i32.or
                                i32.store offset=4
                                br 13 (;@1;)
                              end
                              local.get 7
                              i32.const 0
                              i32.load offset=1055536
                              local.tee 8
                              i32.le_u
                              br_if 1 (;@12;)
                              block ;; label = @14
                                local.get 4
                                i32.eqz
                                br_if 0 (;@14;)
                                block ;; label = @15
                                  block ;; label = @16
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
                                    i32.ctz
                                    local.tee 3
                                    i32.const 3
                                    i32.shl
                                    local.tee 4
                                    i32.const 1055568
                                    i32.add
                                    local.tee 5
                                    local.get 4
                                    i32.const 1055576
                                    i32.add
                                    i32.load
                                    local.tee 4
                                    i32.load offset=8
                                    local.tee 0
                                    i32.ne
                                    br_if 0 (;@16;)
                                    i32.const 0
                                    local.get 6
                                    i32.const -2
                                    local.get 3
                                    i32.rotl
                                    i32.and
                                    local.tee 6
                                    i32.store offset=1055528
                                    br 1 (;@15;)
                                  end
                                  local.get 5
                                  local.get 0
                                  i32.store offset=8
                                  local.get 0
                                  local.get 5
                                  i32.store offset=12
                                end
                                local.get 4
                                local.get 7
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
                                local.get 7
                                i32.sub
                                local.tee 5
                                i32.store
                                local.get 4
                                local.get 7
                                i32.add
                                local.tee 0
                                local.get 5
                                i32.const 1
                                i32.or
                                i32.store offset=4
                                block ;; label = @15
                                  local.get 8
                                  i32.eqz
                                  br_if 0 (;@15;)
                                  local.get 8
                                  i32.const -8
                                  i32.and
                                  i32.const 1055568
                                  i32.add
                                  local.set 7
                                  i32.const 0
                                  i32.load offset=1055548
                                  local.set 3
                                  block ;; label = @16
                                    block ;; label = @17
                                      local.get 6
                                      i32.const 1
                                      local.get 8
                                      i32.const 3
                                      i32.shr_u
                                      i32.shl
                                      local.tee 9
                                      i32.and
                                      br_if 0 (;@17;)
                                      i32.const 0
                                      local.get 6
                                      local.get 9
                                      i32.or
                                      i32.store offset=1055528
                                      local.get 7
                                      local.set 9
                                      br 1 (;@16;)
                                    end
                                    local.get 7
                                    i32.load offset=8
                                    local.set 9
                                  end
                                  local.get 9
                                  local.get 3
                                  i32.store offset=12
                                  local.get 7
                                  local.get 3
                                  i32.store offset=8
                                  local.get 3
                                  local.get 7
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
                                i32.store offset=1055548
                                i32.const 0
                                local.get 5
                                i32.store offset=1055536
                                br 13 (;@1;)
                              end
                              i32.const 0
                              i32.load offset=1055532
                              local.tee 10
                              i32.eqz
                              br_if 1 (;@12;)
                              local.get 10
                              i32.const 0
                              local.get 10
                              i32.sub
                              i32.and
                              i32.ctz
                              i32.const 2
                              i32.shl
                              i32.const 1055832
                              i32.add
                              i32.load
                              local.tee 0
                              i32.load offset=4
                              i32.const -8
                              i32.and
                              local.get 7
                              i32.sub
                              local.set 3
                              local.get 0
                              local.set 5
                              block ;; label = @14
                                loop ;; label = @15
                                  block ;; label = @16
                                    local.get 5
                                    i32.load offset=16
                                    local.tee 4
                                    br_if 0 (;@16;)
                                    local.get 5
                                    i32.const 20
                                    i32.add
                                    i32.load
                                    local.tee 4
                                    i32.eqz
                                    br_if 2 (;@14;)
                                  end
                                  local.get 4
                                  i32.load offset=4
                                  i32.const -8
                                  i32.and
                                  local.get 7
                                  i32.sub
                                  local.tee 5
                                  local.get 3
                                  local.get 5
                                  local.get 3
                                  i32.lt_u
                                  local.tee 5
                                  select
                                  local.set 3
                                  local.get 4
                                  local.get 0
                                  local.get 5
                                  select
                                  local.set 0
                                  local.get 4
                                  local.set 5
                                  br 0 (;@15;)
                                end
                              end
                              local.get 0
                              i32.load offset=24
                              local.set 11
                              block ;; label = @14
                                local.get 0
                                i32.load offset=12
                                local.tee 9
                                local.get 0
                                i32.eq
                                br_if 0 (;@14;)
                                local.get 0
                                i32.load offset=8
                                local.tee 4
                                i32.const 0
                                i32.load offset=1055544
                                i32.lt_u
                                drop
                                local.get 9
                                local.get 4
                                i32.store offset=8
                                local.get 4
                                local.get 9
                                i32.store offset=12
                                br 12 (;@2;)
                              end
                              block ;; label = @14
                                local.get 0
                                i32.const 20
                                i32.add
                                local.tee 5
                                i32.load
                                local.tee 4
                                br_if 0 (;@14;)
                                local.get 0
                                i32.load offset=16
                                local.tee 4
                                i32.eqz
                                br_if 4 (;@10;)
                                local.get 0
                                i32.const 16
                                i32.add
                                local.set 5
                              end
                              loop ;; label = @14
                                local.get 5
                                local.set 2
                                local.get 4
                                local.tee 9
                                i32.const 20
                                i32.add
                                local.tee 5
                                i32.load
                                local.tee 4
                                br_if 0 (;@14;)
                                local.get 9
                                i32.const 16
                                i32.add
                                local.set 5
                                local.get 9
                                i32.load offset=16
                                local.tee 4
                                br_if 0 (;@14;)
                              end
                              local.get 2
                              i32.const 0
                              i32.store
                              br 11 (;@2;)
                            end
                            i32.const -1
                            local.set 7
                            local.get 0
                            i32.const -65
                            i32.gt_u
                            br_if 0 (;@12;)
                            local.get 0
                            i32.const 19
                            i32.add
                            local.tee 4
                            i32.const -16
                            i32.and
                            local.set 7
                            i32.const 0
                            i32.load offset=1055532
                            local.tee 10
                            i32.eqz
                            br_if 0 (;@12;)
                            i32.const 0
                            local.set 8
                            block ;; label = @13
                              local.get 7
                              i32.const 256
                              i32.lt_u
                              br_if 0 (;@13;)
                              i32.const 31
                              local.set 8
                              local.get 7
                              i32.const 16777215
                              i32.gt_u
                              br_if 0 (;@13;)
                              local.get 7
                              i32.const 38
                              local.get 4
                              i32.const 8
                              i32.shr_u
                              i32.clz
                              local.tee 4
                              i32.sub
                              i32.shr_u
                              i32.const 1
                              i32.and
                              local.get 4
                              i32.const 1
                              i32.shl
                              i32.sub
                              i32.const 62
                              i32.add
                              local.set 8
                            end
                            i32.const 0
                            local.get 7
                            i32.sub
                            local.set 3
                            block ;; label = @13
                              block ;; label = @14
                                block ;; label = @15
                                  block ;; label = @16
                                    local.get 8
                                    i32.const 2
                                    i32.shl
                                    i32.const 1055832
                                    i32.add
                                    i32.load
                                    local.tee 5
                                    br_if 0 (;@16;)
                                    i32.const 0
                                    local.set 4
                                    i32.const 0
                                    local.set 9
                                    br 1 (;@15;)
                                  end
                                  i32.const 0
                                  local.set 4
                                  local.get 7
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
                                  loop ;; label = @16
                                    block ;; label = @17
                                      local.get 5
                                      i32.load offset=4
                                      i32.const -8
                                      i32.and
                                      local.get 7
                                      i32.sub
                                      local.tee 6
                                      local.get 3
                                      i32.ge_u
                                      br_if 0 (;@17;)
                                      local.get 6
                                      local.set 3
                                      local.get 5
                                      local.set 9
                                      local.get 6
                                      br_if 0 (;@17;)
                                      i32.const 0
                                      local.set 3
                                      local.get 5
                                      local.set 9
                                      local.get 5
                                      local.set 4
                                      br 3 (;@14;)
                                    end
                                    local.get 4
                                    local.get 5
                                    i32.const 20
                                    i32.add
                                    i32.load
                                    local.tee 6
                                    local.get 6
                                    local.get 5
                                    local.get 0
                                    i32.const 29
                                    i32.shr_u
                                    i32.const 4
                                    i32.and
                                    i32.add
                                    i32.const 16
                                    i32.add
                                    i32.load
                                    local.tee 5
                                    i32.eq
                                    select
                                    local.get 4
                                    local.get 6
                                    select
                                    local.set 4
                                    local.get 0
                                    i32.const 1
                                    i32.shl
                                    local.set 0
                                    local.get 5
                                    br_if 0 (;@16;)
                                  end
                                end
                                block ;; label = @15
                                  local.get 4
                                  local.get 9
                                  i32.or
                                  br_if 0 (;@15;)
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
                                  br_if 3 (;@12;)
                                  local.get 4
                                  i32.const 0
                                  local.get 4
                                  i32.sub
                                  i32.and
                                  i32.ctz
                                  i32.const 2
                                  i32.shl
                                  i32.const 1055832
                                  i32.add
                                  i32.load
                                  local.set 4
                                end
                                local.get 4
                                i32.eqz
                                br_if 1 (;@13;)
                              end
                              loop ;; label = @14
                                local.get 4
                                i32.load offset=4
                                i32.const -8
                                i32.and
                                local.get 7
                                i32.sub
                                local.tee 6
                                local.get 3
                                i32.lt_u
                                local.set 0
                                block ;; label = @15
                                  local.get 4
                                  i32.load offset=16
                                  local.tee 5
                                  br_if 0 (;@15;)
                                  local.get 4
                                  i32.const 20
                                  i32.add
                                  i32.load
                                  local.set 5
                                end
                                local.get 6
                                local.get 3
                                local.get 0
                                select
                                local.set 3
                                local.get 4
                                local.get 9
                                local.get 0
                                select
                                local.set 9
                                local.get 5
                                local.set 4
                                local.get 5
                                br_if 0 (;@14;)
                              end
                            end
                            local.get 9
                            i32.eqz
                            br_if 0 (;@12;)
                            local.get 3
                            i32.const 0
                            i32.load offset=1055536
                            local.get 7
                            i32.sub
                            i32.ge_u
                            br_if 0 (;@12;)
                            local.get 9
                            i32.load offset=24
                            local.set 2
                            block ;; label = @13
                              local.get 9
                              i32.load offset=12
                              local.tee 0
                              local.get 9
                              i32.eq
                              br_if 0 (;@13;)
                              local.get 9
                              i32.load offset=8
                              local.tee 4
                              i32.const 0
                              i32.load offset=1055544
                              i32.lt_u
                              drop
                              local.get 0
                              local.get 4
                              i32.store offset=8
                              local.get 4
                              local.get 0
                              i32.store offset=12
                              br 10 (;@3;)
                            end
                            block ;; label = @13
                              local.get 9
                              i32.const 20
                              i32.add
                              local.tee 5
                              i32.load
                              local.tee 4
                              br_if 0 (;@13;)
                              local.get 9
                              i32.load offset=16
                              local.tee 4
                              i32.eqz
                              br_if 4 (;@9;)
                              local.get 9
                              i32.const 16
                              i32.add
                              local.set 5
                            end
                            loop ;; label = @13
                              local.get 5
                              local.set 6
                              local.get 4
                              local.tee 0
                              i32.const 20
                              i32.add
                              local.tee 5
                              i32.load
                              local.tee 4
                              br_if 0 (;@13;)
                              local.get 0
                              i32.const 16
                              i32.add
                              local.set 5
                              local.get 0
                              i32.load offset=16
                              local.tee 4
                              br_if 0 (;@13;)
                            end
                            local.get 6
                            i32.const 0
                            i32.store
                            br 9 (;@3;)
                          end
                          block ;; label = @12
                            i32.const 0
                            i32.load offset=1055536
                            local.tee 4
                            local.get 7
                            i32.lt_u
                            br_if 0 (;@12;)
                            i32.const 0
                            i32.load offset=1055548
                            local.set 3
                            block ;; label = @13
                              block ;; label = @14
                                local.get 4
                                local.get 7
                                i32.sub
                                local.tee 5
                                i32.const 16
                                i32.lt_u
                                br_if 0 (;@14;)
                                local.get 3
                                local.get 7
                                i32.add
                                local.tee 0
                                local.get 5
                                i32.const 1
                                i32.or
                                i32.store offset=4
                                local.get 3
                                local.get 4
                                i32.add
                                local.get 5
                                i32.store
                                local.get 3
                                local.get 7
                                i32.const 3
                                i32.or
                                i32.store offset=4
                                br 1 (;@13;)
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
                              local.set 0
                              i32.const 0
                              local.set 5
                            end
                            i32.const 0
                            local.get 5
                            i32.store offset=1055536
                            i32.const 0
                            local.get 0
                            i32.store offset=1055548
                            local.get 3
                            i32.const 8
                            i32.add
                            local.set 4
                            br 11 (;@1;)
                          end
                          block ;; label = @12
                            i32.const 0
                            i32.load offset=1055540
                            local.tee 5
                            local.get 7
                            i32.le_u
                            br_if 0 (;@12;)
                            local.get 2
                            local.get 7
                            i32.add
                            local.tee 4
                            local.get 5
                            local.get 7
                            i32.sub
                            local.tee 3
                            i32.const 1
                            i32.or
                            i32.store offset=4
                            i32.const 0
                            local.get 4
                            i32.store offset=1055552
                            i32.const 0
                            local.get 3
                            i32.store offset=1055540
                            local.get 2
                            local.get 7
                            i32.const 3
                            i32.or
                            i32.store offset=4
                            local.get 2
                            i32.const 8
                            i32.add
                            local.set 4
                            br 11 (;@1;)
                          end
                          block ;; label = @12
                            block ;; label = @13
                              i32.const 0
                              i32.load offset=1056000
                              i32.eqz
                              br_if 0 (;@13;)
                              i32.const 0
                              i32.load offset=1056008
                              local.set 3
                              br 1 (;@12;)
                            end
                            i32.const 0
                            i64.const -1
                            i64.store offset=1056012 align=4
                            i32.const 0
                            i64.const 281474976776192
                            i64.store offset=1056004 align=4
                            i32.const 0
                            local.get 1
                            i32.const 12
                            i32.add
                            i32.const -16
                            i32.and
                            i32.const 1431655768
                            i32.xor
                            i32.store offset=1056000
                            i32.const 0
                            i32.const 0
                            i32.store offset=1056020
                            i32.const 0
                            i32.const 0
                            i32.store offset=1055972
                            i32.const 65536
                            local.set 3
                          end
                          i32.const 0
                          local.set 4
                          block ;; label = @12
                            local.get 3
                            local.get 7
                            i32.const 71
                            i32.add
                            local.tee 8
                            i32.add
                            local.tee 0
                            i32.const 0
                            local.get 3
                            i32.sub
                            local.tee 6
                            i32.and
                            local.tee 9
                            local.get 7
                            i32.gt_u
                            br_if 0 (;@12;)
                            i32.const 0
                            i32.const 48
                            i32.store offset=1056024
                            br 11 (;@1;)
                          end
                          block ;; label = @12
                            i32.const 0
                            i32.load offset=1055968
                            local.tee 4
                            i32.eqz
                            br_if 0 (;@12;)
                            block ;; label = @13
                              i32.const 0
                              i32.load offset=1055960
                              local.tee 3
                              local.get 9
                              i32.add
                              local.tee 10
                              local.get 3
                              i32.le_u
                              br_if 0 (;@13;)
                              local.get 10
                              local.get 4
                              i32.le_u
                              br_if 1 (;@12;)
                            end
                            i32.const 0
                            local.set 4
                            i32.const 0
                            i32.const 48
                            i32.store offset=1056024
                            br 11 (;@1;)
                          end
                          i32.const 0
                          i32.load8_u offset=1055972
                          i32.const 4
                          i32.and
                          br_if 5 (;@6;)
                          block ;; label = @12
                            block ;; label = @13
                              block ;; label = @14
                                local.get 2
                                i32.eqz
                                br_if 0 (;@14;)
                                i32.const 1055976
                                local.set 4
                                loop ;; label = @15
                                  block ;; label = @16
                                    local.get 4
                                    i32.load
                                    local.tee 3
                                    local.get 2
                                    i32.gt_u
                                    br_if 0 (;@16;)
                                    local.get 3
                                    local.get 4
                                    i32.load offset=4
                                    i32.add
                                    local.get 2
                                    i32.gt_u
                                    br_if 3 (;@13;)
                                  end
                                  local.get 4
                                  i32.load offset=8
                                  local.tee 4
                                  br_if 0 (;@15;)
                                end
                              end
                              i32.const 0
                              call $sbrk
                              local.tee 0
                              i32.const -1
                              i32.eq
                              br_if 6 (;@7;)
                              local.get 9
                              local.set 6
                              block ;; label = @14
                                i32.const 0
                                i32.load offset=1056004
                                local.tee 4
                                i32.const -1
                                i32.add
                                local.tee 3
                                local.get 0
                                i32.and
                                i32.eqz
                                br_if 0 (;@14;)
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
                                local.set 6
                              end
                              local.get 6
                              local.get 7
                              i32.le_u
                              br_if 6 (;@7;)
                              local.get 6
                              i32.const 2147483646
                              i32.gt_u
                              br_if 6 (;@7;)
                              block ;; label = @14
                                i32.const 0
                                i32.load offset=1055968
                                local.tee 4
                                i32.eqz
                                br_if 0 (;@14;)
                                i32.const 0
                                i32.load offset=1055960
                                local.tee 3
                                local.get 6
                                i32.add
                                local.tee 5
                                local.get 3
                                i32.le_u
                                br_if 7 (;@7;)
                                local.get 5
                                local.get 4
                                i32.gt_u
                                br_if 7 (;@7;)
                              end
                              local.get 6
                              call $sbrk
                              local.tee 4
                              local.get 0
                              i32.ne
                              br_if 1 (;@12;)
                              br 8 (;@5;)
                            end
                            local.get 0
                            local.get 5
                            i32.sub
                            local.get 6
                            i32.and
                            local.tee 6
                            i32.const 2147483646
                            i32.gt_u
                            br_if 5 (;@7;)
                            local.get 6
                            call $sbrk
                            local.tee 0
                            local.get 4
                            i32.load
                            local.get 4
                            i32.load offset=4
                            i32.add
                            i32.eq
                            br_if 4 (;@8;)
                            local.get 0
                            local.set 4
                          end
                          block ;; label = @12
                            local.get 4
                            i32.const -1
                            i32.eq
                            br_if 0 (;@12;)
                            local.get 7
                            i32.const 72
                            i32.add
                            local.get 6
                            i32.le_u
                            br_if 0 (;@12;)
                            block ;; label = @13
                              local.get 8
                              local.get 6
                              i32.sub
                              i32.const 0
                              i32.load offset=1056008
                              local.tee 3
                              i32.add
                              i32.const 0
                              local.get 3
                              i32.sub
                              i32.and
                              local.tee 3
                              i32.const 2147483646
                              i32.le_u
                              br_if 0 (;@13;)
                              local.get 4
                              local.set 0
                              br 8 (;@5;)
                            end
                            block ;; label = @13
                              local.get 3
                              call $sbrk
                              i32.const -1
                              i32.eq
                              br_if 0 (;@13;)
                              local.get 3
                              local.get 6
                              i32.add
                              local.set 6
                              local.get 4
                              local.set 0
                              br 8 (;@5;)
                            end
                            i32.const 0
                            local.get 6
                            i32.sub
                            call $sbrk
                            drop
                            br 5 (;@7;)
                          end
                          local.get 4
                          local.set 0
                          local.get 4
                          i32.const -1
                          i32.ne
                          br_if 6 (;@5;)
                          br 4 (;@7;)
                        end
                        unreachable
                        unreachable
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
                i32.load offset=1055972
                i32.const 4
                i32.or
                i32.store offset=1055972
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
              local.tee 6
              local.get 7
              i32.const 56
              i32.add
              i32.le_u
              br_if 1 (;@4;)
            end
            i32.const 0
            i32.const 0
            i32.load offset=1055960
            local.get 6
            i32.add
            local.tee 4
            i32.store offset=1055960
            block ;; label = @5
              local.get 4
              i32.const 0
              i32.load offset=1055964
              i32.le_u
              br_if 0 (;@5;)
              i32.const 0
              local.get 4
              i32.store offset=1055964
            end
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    i32.const 0
                    i32.load offset=1055552
                    local.tee 3
                    i32.eqz
                    br_if 0 (;@8;)
                    i32.const 1055976
                    local.set 4
                    loop ;; label = @9
                      local.get 0
                      local.get 4
                      i32.load
                      local.tee 5
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
                  block ;; label = @8
                    block ;; label = @9
                      i32.const 0
                      i32.load offset=1055544
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
                    i32.store offset=1055544
                  end
                  i32.const 0
                  local.set 4
                  i32.const 0
                  local.get 6
                  i32.store offset=1055980
                  i32.const 0
                  local.get 0
                  i32.store offset=1055976
                  i32.const 0
                  i32.const -1
                  i32.store offset=1055560
                  i32.const 0
                  i32.const 0
                  i32.load offset=1056000
                  i32.store offset=1055564
                  i32.const 0
                  i32.const 0
                  i32.store offset=1055988
                  loop ;; label = @8
                    local.get 4
                    i32.const 1055588
                    i32.add
                    local.get 4
                    i32.const 1055576
                    i32.add
                    local.tee 3
                    i32.store
                    local.get 3
                    local.get 4
                    i32.const 1055568
                    i32.add
                    local.tee 5
                    i32.store
                    local.get 4
                    i32.const 1055580
                    i32.add
                    local.get 5
                    i32.store
                    local.get 4
                    i32.const 1055596
                    i32.add
                    local.get 4
                    i32.const 1055584
                    i32.add
                    local.tee 5
                    i32.store
                    local.get 5
                    local.get 3
                    i32.store
                    local.get 4
                    i32.const 1055604
                    i32.add
                    local.get 4
                    i32.const 1055592
                    i32.add
                    local.tee 3
                    i32.store
                    local.get 3
                    local.get 5
                    i32.store
                    local.get 4
                    i32.const 1055600
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
                  local.get 6
                  i32.const -56
                  i32.add
                  local.tee 5
                  local.get 4
                  i32.sub
                  local.tee 4
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  i32.const 0
                  i32.const 0
                  i32.load offset=1056016
                  i32.store offset=1055556
                  i32.const 0
                  local.get 4
                  i32.store offset=1055540
                  i32.const 0
                  local.get 3
                  i32.store offset=1055552
                  local.get 0
                  local.get 5
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
                local.get 5
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
                local.tee 5
                i32.add
                local.tee 0
                i32.const 0
                i32.load offset=1055540
                local.get 6
                i32.add
                local.tee 2
                local.get 5
                i32.sub
                local.tee 5
                i32.const 1
                i32.or
                i32.store offset=4
                local.get 4
                local.get 9
                local.get 6
                i32.add
                i32.store offset=4
                i32.const 0
                i32.const 0
                i32.load offset=1056016
                i32.store offset=1055556
                i32.const 0
                local.get 5
                i32.store offset=1055540
                i32.const 0
                local.get 0
                i32.store offset=1055552
                local.get 3
                local.get 2
                i32.add
                i32.const 56
                i32.store offset=4
                br 1 (;@5;)
              end
              block ;; label = @6
                local.get 0
                i32.const 0
                i32.load offset=1055544
                local.tee 9
                i32.ge_u
                br_if 0 (;@6;)
                i32.const 0
                local.get 0
                i32.store offset=1055544
                local.get 0
                local.set 9
              end
              local.get 0
              local.get 6
              i32.add
              local.set 5
              i32.const 1055976
              local.set 4
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    block ;; label = @9
                      block ;; label = @10
                        block ;; label = @11
                          block ;; label = @12
                            loop ;; label = @13
                              local.get 4
                              i32.load
                              local.get 5
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
                        i32.const 1055976
                        local.set 4
                        loop ;; label = @11
                          block ;; label = @12
                            local.get 4
                            i32.load
                            local.tee 5
                            local.get 3
                            i32.gt_u
                            br_if 0 (;@12;)
                            local.get 5
                            local.get 4
                            i32.load offset=4
                            i32.add
                            local.tee 5
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
                      local.get 6
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
                      local.get 7
                      i32.const 3
                      i32.or
                      i32.store offset=4
                      local.get 5
                      i32.const -8
                      local.get 5
                      i32.sub
                      i32.const 15
                      i32.and
                      i32.const 0
                      local.get 5
                      i32.const 8
                      i32.add
                      i32.const 15
                      i32.and
                      select
                      i32.add
                      local.tee 6
                      local.get 2
                      local.get 7
                      i32.add
                      local.tee 7
                      i32.sub
                      local.set 4
                      block ;; label = @10
                        local.get 6
                        local.get 3
                        i32.ne
                        br_if 0 (;@10;)
                        i32.const 0
                        local.get 7
                        i32.store offset=1055552
                        i32.const 0
                        i32.const 0
                        i32.load offset=1055540
                        local.get 4
                        i32.add
                        local.tee 4
                        i32.store offset=1055540
                        local.get 7
                        local.get 4
                        i32.const 1
                        i32.or
                        i32.store offset=4
                        br 3 (;@7;)
                      end
                      block ;; label = @10
                        local.get 6
                        i32.const 0
                        i32.load offset=1055548
                        i32.ne
                        br_if 0 (;@10;)
                        i32.const 0
                        local.get 7
                        i32.store offset=1055548
                        i32.const 0
                        i32.const 0
                        i32.load offset=1055536
                        local.get 4
                        i32.add
                        local.tee 4
                        i32.store offset=1055536
                        local.get 7
                        local.get 4
                        i32.const 1
                        i32.or
                        i32.store offset=4
                        local.get 7
                        local.get 4
                        i32.add
                        local.get 4
                        i32.store
                        br 3 (;@7;)
                      end
                      block ;; label = @10
                        local.get 6
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
                        block ;; label = @11
                          block ;; label = @12
                            local.get 3
                            i32.const 255
                            i32.gt_u
                            br_if 0 (;@12;)
                            local.get 6
                            i32.load offset=8
                            local.tee 5
                            local.get 3
                            i32.const 3
                            i32.shr_u
                            local.tee 9
                            i32.const 3
                            i32.shl
                            i32.const 1055568
                            i32.add
                            local.tee 0
                            i32.eq
                            drop
                            block ;; label = @13
                              local.get 6
                              i32.load offset=12
                              local.tee 3
                              local.get 5
                              i32.ne
                              br_if 0 (;@13;)
                              i32.const 0
                              i32.const 0
                              i32.load offset=1055528
                              i32.const -2
                              local.get 9
                              i32.rotl
                              i32.and
                              i32.store offset=1055528
                              br 2 (;@11;)
                            end
                            local.get 3
                            local.get 0
                            i32.eq
                            drop
                            local.get 3
                            local.get 5
                            i32.store offset=8
                            local.get 5
                            local.get 3
                            i32.store offset=12
                            br 1 (;@11;)
                          end
                          local.get 6
                          i32.load offset=24
                          local.set 10
                          block ;; label = @12
                            block ;; label = @13
                              local.get 6
                              i32.load offset=12
                              local.tee 0
                              local.get 6
                              i32.eq
                              br_if 0 (;@13;)
                              local.get 6
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
                            block ;; label = @13
                              local.get 6
                              i32.const 20
                              i32.add
                              local.tee 3
                              i32.load
                              local.tee 5
                              br_if 0 (;@13;)
                              local.get 6
                              i32.const 16
                              i32.add
                              local.tee 3
                              i32.load
                              local.tee 5
                              br_if 0 (;@13;)
                              i32.const 0
                              local.set 0
                              br 1 (;@12;)
                            end
                            loop ;; label = @13
                              local.get 3
                              local.set 9
                              local.get 5
                              local.tee 0
                              i32.const 20
                              i32.add
                              local.tee 3
                              i32.load
                              local.tee 5
                              br_if 0 (;@13;)
                              local.get 0
                              i32.const 16
                              i32.add
                              local.set 3
                              local.get 0
                              i32.load offset=16
                              local.tee 5
                              br_if 0 (;@13;)
                            end
                            local.get 9
                            i32.const 0
                            i32.store
                          end
                          local.get 10
                          i32.eqz
                          br_if 0 (;@11;)
                          block ;; label = @12
                            block ;; label = @13
                              local.get 6
                              local.get 6
                              i32.load offset=28
                              local.tee 5
                              i32.const 2
                              i32.shl
                              i32.const 1055832
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
                              i32.load offset=1055532
                              i32.const -2
                              local.get 5
                              i32.rotl
                              i32.and
                              i32.store offset=1055532
                              br 2 (;@11;)
                            end
                            local.get 10
                            i32.const 16
                            i32.const 20
                            local.get 10
                            i32.load offset=16
                            local.get 6
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
                          block ;; label = @12
                            local.get 6
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
                          local.get 6
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
                        local.get 6
                        local.get 8
                        i32.add
                        local.tee 6
                        i32.load offset=4
                        local.set 3
                      end
                      local.get 6
                      local.get 3
                      i32.const -2
                      i32.and
                      i32.store offset=4
                      local.get 7
                      local.get 4
                      i32.add
                      local.get 4
                      i32.store
                      local.get 7
                      local.get 4
                      i32.const 1
                      i32.or
                      i32.store offset=4
                      block ;; label = @10
                        local.get 4
                        i32.const 255
                        i32.gt_u
                        br_if 0 (;@10;)
                        local.get 4
                        i32.const -8
                        i32.and
                        i32.const 1055568
                        i32.add
                        local.set 3
                        block ;; label = @11
                          block ;; label = @12
                            i32.const 0
                            i32.load offset=1055528
                            local.tee 5
                            i32.const 1
                            local.get 4
                            i32.const 3
                            i32.shr_u
                            i32.shl
                            local.tee 4
                            i32.and
                            br_if 0 (;@12;)
                            i32.const 0
                            local.get 5
                            local.get 4
                            i32.or
                            i32.store offset=1055528
                            local.get 3
                            local.set 4
                            br 1 (;@11;)
                          end
                          local.get 3
                          i32.load offset=8
                          local.set 4
                        end
                        local.get 4
                        local.get 7
                        i32.store offset=12
                        local.get 3
                        local.get 7
                        i32.store offset=8
                        local.get 7
                        local.get 3
                        i32.store offset=12
                        local.get 7
                        local.get 4
                        i32.store offset=8
                        br 3 (;@7;)
                      end
                      i32.const 31
                      local.set 3
                      block ;; label = @10
                        local.get 4
                        i32.const 16777215
                        i32.gt_u
                        br_if 0 (;@10;)
                        local.get 4
                        i32.const 38
                        local.get 4
                        i32.const 8
                        i32.shr_u
                        i32.clz
                        local.tee 3
                        i32.sub
                        i32.shr_u
                        i32.const 1
                        i32.and
                        local.get 3
                        i32.const 1
                        i32.shl
                        i32.sub
                        i32.const 62
                        i32.add
                        local.set 3
                      end
                      local.get 7
                      local.get 3
                      i32.store offset=28
                      local.get 7
                      i64.const 0
                      i64.store offset=16 align=4
                      local.get 3
                      i32.const 2
                      i32.shl
                      i32.const 1055832
                      i32.add
                      local.set 5
                      block ;; label = @10
                        i32.const 0
                        i32.load offset=1055532
                        local.tee 0
                        i32.const 1
                        local.get 3
                        i32.shl
                        local.tee 9
                        i32.and
                        br_if 0 (;@10;)
                        local.get 5
                        local.get 7
                        i32.store
                        i32.const 0
                        local.get 0
                        local.get 9
                        i32.or
                        i32.store offset=1055532
                        local.get 7
                        local.get 5
                        i32.store offset=24
                        local.get 7
                        local.get 7
                        i32.store offset=8
                        local.get 7
                        local.get 7
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
                      local.get 5
                      i32.load
                      local.set 0
                      loop ;; label = @10
                        local.get 0
                        local.tee 5
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
                        local.get 5
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
                      local.get 7
                      i32.store
                      local.get 7
                      local.get 5
                      i32.store offset=24
                      local.get 7
                      local.get 7
                      i32.store offset=12
                      local.get 7
                      local.get 7
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
                    local.get 6
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
                    local.get 5
                    i32.const 55
                    local.get 5
                    i32.sub
                    i32.const 15
                    i32.and
                    i32.const 0
                    local.get 5
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
                    i32.load offset=1056016
                    i32.store offset=1055556
                    i32.const 0
                    local.get 4
                    i32.store offset=1055540
                    i32.const 0
                    local.get 2
                    i32.store offset=1055552
                    local.get 9
                    i32.const 16
                    i32.add
                    i32.const 0
                    i64.load offset=1055984 align=4
                    i64.store align=4
                    local.get 9
                    i32.const 0
                    i64.load offset=1055976 align=4
                    i64.store offset=8 align=4
                    i32.const 0
                    local.get 9
                    i32.const 8
                    i32.add
                    i32.store offset=1055984
                    i32.const 0
                    local.get 6
                    i32.store offset=1055980
                    i32.const 0
                    local.get 0
                    i32.store offset=1055976
                    i32.const 0
                    i32.const 0
                    i32.store offset=1055988
                    local.get 9
                    i32.const 36
                    i32.add
                    local.set 4
                    loop ;; label = @9
                      local.get 4
                      i32.const 7
                      i32.store
                      local.get 4
                      i32.const 4
                      i32.add
                      local.tee 4
                      local.get 5
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
                    block ;; label = @9
                      local.get 0
                      i32.const 255
                      i32.gt_u
                      br_if 0 (;@9;)
                      local.get 0
                      i32.const -8
                      i32.and
                      i32.const 1055568
                      i32.add
                      local.set 4
                      block ;; label = @10
                        block ;; label = @11
                          i32.const 0
                          i32.load offset=1055528
                          local.tee 5
                          i32.const 1
                          local.get 0
                          i32.const 3
                          i32.shr_u
                          i32.shl
                          local.tee 0
                          i32.and
                          br_if 0 (;@11;)
                          i32.const 0
                          local.get 5
                          local.get 0
                          i32.or
                          i32.store offset=1055528
                          local.get 4
                          local.set 5
                          br 1 (;@10;)
                        end
                        local.get 4
                        i32.load offset=8
                        local.set 5
                      end
                      local.get 5
                      local.get 3
                      i32.store offset=12
                      local.get 4
                      local.get 3
                      i32.store offset=8
                      local.get 3
                      local.get 4
                      i32.store offset=12
                      local.get 3
                      local.get 5
                      i32.store offset=8
                      br 4 (;@5;)
                    end
                    i32.const 31
                    local.set 4
                    block ;; label = @9
                      local.get 0
                      i32.const 16777215
                      i32.gt_u
                      br_if 0 (;@9;)
                      local.get 0
                      i32.const 38
                      local.get 0
                      i32.const 8
                      i32.shr_u
                      i32.clz
                      local.tee 4
                      i32.sub
                      i32.shr_u
                      i32.const 1
                      i32.and
                      local.get 4
                      i32.const 1
                      i32.shl
                      i32.sub
                      i32.const 62
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
                    i32.const 1055832
                    i32.add
                    local.set 5
                    block ;; label = @9
                      i32.const 0
                      i32.load offset=1055532
                      local.tee 9
                      i32.const 1
                      local.get 4
                      i32.shl
                      local.tee 6
                      i32.and
                      br_if 0 (;@9;)
                      local.get 5
                      local.get 3
                      i32.store
                      i32.const 0
                      local.get 9
                      local.get 6
                      i32.or
                      i32.store offset=1055532
                      local.get 3
                      local.get 5
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
                    local.get 5
                    i32.load
                    local.set 9
                    loop ;; label = @9
                      local.get 9
                      local.tee 5
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
                      local.get 5
                      local.get 9
                      i32.const 4
                      i32.and
                      i32.add
                      i32.const 16
                      i32.add
                      local.tee 6
                      i32.load
                      local.tee 9
                      br_if 0 (;@9;)
                    end
                    local.get 6
                    local.get 3
                    i32.store
                    local.get 3
                    local.get 5
                    i32.store offset=24
                    local.get 3
                    local.get 3
                    i32.store offset=12
                    local.get 3
                    local.get 3
                    i32.store offset=8
                    br 3 (;@5;)
                  end
                  local.get 5
                  i32.load offset=8
                  local.tee 4
                  local.get 7
                  i32.store offset=12
                  local.get 5
                  local.get 7
                  i32.store offset=8
                  local.get 7
                  i32.const 0
                  i32.store offset=24
                  local.get 7
                  local.get 5
                  i32.store offset=12
                  local.get 7
                  local.get 4
                  i32.store offset=8
                end
                local.get 2
                i32.const 8
                i32.add
                local.set 4
                br 5 (;@1;)
              end
              local.get 5
              i32.load offset=8
              local.tee 4
              local.get 3
              i32.store offset=12
              local.get 5
              local.get 3
              i32.store offset=8
              local.get 3
              i32.const 0
              i32.store offset=24
              local.get 3
              local.get 5
              i32.store offset=12
              local.get 3
              local.get 4
              i32.store offset=8
            end
            i32.const 0
            i32.load offset=1055540
            local.tee 4
            local.get 7
            i32.le_u
            br_if 0 (;@4;)
            i32.const 0
            i32.load offset=1055552
            local.tee 3
            local.get 7
            i32.add
            local.tee 5
            local.get 4
            local.get 7
            i32.sub
            local.tee 4
            i32.const 1
            i32.or
            i32.store offset=4
            i32.const 0
            local.get 4
            i32.store offset=1055540
            i32.const 0
            local.get 5
            i32.store offset=1055552
            local.get 3
            local.get 7
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
          i32.store offset=1056024
          br 2 (;@1;)
        end
        block ;; label = @3
          local.get 2
          i32.eqz
          br_if 0 (;@3;)
          block ;; label = @4
            block ;; label = @5
              local.get 9
              local.get 9
              i32.load offset=28
              local.tee 5
              i32.const 2
              i32.shl
              i32.const 1055832
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
              local.get 5
              i32.rotl
              i32.and
              local.tee 10
              i32.store offset=1055532
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
          block ;; label = @4
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
        block ;; label = @3
          block ;; label = @4
            local.get 3
            i32.const 15
            i32.gt_u
            br_if 0 (;@4;)
            local.get 9
            local.get 3
            local.get 7
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
          local.get 7
          i32.add
          local.tee 0
          local.get 3
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 9
          local.get 7
          i32.const 3
          i32.or
          i32.store offset=4
          local.get 0
          local.get 3
          i32.add
          local.get 3
          i32.store
          block ;; label = @4
            local.get 3
            i32.const 255
            i32.gt_u
            br_if 0 (;@4;)
            local.get 3
            i32.const -8
            i32.and
            i32.const 1055568
            i32.add
            local.set 4
            block ;; label = @5
              block ;; label = @6
                i32.const 0
                i32.load offset=1055528
                local.tee 5
                i32.const 1
                local.get 3
                i32.const 3
                i32.shr_u
                i32.shl
                local.tee 3
                i32.and
                br_if 0 (;@6;)
                i32.const 0
                local.get 5
                local.get 3
                i32.or
                i32.store offset=1055528
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
          block ;; label = @4
            local.get 3
            i32.const 16777215
            i32.gt_u
            br_if 0 (;@4;)
            local.get 3
            i32.const 38
            local.get 3
            i32.const 8
            i32.shr_u
            i32.clz
            local.tee 4
            i32.sub
            i32.shr_u
            i32.const 1
            i32.and
            local.get 4
            i32.const 1
            i32.shl
            i32.sub
            i32.const 62
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
          i32.const 1055832
          i32.add
          local.set 5
          block ;; label = @4
            local.get 10
            i32.const 1
            local.get 4
            i32.shl
            local.tee 7
            i32.and
            br_if 0 (;@4;)
            local.get 5
            local.get 0
            i32.store
            i32.const 0
            local.get 10
            local.get 7
            i32.or
            i32.store offset=1055532
            local.get 0
            local.get 5
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
          local.get 5
          i32.load
          local.set 7
          block ;; label = @4
            loop ;; label = @5
              local.get 7
              local.tee 5
              i32.load offset=4
              i32.const -8
              i32.and
              local.get 3
              i32.eq
              br_if 1 (;@4;)
              local.get 4
              i32.const 29
              i32.shr_u
              local.set 7
              local.get 4
              i32.const 1
              i32.shl
              local.set 4
              local.get 5
              local.get 7
              i32.const 4
              i32.and
              i32.add
              i32.const 16
              i32.add
              local.tee 6
              i32.load
              local.tee 7
              br_if 0 (;@5;)
            end
            local.get 6
            local.get 0
            i32.store
            local.get 0
            local.get 5
            i32.store offset=24
            local.get 0
            local.get 0
            i32.store offset=12
            local.get 0
            local.get 0
            i32.store offset=8
            br 1 (;@3;)
          end
          local.get 5
          i32.load offset=8
          local.tee 4
          local.get 0
          i32.store offset=12
          local.get 5
          local.get 0
          i32.store offset=8
          local.get 0
          i32.const 0
          i32.store offset=24
          local.get 0
          local.get 5
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
      block ;; label = @2
        local.get 11
        i32.eqz
        br_if 0 (;@2;)
        block ;; label = @3
          block ;; label = @4
            local.get 0
            local.get 0
            i32.load offset=28
            local.tee 5
            i32.const 2
            i32.shl
            i32.const 1055832
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
            local.get 5
            i32.rotl
            i32.and
            i32.store offset=1055532
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
        block ;; label = @3
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
      block ;; label = @2
        block ;; label = @3
          local.get 3
          i32.const 15
          i32.gt_u
          br_if 0 (;@3;)
          local.get 0
          local.get 3
          local.get 7
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
        local.get 7
        i32.add
        local.tee 5
        local.get 3
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 0
        local.get 7
        i32.const 3
        i32.or
        i32.store offset=4
        local.get 5
        local.get 3
        i32.add
        local.get 3
        i32.store
        block ;; label = @3
          local.get 8
          i32.eqz
          br_if 0 (;@3;)
          local.get 8
          i32.const -8
          i32.and
          i32.const 1055568
          i32.add
          local.set 7
          i32.const 0
          i32.load offset=1055548
          local.set 4
          block ;; label = @4
            block ;; label = @5
              i32.const 1
              local.get 8
              i32.const 3
              i32.shr_u
              i32.shl
              local.tee 9
              local.get 6
              i32.and
              br_if 0 (;@5;)
              i32.const 0
              local.get 9
              local.get 6
              i32.or
              i32.store offset=1055528
              local.get 7
              local.set 9
              br 1 (;@4;)
            end
            local.get 7
            i32.load offset=8
            local.set 9
          end
          local.get 9
          local.get 4
          i32.store offset=12
          local.get 7
          local.get 4
          i32.store offset=8
          local.get 4
          local.get 7
          i32.store offset=12
          local.get 4
          local.get 9
          i32.store offset=8
        end
        i32.const 0
        local.get 5
        i32.store offset=1055548
        i32.const 0
        local.get 3
        i32.store offset=1055536
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
    local.get 4
  )
  (func $free (;104;) (type 0) (param i32)
    local.get 0
    call $dlfree
  )
  (func $dlfree (;105;) (type 0) (param i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    block ;; label = @1
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
      block ;; label = @2
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
        i32.load offset=1055544
        local.tee 4
        i32.lt_u
        br_if 1 (;@1;)
        local.get 2
        local.get 0
        i32.add
        local.set 0
        block ;; label = @3
          local.get 1
          i32.const 0
          i32.load offset=1055548
          i32.eq
          br_if 0 (;@3;)
          block ;; label = @4
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
            i32.const 1055568
            i32.add
            local.tee 6
            i32.eq
            drop
            block ;; label = @5
              local.get 1
              i32.load offset=12
              local.tee 2
              local.get 4
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1055528
              i32.const -2
              local.get 5
              i32.rotl
              i32.and
              i32.store offset=1055528
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
          block ;; label = @4
            block ;; label = @5
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
            block ;; label = @5
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
            loop ;; label = @5
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
          block ;; label = @4
            block ;; label = @5
              local.get 1
              local.get 1
              i32.load offset=28
              local.tee 4
              i32.const 2
              i32.shl
              i32.const 1055832
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
              i32.load offset=1055532
              i32.const -2
              local.get 4
              i32.rotl
              i32.and
              i32.store offset=1055532
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
          block ;; label = @4
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
        i32.store offset=1055536
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
      block ;; label = @2
        block ;; label = @3
          local.get 2
          i32.const 2
          i32.and
          br_if 0 (;@3;)
          block ;; label = @4
            local.get 3
            i32.const 0
            i32.load offset=1055552
            i32.ne
            br_if 0 (;@4;)
            i32.const 0
            local.get 1
            i32.store offset=1055552
            i32.const 0
            i32.const 0
            i32.load offset=1055540
            local.get 0
            i32.add
            local.tee 0
            i32.store offset=1055540
            local.get 1
            local.get 0
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 1
            i32.const 0
            i32.load offset=1055548
            i32.ne
            br_if 3 (;@1;)
            i32.const 0
            i32.const 0
            i32.store offset=1055536
            i32.const 0
            i32.const 0
            i32.store offset=1055548
            return
          end
          block ;; label = @4
            local.get 3
            i32.const 0
            i32.load offset=1055548
            i32.ne
            br_if 0 (;@4;)
            i32.const 0
            local.get 1
            i32.store offset=1055548
            i32.const 0
            i32.const 0
            i32.load offset=1055536
            local.get 0
            i32.add
            local.tee 0
            i32.store offset=1055536
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
          block ;; label = @4
            block ;; label = @5
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
              i32.const 1055568
              i32.add
              local.tee 6
              i32.eq
              drop
              block ;; label = @6
                local.get 3
                i32.load offset=12
                local.tee 2
                local.get 4
                i32.ne
                br_if 0 (;@6;)
                i32.const 0
                i32.const 0
                i32.load offset=1055528
                i32.const -2
                local.get 5
                i32.rotl
                i32.and
                i32.store offset=1055528
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
            block ;; label = @5
              block ;; label = @6
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
                i32.load offset=1055544
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
              block ;; label = @6
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
              loop ;; label = @6
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
            block ;; label = @5
              block ;; label = @6
                local.get 3
                local.get 3
                i32.load offset=28
                local.tee 4
                i32.const 2
                i32.shl
                i32.const 1055832
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
                i32.load offset=1055532
                i32.const -2
                local.get 4
                i32.rotl
                i32.and
                i32.store offset=1055532
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
            block ;; label = @5
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
          i32.load offset=1055548
          i32.ne
          br_if 1 (;@2;)
          i32.const 0
          local.get 0
          i32.store offset=1055536
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
      block ;; label = @2
        local.get 0
        i32.const 255
        i32.gt_u
        br_if 0 (;@2;)
        local.get 0
        i32.const -8
        i32.and
        i32.const 1055568
        i32.add
        local.set 2
        block ;; label = @3
          block ;; label = @4
            i32.const 0
            i32.load offset=1055528
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
            i32.store offset=1055528
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
      block ;; label = @2
        local.get 0
        i32.const 16777215
        i32.gt_u
        br_if 0 (;@2;)
        local.get 0
        i32.const 38
        local.get 0
        i32.const 8
        i32.shr_u
        i32.clz
        local.tee 2
        i32.sub
        i32.shr_u
        i32.const 1
        i32.and
        local.get 2
        i32.const 1
        i32.shl
        i32.sub
        i32.const 62
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
      i32.const 1055832
      i32.add
      local.set 4
      block ;; label = @2
        block ;; label = @3
          i32.const 0
          i32.load offset=1055532
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
          i32.store offset=1055532
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
        block ;; label = @3
          loop ;; label = @4
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
      i32.load offset=1055560
      i32.const -1
      i32.add
      local.tee 1
      i32.const -1
      local.get 1
      select
      i32.store offset=1055560
    end
  )
  (func $calloc (;106;) (type 2) (param i32 i32) (result i32)
    (local i32 i64)
    block ;; label = @1
      block ;; label = @2
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
    block ;; label = @1
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
    local.get 0
  )
  (func $realloc (;107;) (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    block ;; label = @1
      local.get 0
      br_if 0 (;@1;)
      local.get 1
      call $dlmalloc
      return
    end
    block ;; label = @1
      local.get 1
      i32.const -64
      i32.lt_u
      br_if 0 (;@1;)
      i32.const 0
      i32.const 48
      i32.store offset=1056024
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
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
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
          i32.load offset=1056008
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
        block ;; label = @3
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
        block ;; label = @3
          local.get 7
          i32.const 0
          i32.load offset=1055552
          i32.ne
          br_if 0 (;@3;)
          i32.const 0
          i32.load offset=1055540
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
          i32.store offset=1055552
          i32.const 0
          local.get 5
          local.get 2
          i32.sub
          local.tee 2
          i32.store offset=1055540
          local.get 1
          local.get 2
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 0
          return
        end
        block ;; label = @3
          local.get 7
          i32.const 0
          i32.load offset=1055548
          i32.ne
          br_if 0 (;@3;)
          i32.const 0
          i32.load offset=1055536
          local.get 5
          i32.add
          local.tee 5
          local.get 2
          i32.lt_u
          br_if 1 (;@2;)
          block ;; label = @4
            block ;; label = @5
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
          i32.store offset=1055548
          i32.const 0
          local.get 1
          i32.store offset=1055536
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
        block ;; label = @3
          block ;; label = @4
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
            i32.const 1055568
            i32.add
            local.tee 8
            i32.eq
            drop
            block ;; label = @5
              local.get 7
              i32.load offset=12
              local.tee 5
              local.get 1
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1055528
              i32.const -2
              local.get 11
              i32.rotl
              i32.and
              i32.store offset=1055528
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
          block ;; label = @4
            block ;; label = @5
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
              i32.load offset=1055544
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
            block ;; label = @5
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
            loop ;; label = @5
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
          block ;; label = @4
            block ;; label = @5
              local.get 7
              local.get 7
              i32.load offset=28
              local.tee 5
              i32.const 2
              i32.shl
              i32.const 1055832
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
              i32.load offset=1055532
              i32.const -2
              local.get 5
              i32.rotl
              i32.and
              i32.store offset=1055532
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
          block ;; label = @4
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
        block ;; label = @3
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
      block ;; label = @2
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
    local.get 0
  )
  (func $dispose_chunk (;108;) (type 1) (param i32 i32)
    (local i32 i32 i32 i32 i32 i32)
    local.get 0
    local.get 1
    i32.add
    local.set 2
    block ;; label = @1
      block ;; label = @2
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
        block ;; label = @3
          block ;; label = @4
            local.get 0
            local.get 3
            i32.sub
            local.tee 0
            i32.const 0
            i32.load offset=1055548
            i32.eq
            br_if 0 (;@4;)
            block ;; label = @5
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
              i32.const 1055568
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
              i32.load offset=1055528
              i32.const -2
              local.get 5
              i32.rotl
              i32.and
              i32.store offset=1055528
              br 3 (;@2;)
            end
            local.get 0
            i32.load offset=24
            local.set 7
            block ;; label = @5
              block ;; label = @6
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
                i32.load offset=1055544
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
              block ;; label = @6
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
              loop ;; label = @6
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
            block ;; label = @5
              block ;; label = @6
                local.get 0
                local.get 0
                i32.load offset=28
                local.tee 4
                i32.const 2
                i32.shl
                i32.const 1055832
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
                i32.load offset=1055532
                i32.const -2
                local.get 4
                i32.rotl
                i32.and
                i32.store offset=1055532
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
            block ;; label = @5
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
          i32.store offset=1055536
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
      block ;; label = @2
        block ;; label = @3
          local.get 2
          i32.load offset=4
          local.tee 3
          i32.const 2
          i32.and
          br_if 0 (;@3;)
          block ;; label = @4
            local.get 2
            i32.const 0
            i32.load offset=1055552
            i32.ne
            br_if 0 (;@4;)
            i32.const 0
            local.get 0
            i32.store offset=1055552
            i32.const 0
            i32.const 0
            i32.load offset=1055540
            local.get 1
            i32.add
            local.tee 1
            i32.store offset=1055540
            local.get 0
            local.get 1
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 0
            i32.const 0
            i32.load offset=1055548
            i32.ne
            br_if 3 (;@1;)
            i32.const 0
            i32.const 0
            i32.store offset=1055536
            i32.const 0
            i32.const 0
            i32.store offset=1055548
            return
          end
          block ;; label = @4
            local.get 2
            i32.const 0
            i32.load offset=1055548
            i32.ne
            br_if 0 (;@4;)
            i32.const 0
            local.get 0
            i32.store offset=1055548
            i32.const 0
            i32.const 0
            i32.load offset=1055536
            local.get 1
            i32.add
            local.tee 1
            i32.store offset=1055536
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
          block ;; label = @4
            block ;; label = @5
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
              i32.const 1055568
              i32.add
              local.tee 6
              i32.eq
              drop
              block ;; label = @6
                local.get 2
                i32.load offset=12
                local.tee 3
                local.get 4
                i32.ne
                br_if 0 (;@6;)
                i32.const 0
                i32.const 0
                i32.load offset=1055528
                i32.const -2
                local.get 5
                i32.rotl
                i32.and
                i32.store offset=1055528
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
            block ;; label = @5
              block ;; label = @6
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
                i32.load offset=1055544
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
              block ;; label = @6
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
              loop ;; label = @6
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
            block ;; label = @5
              block ;; label = @6
                local.get 2
                local.get 2
                i32.load offset=28
                local.tee 4
                i32.const 2
                i32.shl
                i32.const 1055832
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
                i32.load offset=1055532
                i32.const -2
                local.get 4
                i32.rotl
                i32.and
                i32.store offset=1055532
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
            block ;; label = @5
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
          i32.load offset=1055548
          i32.ne
          br_if 1 (;@2;)
          i32.const 0
          local.get 1
          i32.store offset=1055536
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
      block ;; label = @2
        local.get 1
        i32.const 255
        i32.gt_u
        br_if 0 (;@2;)
        local.get 1
        i32.const -8
        i32.and
        i32.const 1055568
        i32.add
        local.set 3
        block ;; label = @3
          block ;; label = @4
            i32.const 0
            i32.load offset=1055528
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
            i32.store offset=1055528
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
      block ;; label = @2
        local.get 1
        i32.const 16777215
        i32.gt_u
        br_if 0 (;@2;)
        local.get 1
        i32.const 38
        local.get 1
        i32.const 8
        i32.shr_u
        i32.clz
        local.tee 3
        i32.sub
        i32.shr_u
        i32.const 1
        i32.and
        local.get 3
        i32.const 1
        i32.shl
        i32.sub
        i32.const 62
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
      i32.const 1055832
      i32.add
      local.set 4
      block ;; label = @2
        i32.const 0
        i32.load offset=1055532
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
        i32.store offset=1055532
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
      block ;; label = @2
        loop ;; label = @3
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
    end
  )
  (func $internal_memalign (;109;) (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32)
    block ;; label = @1
      block ;; label = @2
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
      loop ;; label = @2
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
    block ;; label = @1
      i32.const -64
      local.get 0
      i32.sub
      local.get 1
      i32.gt_u
      br_if 0 (;@1;)
      i32.const 0
      i32.const 48
      i32.store offset=1056024
      i32.const 0
      return
    end
    block ;; label = @1
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
    block ;; label = @1
      block ;; label = @2
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
      block ;; label = @2
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
    block ;; label = @1
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
    i32.add
  )
  (func $aligned_alloc (;110;) (type 2) (param i32 i32) (result i32)
    block ;; label = @1
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
    call $internal_memalign
  )
  (func $_Exit (;111;) (type 0) (param i32)
    local.get 0
    call $__wasi_proc_exit
    unreachable
  )
  (func $__wasilibc_ensure_environ (;112;) (type 6)
    block ;; label = @1
      i32.const 0
      i32.load offset=1055416
      i32.const -1
      i32.ne
      br_if 0 (;@1;)
      call $__wasilibc_initialize_environ
    end
  )
  (func $__wasilibc_initialize_environ (;113;) (type 6)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 0
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        local.get 0
        i32.const 12
        i32.add
        local.get 0
        i32.const 8
        i32.add
        call $__wasi_environ_sizes_get
        br_if 0 (;@2;)
        block ;; label = @3
          local.get 0
          i32.load offset=12
          local.tee 1
          br_if 0 (;@3;)
          i32.const 1056028
          local.set 1
          br 2 (;@1;)
        end
        block ;; label = @3
          block ;; label = @4
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
    i32.store offset=1055416
    local.get 0
    i32.const 16
    i32.add
    global.set $__stack_pointer
  )
  (func $__wasi_environ_get (;114;) (type 2) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call $__imported_wasi_snapshot_preview1_environ_get
    i32.const 65535
    i32.and
  )
  (func $__wasi_environ_sizes_get (;115;) (type 2) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call $__imported_wasi_snapshot_preview1_environ_sizes_get
    i32.const 65535
    i32.and
  )
  (func $__wasi_proc_exit (;116;) (type 0) (param i32)
    local.get 0
    call $__imported_wasi_snapshot_preview1_proc_exit
    unreachable
  )
  (func $abort (;117;) (type 6)
    unreachable
    unreachable
  )
  (func $getcwd (;118;) (type 2) (param i32 i32) (result i32)
    (local i32)
    i32.const 0
    i32.load offset=1055420
    local.set 2
    block ;; label = @1
      block ;; label = @2
        local.get 0
        br_if 0 (;@2;)
        local.get 2
        call $strdup
        local.tee 0
        br_if 1 (;@1;)
        i32.const 0
        i32.const 48
        i32.store offset=1056024
        i32.const 0
        return
      end
      block ;; label = @2
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
      i32.store offset=1056024
    end
    local.get 0
  )
  (func $sbrk (;119;) (type 9) (param i32) (result i32)
    block ;; label = @1
      local.get 0
      br_if 0 (;@1;)
      memory.size
      i32.const 16
      i32.shl
      return
    end
    block ;; label = @1
      local.get 0
      i32.const 65535
      i32.and
      br_if 0 (;@1;)
      local.get 0
      i32.const -1
      i32.le_s
      br_if 0 (;@1;)
      block ;; label = @2
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
        i32.store offset=1056024
        i32.const -1
        return
      end
      local.get 0
      i32.const 16
      i32.shl
      return
    end
    call $abort
    unreachable
  )
  (func $getenv (;120;) (type 9) (param i32) (result i32)
    (local i32 i32 i32 i32)
    call $__wasilibc_ensure_environ
    block ;; label = @1
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
    block ;; label = @1
      local.get 0
      local.get 1
      local.get 0
      i32.sub
      local.tee 3
      i32.add
      i32.load8_u
      br_if 0 (;@1;)
      i32.const 0
      i32.load offset=1055416
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
      block ;; label = @2
        loop ;; label = @3
          block ;; label = @4
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
    local.get 2
  )
  (func $dummy (;121;) (type 6))
  (func $__wasm_call_dtors (;122;) (type 6)
    call $dummy
    call $dummy
  )
  (func $memcmp (;123;) (type 4) (param i32 i32 i32) (result i32)
    (local i32 i32 i32)
    i32.const 0
    local.set 3
    block ;; label = @1
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      block ;; label = @2
        loop ;; label = @3
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
    local.get 3
  )
  (func $memcpy (;124;) (type 4) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32)
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
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
    block ;; label = @1
      block ;; label = @2
        local.get 4
        i32.const 3
        i32.and
        local.tee 2
        br_if 0 (;@2;)
        block ;; label = @3
          block ;; label = @4
            local.get 3
            i32.const 16
            i32.ge_u
            br_if 0 (;@4;)
            local.get 3
            local.set 2
            br 1 (;@3;)
          end
          block ;; label = @4
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
          loop ;; label = @4
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
        block ;; label = @3
          local.get 2
          i32.const 8
          i32.lt_u
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
        block ;; label = @3
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
        block ;; label = @3
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
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                local.get 3
                i32.const 32
                i32.lt_u
                br_if 0 (;@6;)
                block ;; label = @7
                  block ;; label = @8
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
              block ;; label = @6
                block ;; label = @7
                  local.get 3
                  i32.const 16
                  i32.ge_u
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
      block ;; label = @2
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
      block ;; label = @2
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
    local.get 0
  )
  (func $memmove (;125;) (type 4) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32)
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
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
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            local.get 0
            local.get 1
            i32.ge_u
            br_if 0 (;@4;)
            block ;; label = @5
              local.get 4
              i32.eqz
              br_if 0 (;@5;)
              local.get 2
              local.set 5
              local.get 0
              local.set 3
              br 3 (;@2;)
            end
            block ;; label = @5
              local.get 0
              i32.const 3
              i32.and
              br_if 0 (;@5;)
              local.get 2
              local.set 5
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
            local.set 5
            block ;; label = @5
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
            local.get 5
            i32.eqz
            br_if 3 (;@1;)
            local.get 0
            local.get 1
            i32.load8_u offset=1
            i32.store8 offset=1
            local.get 2
            i32.const -2
            i32.add
            local.set 5
            block ;; label = @5
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
            local.get 5
            i32.eqz
            br_if 3 (;@1;)
            local.get 0
            local.get 1
            i32.load8_u offset=2
            i32.store8 offset=2
            local.get 2
            i32.const -3
            i32.add
            local.set 5
            block ;; label = @5
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
            local.get 5
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
            local.set 5
            br 1 (;@3;)
          end
          block ;; label = @4
            local.get 4
            br_if 0 (;@4;)
            block ;; label = @5
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
              block ;; label = @6
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
              block ;; label = @6
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
              block ;; label = @6
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
            block ;; label = @5
              local.get 2
              i32.const -4
              i32.add
              local.tee 6
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
              local.set 5
              loop ;; label = @6
                local.get 5
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
            local.get 6
            i32.const 12
            i32.lt_u
            br_if 0 (;@4;)
            local.get 1
            i32.const -16
            i32.add
            local.set 5
            local.get 0
            i32.const -16
            i32.add
            local.set 6
            loop ;; label = @5
              local.get 6
              local.get 2
              i32.add
              local.tee 3
              i32.const 12
              i32.add
              local.get 5
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
          local.set 3
          block ;; label = @4
            local.get 2
            i32.const 3
            i32.and
            local.tee 4
            i32.eqz
            br_if 0 (;@4;)
            local.get 1
            i32.const -1
            i32.add
            local.set 5
            local.get 0
            i32.const -1
            i32.add
            local.set 6
            local.get 2
            local.set 3
            loop ;; label = @5
              local.get 6
              local.get 3
              i32.add
              local.get 5
              local.get 3
              i32.add
              i32.load8_u
              i32.store8
              local.get 3
              i32.const -1
              i32.add
              local.set 3
              local.get 4
              i32.const -1
              i32.add
              local.tee 4
              br_if 0 (;@5;)
            end
          end
          local.get 2
          i32.const 4
          i32.lt_u
          br_if 2 (;@1;)
          local.get 1
          i32.const -4
          i32.add
          local.set 4
          local.get 0
          i32.const -4
          i32.add
          local.set 5
          loop ;; label = @4
            local.get 5
            local.get 3
            i32.add
            local.tee 1
            i32.const 3
            i32.add
            local.get 4
            local.get 3
            i32.add
            local.tee 2
            i32.const 3
            i32.add
            i32.load8_u
            i32.store8
            local.get 1
            i32.const 2
            i32.add
            local.get 2
            i32.const 2
            i32.add
            i32.load8_u
            i32.store8
            local.get 1
            i32.const 1
            i32.add
            local.get 2
            i32.const 1
            i32.add
            i32.load8_u
            i32.store8
            local.get 1
            local.get 2
            i32.load8_u
            i32.store8
            local.get 3
            i32.const -4
            i32.add
            local.tee 3
            br_if 0 (;@4;)
            br 3 (;@1;)
          end
        end
        local.get 5
        i32.const 4
        i32.lt_u
        br_if 0 (;@2;)
        block ;; label = @3
          local.get 5
          i32.const -4
          i32.add
          local.tee 4
          i32.const 2
          i32.shr_u
          i32.const 1
          i32.add
          i32.const 7
          i32.and
          local.tee 2
          i32.eqz
          br_if 0 (;@3;)
          loop ;; label = @4
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
            local.get 5
            i32.const -4
            i32.add
            local.set 5
            local.get 2
            i32.const -1
            i32.add
            local.tee 2
            br_if 0 (;@4;)
          end
        end
        local.get 4
        i32.const 28
        i32.lt_u
        br_if 0 (;@2;)
        loop ;; label = @3
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
          local.get 5
          i32.const -32
          i32.add
          local.tee 5
          i32.const 3
          i32.gt_u
          br_if 0 (;@3;)
        end
      end
      local.get 5
      i32.eqz
      br_if 0 (;@1;)
      block ;; label = @2
        block ;; label = @3
          local.get 5
          i32.const 7
          i32.and
          local.tee 4
          br_if 0 (;@3;)
          local.get 5
          local.set 2
          br 1 (;@2;)
        end
        local.get 5
        local.set 2
        loop ;; label = @3
          local.get 3
          local.get 1
          i32.load8_u
          i32.store8
          local.get 2
          i32.const -1
          i32.add
          local.set 2
          local.get 3
          i32.const 1
          i32.add
          local.set 3
          local.get 1
          i32.const 1
          i32.add
          local.set 1
          local.get 4
          i32.const -1
          i32.add
          local.tee 4
          br_if 0 (;@3;)
        end
      end
      local.get 5
      i32.const 8
      i32.lt_u
      br_if 0 (;@1;)
      loop ;; label = @2
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
        local.get 2
        i32.const -8
        i32.add
        local.tee 2
        br_if 0 (;@2;)
      end
    end
    local.get 0
  )
  (func $memset (;126;) (type 4) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i64)
    block ;; label = @1
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
    block ;; label = @1
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
      loop ;; label = @2
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
    local.get 0
  )
  (func $__strchrnul (;127;) (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32)
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
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
            block ;; label = @5
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
        block ;; label = @3
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
        block ;; label = @3
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
        block ;; label = @3
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
      block ;; label = @2
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
        loop ;; label = @3
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
      loop ;; label = @2
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
    local.get 3
  )
  (func $__stpcpy (;128;) (type 2) (param i32 i32) (result i32)
    (local i32 i32)
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
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
        block ;; label = @3
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
          block ;; label = @4
            local.get 2
            br_if 0 (;@4;)
            local.get 0
            return
          end
          local.get 0
          i32.const 1
          i32.add
          local.set 2
          block ;; label = @4
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
          block ;; label = @4
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
          block ;; label = @4
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
        loop ;; label = @3
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
      block ;; label = @2
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
      loop ;; label = @2
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
    local.get 2
  )
  (func $strcpy (;129;) (type 2) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call $__stpcpy
    drop
    local.get 0
  )
  (func $strdup (;130;) (type 9) (param i32) (result i32)
    (local i32 i32)
    block ;; label = @1
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
    local.get 2
  )
  (func $#func131<dummy> (@name "dummy") (;131;) (type 2) (param i32 i32) (result i32)
    local.get 0
  )
  (func $__lctrans (;132;) (type 2) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call $#func131<dummy>
  )
  (func $strerror (;133;) (type 9) (param i32) (result i32)
    (local i32)
    block ;; label = @1
      i32.const 0
      i32.load offset=1056056
      local.tee 1
      br_if 0 (;@1;)
      i32.const 1056032
      local.set 1
      i32.const 0
      i32.const 1056032
      i32.store offset=1056056
    end
    i32.const 0
    local.get 0
    local.get 0
    i32.const 76
    i32.gt_u
    select
    i32.const 1
    i32.shl
    i32.const 1053984
    i32.add
    i32.load16_u
    i32.const 1052426
    i32.add
    local.get 1
    i32.load offset=20
    call $__lctrans
  )
  (func $strerror_r (;134;) (type 4) (param i32 i32 i32) (result i32)
    (local i32)
    block ;; label = @1
      block ;; label = @2
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
    local.get 3
  )
  (func $strlen (;135;) (type 9) (param i32) (result i32)
    (local i32 i32)
    local.get 0
    local.set 1
    block ;; label = @1
      block ;; label = @2
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
      loop ;; label = @2
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
      loop ;; label = @2
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
    i32.sub
  )
  (func $strncmp (;136;) (type 4) (param i32 i32 i32) (result i32)
    (local i32 i32 i32)
    block ;; label = @1
      local.get 2
      br_if 0 (;@1;)
      i32.const 0
      return
    end
    i32.const 0
    local.set 3
    block ;; label = @1
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
      loop ;; label = @2
        block ;; label = @3
          local.get 1
          i32.load8_u
          local.tee 5
          br_if 0 (;@3;)
          local.get 4
          local.set 3
          br 2 (;@1;)
        end
        block ;; label = @3
          local.get 2
          br_if 0 (;@3;)
          local.get 4
          local.set 3
          br 2 (;@1;)
        end
        block ;; label = @3
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
    i32.sub
  )
  (func $_ZN4core3ptr37drop_in_place$LT$core..fmt..Error$GT$17h9c4ba54b1b1f88d7E (;137;) (type 0) (param i32))
  (func $_ZN69_$LT$core..alloc..layout..LayoutError$u20$as$u20$core..fmt..Debug$GT$3fmt17hcd10cc24a0c59435E (;138;) (type 2) (param i32 i32) (result i32)
    local.get 1
    i32.const 1054140
    i32.const 11
    call $_ZN4core3fmt9Formatter9write_str17h8cfbd807d1bba582E
  )
  (func $_ZN5alloc7raw_vec11finish_grow17h3d670e4656a5cefbE (;139;) (type 7) (param i32 i32 i32 i32)
    block ;; label = @1
      block ;; label = @2
        local.get 1
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        i32.const -1
        i32.le_s
        br_if 1 (;@1;)
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                local.get 3
                i32.load offset=4
                i32.eqz
                br_if 0 (;@6;)
                block ;; label = @7
                  local.get 3
                  i32.const 8
                  i32.add
                  i32.load
                  local.tee 1
                  br_if 0 (;@7;)
                  block ;; label = @8
                    local.get 2
                    br_if 0 (;@8;)
                    i32.const 1
                    local.set 1
                    br 4 (;@4;)
                  end
                  i32.const 0
                  i32.load8_u offset=1055425
                  drop
                  local.get 2
                  i32.const 1
                  call $__rust_alloc
                  local.set 1
                  br 2 (;@5;)
                end
                local.get 3
                i32.load
                local.get 1
                i32.const 1
                local.get 2
                call $__rust_realloc
                local.set 1
                br 1 (;@5;)
              end
              block ;; label = @6
                local.get 2
                br_if 0 (;@6;)
                i32.const 1
                local.set 1
                br 2 (;@4;)
              end
              i32.const 0
              i32.load8_u offset=1055425
              drop
              local.get 2
              i32.const 1
              call $__rust_alloc
              local.set 1
            end
            local.get 1
            i32.eqz
            br_if 1 (;@3;)
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
          i32.store
          return
        end
        local.get 0
        i32.const 1
        i32.store offset=4
        local.get 0
        i32.const 8
        i32.add
        local.get 2
        i32.store
        local.get 0
        i32.const 1
        i32.store
        return
      end
      local.get 0
      i32.const 0
      i32.store offset=4
      local.get 0
      i32.const 8
      i32.add
      local.get 2
      i32.store
      local.get 0
      i32.const 1
      i32.store
      return
    end
    local.get 0
    i32.const 0
    i32.store offset=4
    local.get 0
    i32.const 1
    i32.store
  )
  (func $_ZN5alloc5alloc18handle_alloc_error17h76ec24bcd8f509d8E (;140;) (type 1) (param i32 i32)
    local.get 1
    local.get 0
    call $__rust_alloc_error_handler
    unreachable
  )
  (func $_ZN5alloc7raw_vec17capacity_overflow17h4ae6daed6ccaada6E (;141;) (type 6)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 0
    global.set $__stack_pointer
    local.get 0
    i32.const 20
    i32.add
    i64.const 0
    i64.store align=4
    local.get 0
    i32.const 1
    i32.store offset=12
    local.get 0
    i32.const 1054196
    i32.store offset=8
    local.get 0
    i32.const 1054140
    i32.store offset=16
    local.get 0
    i32.const 8
    i32.add
    i32.const 1054204
    call $_ZN4core9panicking9panic_fmt17h2298343e5f8a8826E
    unreachable
  )
  (func $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17h62e274fcd4fedb31E (;142;) (type 1) (param i32 i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
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
        block ;; label = @3
          block ;; label = @4
            local.get 3
            br_if 0 (;@4;)
            local.get 2
            i32.const 0
            i32.store offset=24
            br 1 (;@3;)
          end
          local.get 2
          local.get 3
          i32.store offset=28
          local.get 2
          i32.const 1
          i32.store offset=24
          local.get 2
          local.get 0
          i32.load offset=4
          i32.store offset=20
        end
        local.get 2
        i32.const 8
        i32.add
        local.get 4
        local.get 1
        local.get 2
        i32.const 20
        i32.add
        call $_ZN5alloc7raw_vec11finish_grow17h3d670e4656a5cefbE
        local.get 2
        i32.load offset=12
        local.set 3
        block ;; label = @3
          local.get 2
          i32.load offset=8
          br_if 0 (;@3;)
          local.get 0
          local.get 1
          i32.store
          local.get 0
          local.get 3
          i32.store offset=4
          br 2 (;@1;)
        end
        local.get 3
        i32.const -2147483647
        i32.eq
        br_if 1 (;@1;)
        local.get 3
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        local.get 2
        i32.const 16
        i32.add
        i32.load
        call $_ZN5alloc5alloc18handle_alloc_error17h76ec24bcd8f509d8E
        unreachable
      end
      call $_ZN5alloc7raw_vec17capacity_overflow17h4ae6daed6ccaada6E
      unreachable
    end
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
  )
  (func $_ZN72_$LT$$RF$str$u20$as$u20$alloc..ffi..c_str..CString..new..SpecNewImpl$GT$13spec_new_impl17hc178f76b63054b4aE (;143;) (type 3) (param i32 i32 i32)
    (local i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
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
              i32.const 0
              i32.load8_u offset=1055425
              drop
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
              block ;; label = @6
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
                call $_ZN4core5slice6memchr14memchr_aligned17hc6d2e1527adf972eE
                local.get 3
                i32.load offset=12
                local.set 7
                local.get 3
                i32.load offset=8
                local.set 5
                br 5 (;@1;)
              end
              block ;; label = @6
                local.get 2
                br_if 0 (;@6;)
                i32.const 0
                local.set 7
                i32.const 0
                local.set 5
                br 5 (;@1;)
              end
              block ;; label = @6
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
              block ;; label = @6
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
            i32.const 1054220
            i32.const 43
            i32.const 1054296
            call $_ZN4core9panicking5panic17h89e93701a34d6dc7E
            unreachable
          end
          call $_ZN5alloc7raw_vec17capacity_overflow17h4ae6daed6ccaada6E
          unreachable
        end
        i32.const 1
        local.get 4
        call $_ZN5alloc5alloc18handle_alloc_error17h76ec24bcd8f509d8E
        unreachable
      end
      local.get 2
      local.set 7
      i32.const 0
      local.set 5
    end
    block ;; label = @1
      block ;; label = @2
        local.get 5
        br_if 0 (;@2;)
        local.get 3
        local.get 2
        i32.store offset=28
        local.get 3
        local.get 6
        i32.store offset=24
        local.get 3
        local.get 4
        i32.store offset=20
        local.get 3
        local.get 3
        i32.const 20
        i32.add
        call $_ZN5alloc3ffi5c_str7CString19_from_vec_unchecked17h76f73ba3380ef61dE
        local.get 0
        local.get 3
        i64.load
        i64.store offset=4 align=4
        i32.const -2147483648
        local.set 4
        br 1 (;@1;)
      end
      local.get 0
      local.get 2
      i32.store offset=8
      local.get 0
      local.get 6
      i32.store offset=4
      local.get 0
      local.get 7
      i32.store offset=12
    end
    local.get 0
    local.get 4
    i32.store
    local.get 3
    i32.const 32
    i32.add
    global.set $__stack_pointer
  )
  (func $_ZN5alloc3ffi5c_str7CString19_from_vec_unchecked17h76f73ba3380ef61dE (;144;) (type 1) (param i32 i32)
    (local i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block ;; label = @1
      local.get 1
      i32.load
      local.tee 3
      local.get 1
      i32.load offset=8
      local.tee 4
      i32.sub
      br_if 0 (;@1;)
      block ;; label = @2
        local.get 4
        i32.const 1
        i32.add
        local.tee 5
        i32.eqz
        br_if 0 (;@2;)
        local.get 5
        i32.const -1
        i32.xor
        i32.const 31
        i32.shr_u
        local.set 6
        block ;; label = @3
          block ;; label = @4
            local.get 3
            br_if 0 (;@4;)
            local.get 2
            i32.const 0
            i32.store offset=24
            br 1 (;@3;)
          end
          local.get 2
          local.get 3
          i32.store offset=28
          local.get 2
          i32.const 1
          i32.store offset=24
          local.get 2
          local.get 1
          i32.load offset=4
          i32.store offset=20
        end
        local.get 2
        i32.const 8
        i32.add
        local.get 6
        local.get 5
        local.get 2
        i32.const 20
        i32.add
        call $_ZN5alloc7raw_vec11finish_grow17h3d670e4656a5cefbE
        local.get 2
        i32.load offset=12
        local.set 6
        block ;; label = @3
          local.get 2
          i32.load offset=8
          br_if 0 (;@3;)
          local.get 1
          local.get 5
          i32.store
          local.get 1
          local.get 6
          i32.store offset=4
          local.get 5
          local.set 3
          br 2 (;@1;)
        end
        local.get 6
        i32.const -2147483647
        i32.eq
        br_if 1 (;@1;)
        local.get 6
        i32.eqz
        br_if 0 (;@2;)
        local.get 6
        local.get 2
        i32.const 16
        i32.add
        i32.load
        call $_ZN5alloc5alloc18handle_alloc_error17h76ec24bcd8f509d8E
        unreachable
      end
      call $_ZN5alloc7raw_vec17capacity_overflow17h4ae6daed6ccaada6E
      unreachable
    end
    block ;; label = @1
      local.get 4
      local.get 3
      i32.ne
      br_if 0 (;@1;)
      local.get 1
      local.get 4
      call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17h62e274fcd4fedb31E
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
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 3
          local.get 5
          i32.gt_u
          br_if 0 (;@3;)
          local.get 1
          local.set 4
          br 1 (;@2;)
        end
        block ;; label = @3
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
    i32.const 1
    local.get 5
    call $_ZN5alloc5alloc18handle_alloc_error17h76ec24bcd8f509d8E
    unreachable
  )
  (func $_ZN5alloc4sync32arcinner_layout_for_value_layout17h3fb142d6d262ede9E (;145;) (type 3) (param i32 i32 i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    block ;; label = @1
      local.get 1
      i32.const 7
      i32.add
      i32.const 0
      local.get 1
      i32.sub
      i32.and
      local.tee 4
      local.get 4
      i32.const -8
      i32.add
      i32.lt_u
      br_if 0 (;@1;)
      local.get 4
      local.get 2
      i32.add
      local.tee 2
      local.get 4
      i32.lt_u
      br_if 0 (;@1;)
      local.get 2
      i32.const -2147483648
      local.get 1
      i32.const 4
      local.get 1
      i32.const 4
      i32.gt_u
      select
      local.tee 1
      i32.sub
      i32.gt_u
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      i32.store
      local.get 0
      local.get 1
      local.get 2
      i32.add
      i32.const -1
      i32.add
      i32.const 0
      local.get 1
      i32.sub
      i32.and
      i32.store offset=4
      local.get 3
      i32.const 16
      i32.add
      global.set $__stack_pointer
      return
    end
    i32.const 1054312
    i32.const 43
    local.get 3
    i32.const 15
    i32.add
    i32.const 1054356
    i32.const 1054400
    call $_ZN4core6result13unwrap_failed17h3354cd59a5a00fe5E
    unreachable
  )
  (func $_ZN4core3ops8function6FnOnce9call_once17hb10a2d73cc646a7dE (;146;) (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    drop
    loop (result i32) ;; label = @1
      br 0 (;@1;)
    end
  )
  (func $_ZN4core3ptr25drop_in_place$LT$char$GT$17h46384f1420881d52E (;147;) (type 0) (param i32))
  (func $_ZN4core3ptr37drop_in_place$LT$core..fmt..Error$GT$17h7b0bdc3db1b1dfe9E (;148;) (type 0) (param i32))
  (func $_ZN4core9panicking9panic_fmt17h2298343e5f8a8826E (;149;) (type 1) (param i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    i32.const 1
    i32.store16 offset=28
    local.get 2
    local.get 1
    i32.store offset=24
    local.get 2
    local.get 0
    i32.store offset=20
    local.get 2
    i32.const 1054512
    i32.store offset=16
    local.get 2
    i32.const 1054416
    i32.store offset=12
    local.get 2
    i32.const 12
    i32.add
    call $rust_begin_unwind
    unreachable
  )
  (func $_ZN4core5slice5index26slice_start_index_len_fail17h46dad5e8fa8e060bE (;150;) (type 3) (param i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    local.get 0
    i32.store
    local.get 3
    local.get 1
    i32.store offset=4
    local.get 3
    i32.const 8
    i32.add
    i32.const 12
    i32.add
    i64.const 2
    i64.store align=4
    local.get 3
    i32.const 32
    i32.add
    i32.const 12
    i32.add
    i32.const 7
    i32.store
    local.get 3
    i32.const 2
    i32.store offset=12
    local.get 3
    i32.const 1055108
    i32.store offset=8
    local.get 3
    i32.const 7
    i32.store offset=36
    local.get 3
    local.get 3
    i32.const 32
    i32.add
    i32.store offset=16
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
    call $_ZN4core9panicking9panic_fmt17h2298343e5f8a8826E
    unreachable
  )
  (func $_ZN4core5slice5index24slice_end_index_len_fail17hdd28afa5458bc752E (;151;) (type 3) (param i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    local.get 0
    i32.store
    local.get 3
    local.get 1
    i32.store offset=4
    local.get 3
    i32.const 8
    i32.add
    i32.const 12
    i32.add
    i64.const 2
    i64.store align=4
    local.get 3
    i32.const 32
    i32.add
    i32.const 12
    i32.add
    i32.const 7
    i32.store
    local.get 3
    i32.const 2
    i32.store offset=12
    local.get 3
    i32.const 1055140
    i32.store offset=8
    local.get 3
    i32.const 7
    i32.store offset=36
    local.get 3
    local.get 3
    i32.const 32
    i32.add
    i32.store offset=16
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
    call $_ZN4core9panicking9panic_fmt17h2298343e5f8a8826E
    unreachable
  )
  (func $_ZN4core3fmt9Formatter3pad17h38d826cfb428dcf3E (;152;) (type 4) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32)
    block ;; label = @1
      local.get 0
      i32.load
      local.tee 3
      local.get 0
      i32.load offset=8
      local.tee 4
      i32.or
      i32.eqz
      br_if 0 (;@1;)
      block ;; label = @2
        local.get 4
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        local.get 2
        i32.add
        local.set 5
        local.get 0
        i32.const 12
        i32.add
        i32.load
        i32.const 1
        i32.add
        local.set 6
        i32.const 0
        local.set 7
        local.get 1
        local.set 8
        block ;; label = @3
          loop ;; label = @4
            local.get 8
            local.set 4
            local.get 6
            i32.const -1
            i32.add
            local.tee 6
            i32.eqz
            br_if 1 (;@3;)
            local.get 4
            local.get 5
            i32.eq
            br_if 2 (;@2;)
            block ;; label = @5
              block ;; label = @6
                local.get 4
                i32.load8_s
                local.tee 9
                i32.const -1
                i32.le_s
                br_if 0 (;@6;)
                local.get 4
                i32.const 1
                i32.add
                local.set 8
                local.get 9
                i32.const 255
                i32.and
                local.set 9
                br 1 (;@5;)
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
              block ;; label = @6
                local.get 9
                i32.const -33
                i32.gt_u
                br_if 0 (;@6;)
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
                br 1 (;@5;)
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
              block ;; label = @6
                local.get 9
                i32.const -16
                i32.ge_u
                br_if 0 (;@6;)
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
                br 1 (;@5;)
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
              br_if 3 (;@2;)
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
            br_if 0 (;@4;)
            br 2 (;@2;)
          end
        end
        local.get 4
        local.get 5
        i32.eq
        br_if 0 (;@2;)
        block ;; label = @3
          local.get 4
          i32.load8_s
          local.tee 8
          i32.const -1
          i32.gt_s
          br_if 0 (;@3;)
          local.get 8
          i32.const -32
          i32.lt_u
          br_if 0 (;@3;)
          local.get 8
          i32.const -16
          i32.lt_u
          br_if 0 (;@3;)
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
          br_if 1 (;@2;)
        end
        block ;; label = @3
          block ;; label = @4
            local.get 7
            i32.eqz
            br_if 0 (;@4;)
            block ;; label = @5
              local.get 7
              local.get 2
              i32.lt_u
              br_if 0 (;@5;)
              i32.const 0
              local.set 4
              local.get 7
              local.get 2
              i32.eq
              br_if 1 (;@4;)
              br 2 (;@3;)
            end
            i32.const 0
            local.set 4
            local.get 1
            local.get 7
            i32.add
            i32.load8_s
            i32.const -64
            i32.lt_s
            br_if 1 (;@3;)
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
      block ;; label = @2
        local.get 3
        br_if 0 (;@2;)
        local.get 0
        i32.load offset=20
        local.get 1
        local.get 2
        local.get 0
        i32.const 24
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 4)
        return
      end
      local.get 0
      i32.load offset=4
      local.set 5
      block ;; label = @2
        block ;; label = @3
          local.get 2
          i32.const 16
          i32.lt_u
          br_if 0 (;@3;)
          local.get 1
          local.get 2
          call $_ZN4core3str5count14do_count_chars17h56d3a936cd8fd6a4E
          local.set 4
          br 1 (;@2;)
        end
        block ;; label = @3
          local.get 2
          br_if 0 (;@3;)
          i32.const 0
          local.set 4
          br 1 (;@2;)
        end
        local.get 2
        i32.const 3
        i32.and
        local.set 6
        block ;; label = @3
          block ;; label = @4
            local.get 2
            i32.const 4
            i32.ge_u
            br_if 0 (;@4;)
            i32.const 0
            local.set 4
            i32.const 0
            local.set 9
            br 1 (;@3;)
          end
          local.get 2
          i32.const -4
          i32.and
          local.set 7
          i32.const 0
          local.set 4
          i32.const 0
          local.set 9
          loop ;; label = @4
            local.get 4
            local.get 1
            local.get 9
            i32.add
            local.tee 8
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.get 8
            i32.const 1
            i32.add
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.get 8
            i32.const 2
            i32.add
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.get 8
            i32.const 3
            i32.add
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.set 4
            local.get 7
            local.get 9
            i32.const 4
            i32.add
            local.tee 9
            i32.ne
            br_if 0 (;@4;)
          end
        end
        local.get 6
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        local.get 9
        i32.add
        local.set 8
        loop ;; label = @3
          local.get 4
          local.get 8
          i32.load8_s
          i32.const -65
          i32.gt_s
          i32.add
          local.set 4
          local.get 8
          i32.const 1
          i32.add
          local.set 8
          local.get 6
          i32.const -1
          i32.add
          local.tee 6
          br_if 0 (;@3;)
        end
      end
      block ;; label = @2
        block ;; label = @3
          local.get 5
          local.get 4
          i32.le_u
          br_if 0 (;@3;)
          local.get 5
          local.get 4
          i32.sub
          local.set 7
          i32.const 0
          local.set 4
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                local.get 0
                i32.load8_u offset=32
                br_table 2 (;@4;) 0 (;@6;) 1 (;@5;) 2 (;@4;) 2 (;@4;)
              end
              local.get 7
              local.set 4
              i32.const 0
              local.set 7
              br 1 (;@4;)
            end
            local.get 7
            i32.const 1
            i32.shr_u
            local.set 4
            local.get 7
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
          i32.const 24
          i32.add
          i32.load
          local.set 8
          local.get 0
          i32.load offset=16
          local.set 6
          local.get 0
          i32.load offset=20
          local.set 9
          loop ;; label = @4
            local.get 4
            i32.const -1
            i32.add
            local.tee 4
            i32.eqz
            br_if 2 (;@2;)
            local.get 9
            local.get 6
            local.get 8
            i32.load offset=16
            call_indirect (type 2)
            i32.eqz
            br_if 0 (;@4;)
          end
          i32.const 1
          return
        end
        local.get 0
        i32.load offset=20
        local.get 1
        local.get 2
        local.get 0
        i32.const 24
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 4)
        return
      end
      i32.const 1
      local.set 4
      block ;; label = @2
        local.get 9
        local.get 1
        local.get 2
        local.get 8
        i32.load offset=12
        call_indirect (type 4)
        br_if 0 (;@2;)
        i32.const 0
        local.set 4
        block ;; label = @3
          loop ;; label = @4
            block ;; label = @5
              local.get 7
              local.get 4
              i32.ne
              br_if 0 (;@5;)
              local.get 7
              local.set 4
              br 2 (;@3;)
            end
            local.get 4
            i32.const 1
            i32.add
            local.set 4
            local.get 9
            local.get 6
            local.get 8
            i32.load offset=16
            call_indirect (type 2)
            i32.eqz
            br_if 0 (;@4;)
          end
          local.get 4
          i32.const -1
          i32.add
          local.set 4
        end
        local.get 4
        local.get 7
        i32.lt_u
        local.set 4
      end
      local.get 4
      return
    end
    local.get 0
    i32.load offset=20
    local.get 1
    local.get 2
    local.get 0
    i32.const 24
    i32.add
    i32.load
    i32.load offset=12
    call_indirect (type 4)
  )
  (func $_ZN4core9panicking5panic17h89e93701a34d6dc7E (;153;) (type 3) (param i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    i32.const 12
    i32.add
    i64.const 0
    i64.store align=4
    local.get 3
    i32.const 1
    i32.store offset=4
    local.get 3
    i32.const 1054416
    i32.store offset=8
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
    i32.store
    local.get 3
    local.get 2
    call $_ZN4core9panicking9panic_fmt17h2298343e5f8a8826E
    unreachable
  )
  (func $_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17hbb0e20284f86c495E (;154;) (type 2) (param i32 i32) (result i32)
    local.get 0
    i64.load32_u
    i32.const 1
    local.get 1
    call $_ZN4core3fmt3num3imp7fmt_u6417h4c54fdb4bba2a591E
  )
  (func $_ZN4core3fmt5write17h2d8708cc7ff40a92E (;155;) (type 4) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    i32.const 36
    i32.add
    local.get 1
    i32.store
    local.get 3
    i32.const 3
    i32.store8 offset=44
    local.get 3
    i32.const 32
    i32.store offset=28
    i32.const 0
    local.set 4
    local.get 3
    i32.const 0
    i32.store offset=40
    local.get 3
    local.get 0
    i32.store offset=32
    local.get 3
    i32.const 0
    i32.store offset=20
    local.get 3
    i32.const 0
    i32.store offset=12
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              local.get 2
              i32.load offset=16
              local.tee 5
              br_if 0 (;@5;)
              local.get 2
              i32.const 12
              i32.add
              i32.load
              local.tee 0
              i32.eqz
              br_if 1 (;@4;)
              local.get 2
              i32.load offset=8
              local.tee 1
              local.get 0
              i32.const 3
              i32.shl
              i32.add
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
              i32.load
              local.set 0
              i32.const 0
              local.set 7
              loop ;; label = @6
                block ;; label = @7
                  local.get 0
                  i32.const 4
                  i32.add
                  i32.load
                  local.tee 8
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 3
                  i32.load offset=32
                  local.get 0
                  i32.load
                  local.get 8
                  local.get 3
                  i32.load offset=36
                  i32.load offset=12
                  call_indirect (type 4)
                  br_if 4 (;@3;)
                end
                local.get 1
                i32.load
                local.get 3
                i32.const 12
                i32.add
                local.get 1
                i32.const 4
                i32.add
                i32.load
                call_indirect (type 2)
                br_if 3 (;@3;)
                local.get 7
                i32.const 1
                i32.add
                local.set 7
                local.get 0
                i32.const 8
                i32.add
                local.set 0
                local.get 1
                i32.const 8
                i32.add
                local.tee 1
                local.get 6
                i32.ne
                br_if 0 (;@6;)
                br 2 (;@4;)
              end
            end
            local.get 2
            i32.const 20
            i32.add
            i32.load
            local.tee 1
            i32.eqz
            br_if 0 (;@4;)
            local.get 1
            i32.const 5
            i32.shl
            local.set 9
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
            local.set 10
            local.get 2
            i32.load
            local.set 0
            i32.const 0
            local.set 7
            i32.const 0
            local.set 11
            loop ;; label = @5
              block ;; label = @6
                local.get 0
                i32.const 4
                i32.add
                i32.load
                local.tee 1
                i32.eqz
                br_if 0 (;@6;)
                local.get 3
                i32.load offset=32
                local.get 0
                i32.load
                local.get 1
                local.get 3
                i32.load offset=36
                i32.load offset=12
                call_indirect (type 4)
                br_if 3 (;@3;)
              end
              local.get 3
              local.get 5
              local.get 7
              i32.add
              local.tee 1
              i32.const 16
              i32.add
              i32.load
              i32.store offset=28
              local.get 3
              local.get 1
              i32.const 28
              i32.add
              i32.load8_u
              i32.store8 offset=44
              local.get 3
              local.get 1
              i32.const 24
              i32.add
              i32.load
              i32.store offset=40
              local.get 1
              i32.const 12
              i32.add
              i32.load
              local.set 6
              i32.const 0
              local.set 12
              i32.const 0
              local.set 8
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    local.get 1
                    i32.const 8
                    i32.add
                    i32.load
                    br_table 1 (;@7;) 0 (;@8;) 2 (;@6;) 1 (;@7;)
                  end
                  local.get 6
                  i32.const 3
                  i32.shl
                  local.set 13
                  i32.const 0
                  local.set 8
                  local.get 10
                  local.get 13
                  i32.add
                  local.tee 13
                  i32.load offset=4
                  i32.const 57
                  i32.ne
                  br_if 1 (;@6;)
                  local.get 13
                  i32.load
                  i32.load
                  local.set 6
                end
                i32.const 1
                local.set 8
              end
              local.get 3
              local.get 6
              i32.store offset=16
              local.get 3
              local.get 8
              i32.store offset=12
              local.get 1
              i32.const 4
              i32.add
              i32.load
              local.set 8
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    local.get 1
                    i32.load
                    br_table 1 (;@7;) 0 (;@8;) 2 (;@6;) 1 (;@7;)
                  end
                  local.get 8
                  i32.const 3
                  i32.shl
                  local.set 6
                  local.get 10
                  local.get 6
                  i32.add
                  local.tee 6
                  i32.load offset=4
                  i32.const 57
                  i32.ne
                  br_if 1 (;@6;)
                  local.get 6
                  i32.load
                  i32.load
                  local.set 8
                end
                i32.const 1
                local.set 12
              end
              local.get 3
              local.get 8
              i32.store offset=24
              local.get 3
              local.get 12
              i32.store offset=20
              local.get 10
              local.get 1
              i32.const 20
              i32.add
              i32.load
              i32.const 3
              i32.shl
              i32.add
              local.tee 1
              i32.load
              local.get 3
              i32.const 12
              i32.add
              local.get 1
              i32.const 4
              i32.add
              i32.load
              call_indirect (type 2)
              br_if 2 (;@3;)
              local.get 11
              i32.const 1
              i32.add
              local.set 11
              local.get 0
              i32.const 8
              i32.add
              local.set 0
              local.get 9
              local.get 7
              i32.const 32
              i32.add
              local.tee 7
              i32.ne
              br_if 0 (;@5;)
            end
          end
          local.get 4
          local.get 2
          i32.load offset=4
          i32.ge_u
          br_if 1 (;@2;)
          local.get 3
          i32.load offset=32
          local.get 2
          i32.load
          local.get 4
          i32.const 3
          i32.shl
          i32.add
          local.tee 1
          i32.load
          local.get 1
          i32.load offset=4
          local.get 3
          i32.load offset=36
          i32.load offset=12
          call_indirect (type 4)
          i32.eqz
          br_if 1 (;@2;)
        end
        i32.const 1
        local.set 1
        br 1 (;@1;)
      end
      i32.const 0
      local.set 1
    end
    local.get 3
    i32.const 48
    i32.add
    global.set $__stack_pointer
    local.get 1
  )
  (func $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h9742a3b61000964fE (;156;) (type 1) (param i32 i32)
    local.get 0
    i64.const 834069368081456416
    i64.store offset=8
    local.get 0
    i64.const 607490806584822236
    i64.store
  )
  (func $_ZN63_$LT$core..cell..BorrowMutError$u20$as$u20$core..fmt..Debug$GT$3fmt17h5fd6b2ac60091da4E (;157;) (type 2) (param i32 i32) (result i32)
    local.get 1
    i32.load offset=20
    i32.const 1054417
    i32.const 14
    local.get 1
    i32.const 24
    i32.add
    i32.load
    i32.load offset=12
    call_indirect (type 4)
  )
  (func $_ZN4core4cell22panic_already_borrowed17ha086677c15c4e605E (;158;) (type 0) (param i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    local.get 1
    i32.const 24
    i32.add
    i64.const 1
    i64.store align=4
    local.get 1
    i32.const 1
    i32.store offset=16
    local.get 1
    i32.const 1054452
    i32.store offset=12
    local.get 1
    i32.const 58
    i32.store offset=40
    local.get 1
    local.get 1
    i32.const 36
    i32.add
    i32.store offset=20
    local.get 1
    local.get 1
    i32.const 47
    i32.add
    i32.store offset=36
    local.get 1
    i32.const 12
    i32.add
    local.get 0
    call $_ZN4core9panicking9panic_fmt17h2298343e5f8a8826E
    unreachable
  )
  (func $_ZN4core3ffi5c_str4CStr19from_bytes_with_nul17h4468beb2bd7d2757E (;159;) (type 3) (param i32 i32 i32)
    (local i32 i32 i32 i32)
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  local.get 2
                  i32.const 8
                  i32.lt_u
                  br_if 0 (;@7;)
                  local.get 1
                  i32.const 3
                  i32.add
                  i32.const -4
                  i32.and
                  local.tee 3
                  local.get 1
                  i32.eq
                  br_if 1 (;@6;)
                  local.get 3
                  local.get 1
                  i32.sub
                  local.tee 3
                  i32.eqz
                  br_if 1 (;@6;)
                  i32.const 0
                  local.set 4
                  loop ;; label = @8
                    local.get 1
                    local.get 4
                    i32.add
                    i32.load8_u
                    i32.eqz
                    br_if 5 (;@3;)
                    local.get 3
                    local.get 4
                    i32.const 1
                    i32.add
                    local.tee 4
                    i32.ne
                    br_if 0 (;@8;)
                  end
                  local.get 3
                  local.get 2
                  i32.const -8
                  i32.add
                  local.tee 5
                  i32.gt_u
                  br_if 3 (;@4;)
                  br 2 (;@5;)
                end
                local.get 2
                i32.eqz
                br_if 4 (;@2;)
                block ;; label = @7
                  local.get 1
                  i32.load8_u
                  br_if 0 (;@7;)
                  i32.const 0
                  local.set 4
                  br 4 (;@3;)
                end
                i32.const 1
                local.set 4
                local.get 2
                i32.const 1
                i32.eq
                br_if 4 (;@2;)
                local.get 1
                i32.load8_u offset=1
                i32.eqz
                br_if 3 (;@3;)
                i32.const 2
                local.set 4
                local.get 2
                i32.const 2
                i32.eq
                br_if 4 (;@2;)
                local.get 1
                i32.load8_u offset=2
                i32.eqz
                br_if 3 (;@3;)
                i32.const 3
                local.set 4
                local.get 2
                i32.const 3
                i32.eq
                br_if 4 (;@2;)
                local.get 1
                i32.load8_u offset=3
                i32.eqz
                br_if 3 (;@3;)
                i32.const 4
                local.set 4
                local.get 2
                i32.const 4
                i32.eq
                br_if 4 (;@2;)
                local.get 1
                i32.load8_u offset=4
                i32.eqz
                br_if 3 (;@3;)
                i32.const 5
                local.set 4
                local.get 2
                i32.const 5
                i32.eq
                br_if 4 (;@2;)
                local.get 1
                i32.load8_u offset=5
                i32.eqz
                br_if 3 (;@3;)
                i32.const 6
                local.set 4
                local.get 2
                i32.const 6
                i32.eq
                br_if 4 (;@2;)
                local.get 1
                i32.load8_u offset=6
                i32.eqz
                br_if 3 (;@3;)
                br 4 (;@2;)
              end
              local.get 2
              i32.const -8
              i32.add
              local.set 5
              i32.const 0
              local.set 3
            end
            loop ;; label = @5
              local.get 1
              local.get 3
              i32.add
              local.tee 4
              i32.const 4
              i32.add
              i32.load
              local.tee 6
              i32.const -16843009
              i32.add
              local.get 6
              i32.const -1
              i32.xor
              i32.and
              local.get 4
              i32.load
              local.tee 4
              i32.const -16843009
              i32.add
              local.get 4
              i32.const -1
              i32.xor
              i32.and
              i32.or
              i32.const -2139062144
              i32.and
              br_if 1 (;@4;)
              local.get 3
              i32.const 8
              i32.add
              local.tee 3
              local.get 5
              i32.le_u
              br_if 0 (;@5;)
            end
          end
          local.get 3
          local.get 2
          i32.eq
          br_if 1 (;@2;)
          loop ;; label = @4
            block ;; label = @5
              local.get 1
              local.get 3
              i32.add
              i32.load8_u
              br_if 0 (;@5;)
              local.get 3
              local.set 4
              br 2 (;@3;)
            end
            local.get 2
            local.get 3
            i32.const 1
            i32.add
            local.tee 3
            i32.ne
            br_if 0 (;@4;)
            br 2 (;@2;)
          end
        end
        local.get 4
        i32.const 1
        i32.add
        local.get 2
        i32.eq
        br_if 1 (;@1;)
        local.get 0
        i32.const 0
        i32.store offset=4
        local.get 0
        i32.const 8
        i32.add
        local.get 4
        i32.store
        local.get 0
        i32.const 1
        i32.store
        return
      end
      local.get 0
      i32.const 1
      i32.store offset=4
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
    local.get 2
    i32.store
    local.get 0
    i32.const 0
    i32.store
  )
  (func $_ZN4core3str8converts9from_utf817h2f7de8ce7fc1ff13E (;160;) (type 3) (param i32 i32 i32)
    (local i32 i32 i32 i32 i32 i64 i64 i32)
    block ;; label = @1
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
      local.set 5
      i32.const 0
      local.set 3
      loop ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                local.get 1
                local.get 3
                i32.add
                i32.load8_u
                local.tee 6
                i32.extend8_s
                local.tee 7
                i32.const 0
                i32.lt_s
                br_if 0 (;@6;)
                local.get 5
                local.get 3
                i32.sub
                i32.const 3
                i32.and
                br_if 1 (;@5;)
                local.get 3
                local.get 4
                i32.ge_u
                br_if 2 (;@4;)
                loop ;; label = @7
                  local.get 1
                  local.get 3
                  i32.add
                  local.tee 6
                  i32.const 4
                  i32.add
                  i32.load
                  local.get 6
                  i32.load
                  i32.or
                  i32.const -2139062144
                  i32.and
                  br_if 3 (;@4;)
                  local.get 3
                  i32.const 8
                  i32.add
                  local.tee 3
                  local.get 4
                  i32.lt_u
                  br_if 0 (;@7;)
                  br 3 (;@4;)
                end
              end
              i64.const 1099511627776
              local.set 8
              i64.const 4294967296
              local.set 9
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    block ;; label = @9
                      block ;; label = @10
                        block ;; label = @11
                          block ;; label = @12
                            block ;; label = @13
                              block ;; label = @14
                                block ;; label = @15
                                  block ;; label = @16
                                    block ;; label = @17
                                      local.get 6
                                      i32.const 1055156
                                      i32.add
                                      i32.load8_u
                                      i32.const -2
                                      i32.add
                                      br_table 0 (;@17;) 1 (;@16;) 2 (;@15;) 10 (;@7;)
                                    end
                                    local.get 3
                                    i32.const 1
                                    i32.add
                                    local.tee 6
                                    local.get 2
                                    i32.lt_u
                                    br_if 2 (;@14;)
                                    i64.const 0
                                    local.set 8
                                    i64.const 0
                                    local.set 9
                                    br 9 (;@7;)
                                  end
                                  i64.const 0
                                  local.set 8
                                  local.get 3
                                  i32.const 1
                                  i32.add
                                  local.tee 10
                                  local.get 2
                                  i32.lt_u
                                  br_if 2 (;@13;)
                                  i64.const 0
                                  local.set 9
                                  br 8 (;@7;)
                                end
                                i64.const 0
                                local.set 8
                                local.get 3
                                i32.const 1
                                i32.add
                                local.tee 10
                                local.get 2
                                i32.lt_u
                                br_if 2 (;@12;)
                                i64.const 0
                                local.set 9
                                br 7 (;@7;)
                              end
                              i64.const 1099511627776
                              local.set 8
                              i64.const 4294967296
                              local.set 9
                              local.get 1
                              local.get 6
                              i32.add
                              i32.load8_s
                              i32.const -65
                              i32.gt_s
                              br_if 6 (;@7;)
                              br 7 (;@6;)
                            end
                            local.get 1
                            local.get 10
                            i32.add
                            i32.load8_s
                            local.set 10
                            block ;; label = @13
                              block ;; label = @14
                                block ;; label = @15
                                  local.get 6
                                  i32.const -224
                                  i32.add
                                  br_table 0 (;@15;) 2 (;@13;) 2 (;@13;) 2 (;@13;) 2 (;@13;) 2 (;@13;) 2 (;@13;) 2 (;@13;) 2 (;@13;) 2 (;@13;) 2 (;@13;) 2 (;@13;) 2 (;@13;) 1 (;@14;) 2 (;@13;)
                                end
                                local.get 10
                                i32.const -32
                                i32.and
                                i32.const -96
                                i32.eq
                                br_if 4 (;@10;)
                                br 3 (;@11;)
                              end
                              local.get 10
                              i32.const -97
                              i32.gt_s
                              br_if 2 (;@11;)
                              br 3 (;@10;)
                            end
                            block ;; label = @13
                              local.get 7
                              i32.const 31
                              i32.add
                              i32.const 255
                              i32.and
                              i32.const 12
                              i32.lt_u
                              br_if 0 (;@13;)
                              local.get 7
                              i32.const -2
                              i32.and
                              i32.const -18
                              i32.ne
                              br_if 2 (;@11;)
                              local.get 10
                              i32.const -64
                              i32.lt_s
                              br_if 3 (;@10;)
                              br 2 (;@11;)
                            end
                            local.get 10
                            i32.const -64
                            i32.lt_s
                            br_if 2 (;@10;)
                            br 1 (;@11;)
                          end
                          local.get 1
                          local.get 10
                          i32.add
                          i32.load8_s
                          local.set 10
                          block ;; label = @12
                            block ;; label = @13
                              block ;; label = @14
                                block ;; label = @15
                                  local.get 6
                                  i32.const -240
                                  i32.add
                                  br_table 1 (;@14;) 0 (;@15;) 0 (;@15;) 0 (;@15;) 2 (;@13;) 0 (;@15;)
                                end
                                local.get 7
                                i32.const 15
                                i32.add
                                i32.const 255
                                i32.and
                                i32.const 2
                                i32.gt_u
                                br_if 3 (;@11;)
                                local.get 10
                                i32.const -64
                                i32.ge_s
                                br_if 3 (;@11;)
                                br 2 (;@12;)
                              end
                              local.get 10
                              i32.const 112
                              i32.add
                              i32.const 255
                              i32.and
                              i32.const 48
                              i32.ge_u
                              br_if 2 (;@11;)
                              br 1 (;@12;)
                            end
                            local.get 10
                            i32.const -113
                            i32.gt_s
                            br_if 1 (;@11;)
                          end
                          block ;; label = @12
                            local.get 3
                            i32.const 2
                            i32.add
                            local.tee 6
                            local.get 2
                            i32.lt_u
                            br_if 0 (;@12;)
                            i64.const 0
                            local.set 9
                            br 5 (;@7;)
                          end
                          local.get 1
                          local.get 6
                          i32.add
                          i32.load8_s
                          i32.const -65
                          i32.gt_s
                          br_if 2 (;@9;)
                          i64.const 0
                          local.set 9
                          local.get 3
                          i32.const 3
                          i32.add
                          local.tee 6
                          local.get 2
                          i32.ge_u
                          br_if 4 (;@7;)
                          local.get 1
                          local.get 6
                          i32.add
                          i32.load8_s
                          i32.const -65
                          i32.le_s
                          br_if 5 (;@6;)
                          i64.const 3298534883328
                          local.set 8
                          br 3 (;@8;)
                        end
                        i64.const 1099511627776
                        local.set 8
                        br 2 (;@8;)
                      end
                      i64.const 0
                      local.set 9
                      local.get 3
                      i32.const 2
                      i32.add
                      local.tee 6
                      local.get 2
                      i32.ge_u
                      br_if 2 (;@7;)
                      local.get 1
                      local.get 6
                      i32.add
                      i32.load8_s
                      i32.const -65
                      i32.le_s
                      br_if 3 (;@6;)
                    end
                    i64.const 2199023255552
                    local.set 8
                  end
                  i64.const 4294967296
                  local.set 9
                end
                local.get 0
                local.get 8
                local.get 3
                i64.extend_i32_u
                i64.or
                local.get 9
                i64.or
                i64.store offset=4 align=4
                local.get 0
                i32.const 1
                i32.store
                return
              end
              local.get 6
              i32.const 1
              i32.add
              local.set 3
              br 2 (;@3;)
            end
            local.get 3
            i32.const 1
            i32.add
            local.set 3
            br 1 (;@3;)
          end
          local.get 3
          local.get 2
          i32.ge_u
          br_if 0 (;@3;)
          loop ;; label = @4
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
    i32.store
  )
  (func $_ZN4core3fmt8builders11DebugStruct5field17h8f9ba4edc21dc2d3E (;161;) (type 11) (param i32 i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i64)
    global.get $__stack_pointer
    i32.const 64
    i32.sub
    local.tee 5
    global.set $__stack_pointer
    i32.const 1
    local.set 6
    block ;; label = @1
      local.get 0
      i32.load8_u offset=4
      br_if 0 (;@1;)
      local.get 0
      i32.load8_u offset=5
      local.set 7
      block ;; label = @2
        local.get 0
        i32.load
        local.tee 8
        i32.load offset=28
        local.tee 9
        i32.const 4
        i32.and
        br_if 0 (;@2;)
        i32.const 1
        local.set 6
        local.get 8
        i32.load offset=20
        i32.const 1054723
        i32.const 1054720
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
        i32.const 24
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 4)
        br_if 1 (;@1;)
        i32.const 1
        local.set 6
        local.get 8
        i32.load offset=20
        local.get 1
        local.get 2
        local.get 8
        i32.load offset=24
        i32.load offset=12
        call_indirect (type 4)
        br_if 1 (;@1;)
        i32.const 1
        local.set 6
        local.get 8
        i32.load offset=20
        i32.const 1054672
        i32.const 2
        local.get 8
        i32.load offset=24
        i32.load offset=12
        call_indirect (type 4)
        br_if 1 (;@1;)
        local.get 3
        local.get 8
        local.get 4
        i32.load offset=12
        call_indirect (type 2)
        local.set 6
        br 1 (;@1;)
      end
      block ;; label = @2
        local.get 7
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        i32.const 1
        local.set 6
        local.get 8
        i32.load offset=20
        i32.const 1054725
        i32.const 3
        local.get 8
        i32.const 24
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 4)
        br_if 1 (;@1;)
        local.get 8
        i32.load offset=28
        local.set 9
      end
      i32.const 1
      local.set 6
      local.get 5
      i32.const 1
      i32.store8 offset=27
      local.get 5
      i32.const 52
      i32.add
      i32.const 1054692
      i32.store
      local.get 5
      local.get 8
      i64.load offset=20 align=4
      i64.store offset=12 align=4
      local.get 5
      local.get 5
      i32.const 27
      i32.add
      i32.store offset=20
      local.get 5
      local.get 8
      i64.load offset=8 align=4
      i64.store offset=36 align=4
      local.get 8
      i64.load align=4
      local.set 10
      local.get 5
      local.get 9
      i32.store offset=56
      local.get 5
      local.get 8
      i32.load offset=16
      i32.store offset=44
      local.get 5
      local.get 8
      i32.load8_u offset=32
      i32.store8 offset=60
      local.get 5
      local.get 10
      i64.store offset=28 align=4
      local.get 5
      local.get 5
      i32.const 12
      i32.add
      i32.store offset=48
      local.get 5
      i32.const 12
      i32.add
      local.get 1
      local.get 2
      call $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h200811bc6ac14f53E
      br_if 0 (;@1;)
      local.get 5
      i32.const 12
      i32.add
      i32.const 1054672
      i32.const 2
      call $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h200811bc6ac14f53E
      br_if 0 (;@1;)
      local.get 3
      local.get 5
      i32.const 28
      i32.add
      local.get 4
      i32.load offset=12
      call_indirect (type 2)
      br_if 0 (;@1;)
      local.get 5
      i32.load offset=48
      i32.const 1054728
      i32.const 2
      local.get 5
      i32.load offset=52
      i32.load offset=12
      call_indirect (type 4)
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
    local.get 0
  )
  (func $_ZN4core3fmt3num3imp51_$LT$impl$u20$core..fmt..Display$u20$for$u20$u8$GT$3fmt17h27012b48f7c972baE (;162;) (type 2) (param i32 i32) (result i32)
    local.get 0
    i64.load8_u
    i32.const 1
    local.get 1
    call $_ZN4core3fmt3num3imp7fmt_u6417h4c54fdb4bba2a591E
  )
  (func $_ZN4core6result13unwrap_failed17h3354cd59a5a00fe5E (;163;) (type 12) (param i32 i32 i32 i32 i32)
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
    i64.const 2
    i64.store align=4
    local.get 5
    i32.const 48
    i32.add
    i32.const 12
    i32.add
    i32.const 59
    i32.store
    local.get 5
    i32.const 2
    i32.store offset=28
    local.get 5
    i32.const 1054676
    i32.store offset=24
    local.get 5
    i32.const 60
    i32.store offset=52
    local.get 5
    local.get 5
    i32.const 48
    i32.add
    i32.store offset=32
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
    call $_ZN4core9panicking9panic_fmt17h2298343e5f8a8826E
    unreachable
  )
  (func $_ZN4core6option13expect_failed17hed265c0c5be592b4E (;164;) (type 3) (param i32 i32 i32)
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
    i64.const 1
    i64.store align=4
    local.get 3
    i32.const 1
    i32.store offset=20
    local.get 3
    i32.const 1054460
    i32.store offset=16
    local.get 3
    i32.const 60
    i32.store offset=44
    local.get 3
    local.get 3
    i32.const 40
    i32.add
    i32.store offset=24
    local.get 3
    local.get 3
    i32.const 8
    i32.add
    i32.store offset=40
    local.get 3
    i32.const 16
    i32.add
    local.get 2
    call $_ZN4core9panicking9panic_fmt17h2298343e5f8a8826E
    unreachable
  )
  (func $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h788844651b7cde2eE (;165;) (type 2) (param i32 i32) (result i32)
    local.get 1
    local.get 0
    i32.load
    local.get 0
    i32.load offset=4
    call $_ZN4core3fmt9Formatter3pad17h38d826cfb428dcf3E
  )
  (func $_ZN70_$LT$core..panic..location..Location$u20$as$u20$core..fmt..Display$GT$3fmt17h6e0a5b25ce921e81E (;166;) (type 2) (param i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    i32.const 44
    i32.add
    i32.const 7
    i32.store
    local.get 2
    i32.const 24
    i32.add
    i32.const 12
    i32.add
    i32.const 7
    i32.store
    local.get 2
    i32.const 12
    i32.add
    i64.const 3
    i64.store align=4
    local.get 2
    i32.const 3
    i32.store offset=4
    local.get 2
    i32.const 1054472
    i32.store
    local.get 2
    i32.const 60
    i32.store offset=28
    local.get 2
    local.get 0
    i32.store offset=24
    local.get 2
    local.get 0
    i32.const 12
    i32.add
    i32.store offset=40
    local.get 2
    local.get 0
    i32.const 8
    i32.add
    i32.store offset=32
    local.get 1
    i32.const 24
    i32.add
    i32.load
    local.set 0
    local.get 2
    local.get 2
    i32.const 24
    i32.add
    i32.store offset=8
    local.get 1
    i32.load offset=20
    local.get 0
    local.get 2
    call $_ZN4core3fmt5write17h2d8708cc7ff40a92E
    local.set 0
    local.get 2
    i32.const 48
    i32.add
    global.set $__stack_pointer
    local.get 0
  )
  (func $_ZN73_$LT$core..panic..panic_info..PanicInfo$u20$as$u20$core..fmt..Display$GT$3fmt17hdd12f2cd47938749E (;167;) (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 64
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    i32.const 1
    local.set 3
    block ;; label = @1
      local.get 1
      i32.load offset=20
      local.tee 4
      i32.const 1054496
      i32.const 12
      local.get 1
      i32.const 24
      i32.add
      i32.load
      local.tee 5
      i32.load offset=12
      local.tee 6
      call_indirect (type 4)
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=12
      local.set 1
      local.get 2
      i32.const 16
      i32.add
      i32.const 12
      i32.add
      i64.const 3
      i64.store align=4
      local.get 2
      i32.const 60
      i32.add
      i32.const 7
      i32.store
      local.get 2
      i32.const 40
      i32.add
      i32.const 12
      i32.add
      i32.const 7
      i32.store
      local.get 2
      i32.const 3
      i32.store offset=20
      local.get 2
      i32.const 1054472
      i32.store offset=16
      local.get 2
      local.get 1
      i32.const 12
      i32.add
      i32.store offset=56
      local.get 2
      local.get 1
      i32.const 8
      i32.add
      i32.store offset=48
      local.get 2
      i32.const 60
      i32.store offset=44
      local.get 2
      local.get 1
      i32.store offset=40
      local.get 2
      local.get 2
      i32.const 40
      i32.add
      i32.store offset=24
      local.get 4
      local.get 5
      local.get 2
      i32.const 16
      i32.add
      call $_ZN4core3fmt5write17h2d8708cc7ff40a92E
      br_if 0 (;@1;)
      block ;; label = @2
        block ;; label = @3
          local.get 0
          i32.load offset=8
          local.tee 1
          i32.eqz
          br_if 0 (;@3;)
          local.get 4
          i32.const 1054508
          i32.const 2
          local.get 6
          call_indirect (type 4)
          br_if 2 (;@1;)
          local.get 2
          i32.const 40
          i32.add
          i32.const 16
          i32.add
          local.get 1
          i32.const 16
          i32.add
          i64.load align=4
          i64.store
          local.get 2
          i32.const 40
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
          i64.store offset=40
          local.get 4
          local.get 5
          local.get 2
          i32.const 40
          i32.add
          call $_ZN4core3fmt5write17h2d8708cc7ff40a92E
          br_if 2 (;@1;)
          br 1 (;@2;)
        end
        local.get 2
        local.get 0
        i32.load
        local.tee 1
        local.get 0
        i32.load offset=4
        i32.const 12
        i32.add
        i32.load
        call_indirect (type 1)
        local.get 2
        i64.load
        i64.const -4493808902380553279
        i64.xor
        local.get 2
        i32.const 8
        i32.add
        i64.load
        i64.const -163230743173927068
        i64.xor
        i64.or
        i64.eqz
        i32.eqz
        br_if 0 (;@2;)
        local.get 4
        i32.const 1054508
        i32.const 2
        local.get 6
        call_indirect (type 4)
        br_if 1 (;@1;)
        local.get 4
        local.get 1
        i32.load
        local.get 1
        i32.load offset=4
        local.get 6
        call_indirect (type 4)
        br_if 1 (;@1;)
      end
      i32.const 0
      local.set 3
    end
    local.get 2
    i32.const 64
    i32.add
    global.set $__stack_pointer
    local.get 3
  )
  (func $_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i32$GT$3fmt17he478be80e254b577E (;168;) (type 2) (param i32 i32) (result i32)
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
    loop ;; label = @1
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
      i32.const 16
      i32.lt_u
      local.set 4
      local.get 0
      i32.const 4
      i32.shr_u
      local.set 0
      local.get 4
      i32.eqz
      br_if 0 (;@1;)
    end
    block ;; label = @1
      local.get 3
      i32.const 128
      i32.add
      local.tee 0
      i32.const 128
      i32.le_u
      br_if 0 (;@1;)
      local.get 0
      i32.const 128
      i32.const 1054764
      call $_ZN4core5slice5index26slice_start_index_len_fail17h46dad5e8fa8e060bE
      unreachable
    end
    local.get 1
    i32.const 1
    i32.const 1054780
    i32.const 2
    local.get 2
    local.get 3
    i32.add
    i32.const 128
    i32.add
    i32.const 0
    local.get 3
    i32.sub
    call $_ZN4core3fmt9Formatter12pad_integral17he11b5068b3cc231fE
    local.set 0
    local.get 2
    i32.const 128
    i32.add
    global.set $__stack_pointer
    local.get 0
  )
  (func $_ZN4core9panicking19assert_failed_inner17h1836e5e3a2907691E (;169;) (type 13) (param i32 i32 i32 i32 i32 i32 i32)
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
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            local.get 0
            i32.const 255
            i32.and
            br_table 0 (;@4;) 1 (;@3;) 2 (;@2;) 0 (;@4;)
          end
          local.get 7
          i32.const 1054528
          i32.store offset=24
          i32.const 2
          local.set 2
          br 2 (;@1;)
        end
        local.get 7
        i32.const 1054530
        i32.store offset=24
        i32.const 2
        local.set 2
        br 1 (;@1;)
      end
      local.get 7
      i32.const 1054532
      i32.store offset=24
      i32.const 7
      local.set 2
    end
    local.get 7
    local.get 2
    i32.store offset=28
    block ;; label = @1
      local.get 5
      i32.load
      br_if 0 (;@1;)
      local.get 7
      i32.const 76
      i32.add
      i32.const 59
      i32.store
      local.get 7
      i32.const 56
      i32.add
      i32.const 12
      i32.add
      i32.const 59
      i32.store
      local.get 7
      i32.const 88
      i32.add
      i32.const 12
      i32.add
      i64.const 3
      i64.store align=4
      local.get 7
      i32.const 3
      i32.store offset=92
      local.get 7
      i32.const 1054588
      i32.store offset=88
      local.get 7
      i32.const 60
      i32.store offset=60
      local.get 7
      local.get 7
      i32.const 56
      i32.add
      i32.store offset=96
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
      call $_ZN4core9panicking9panic_fmt17h2298343e5f8a8826E
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
    i64.const 4
    i64.store align=4
    local.get 7
    i32.const 84
    i32.add
    i32.const 59
    i32.store
    local.get 7
    i32.const 76
    i32.add
    i32.const 59
    i32.store
    local.get 7
    i32.const 56
    i32.add
    i32.const 12
    i32.add
    i32.const 61
    i32.store
    local.get 7
    i32.const 4
    i32.store offset=92
    local.get 7
    i32.const 1054640
    i32.store offset=88
    local.get 7
    i32.const 60
    i32.store offset=60
    local.get 7
    local.get 7
    i32.const 56
    i32.add
    i32.store offset=96
    local.get 7
    local.get 7
    i32.const 16
    i32.add
    i32.store offset=80
    local.get 7
    local.get 7
    i32.const 8
    i32.add
    i32.store offset=72
    local.get 7
    local.get 7
    i32.const 32
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
    call $_ZN4core9panicking9panic_fmt17h2298343e5f8a8826E
    unreachable
  )
  (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hcb5869c8b536002bE (;170;) (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 1
    local.get 0
    i32.load offset=4
    i32.load offset=12
    call_indirect (type 2)
  )
  (func $_ZN59_$LT$core..fmt..Arguments$u20$as$u20$core..fmt..Display$GT$3fmt17hc607b93f50400119E (;171;) (type 2) (param i32 i32) (result i32)
    local.get 1
    i32.load offset=20
    local.get 1
    i32.const 24
    i32.add
    i32.load
    local.get 0
    call $_ZN4core3fmt5write17h2d8708cc7ff40a92E
  )
  (func $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h200811bc6ac14f53E (;172;) (type 4) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    local.get 0
    i32.load offset=4
    local.set 3
    local.get 0
    i32.load
    local.set 4
    local.get 0
    i32.load offset=8
    local.set 5
    i32.const 0
    local.set 6
    i32.const 0
    local.set 7
    i32.const 0
    local.set 8
    i32.const 0
    local.set 9
    block ;; label = @1
      loop ;; label = @2
        local.get 9
        i32.const 255
        i32.and
        br_if 1 (;@1;)
        block ;; label = @3
          block ;; label = @4
            local.get 8
            local.get 2
            i32.gt_u
            br_if 0 (;@4;)
            loop ;; label = @5
              local.get 1
              local.get 8
              i32.add
              local.set 10
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    block ;; label = @9
                      block ;; label = @10
                        local.get 2
                        local.get 8
                        i32.sub
                        local.tee 11
                        i32.const 8
                        i32.lt_u
                        br_if 0 (;@10;)
                        local.get 10
                        i32.const 3
                        i32.add
                        i32.const -4
                        i32.and
                        local.tee 0
                        local.get 10
                        i32.eq
                        br_if 1 (;@9;)
                        local.get 0
                        local.get 10
                        i32.sub
                        local.tee 0
                        i32.eqz
                        br_if 1 (;@9;)
                        i32.const 0
                        local.set 12
                        loop ;; label = @11
                          local.get 10
                          local.get 12
                          i32.add
                          i32.load8_u
                          i32.const 10
                          i32.eq
                          br_if 5 (;@6;)
                          local.get 0
                          local.get 12
                          i32.const 1
                          i32.add
                          local.tee 12
                          i32.ne
                          br_if 0 (;@11;)
                        end
                        local.get 0
                        local.get 11
                        i32.const -8
                        i32.add
                        local.tee 13
                        i32.gt_u
                        br_if 3 (;@7;)
                        br 2 (;@8;)
                      end
                      block ;; label = @10
                        local.get 2
                        local.get 8
                        i32.ne
                        br_if 0 (;@10;)
                        local.get 2
                        local.set 8
                        br 6 (;@4;)
                      end
                      i32.const 0
                      local.set 12
                      loop ;; label = @10
                        local.get 10
                        local.get 12
                        i32.add
                        i32.load8_u
                        i32.const 10
                        i32.eq
                        br_if 4 (;@6;)
                        local.get 11
                        local.get 12
                        i32.const 1
                        i32.add
                        local.tee 12
                        i32.ne
                        br_if 0 (;@10;)
                      end
                      local.get 2
                      local.set 8
                      br 5 (;@4;)
                    end
                    local.get 11
                    i32.const -8
                    i32.add
                    local.set 13
                    i32.const 0
                    local.set 0
                  end
                  loop ;; label = @8
                    local.get 10
                    local.get 0
                    i32.add
                    local.tee 12
                    i32.const 4
                    i32.add
                    i32.load
                    local.tee 9
                    i32.const 168430090
                    i32.xor
                    i32.const -16843009
                    i32.add
                    local.get 9
                    i32.const -1
                    i32.xor
                    i32.and
                    local.get 12
                    i32.load
                    local.tee 12
                    i32.const 168430090
                    i32.xor
                    i32.const -16843009
                    i32.add
                    local.get 12
                    i32.const -1
                    i32.xor
                    i32.and
                    i32.or
                    i32.const -2139062144
                    i32.and
                    br_if 1 (;@7;)
                    local.get 0
                    i32.const 8
                    i32.add
                    local.tee 0
                    local.get 13
                    i32.le_u
                    br_if 0 (;@8;)
                  end
                end
                block ;; label = @7
                  local.get 0
                  local.get 11
                  i32.ne
                  br_if 0 (;@7;)
                  local.get 2
                  local.set 8
                  br 3 (;@4;)
                end
                loop ;; label = @7
                  block ;; label = @8
                    local.get 10
                    local.get 0
                    i32.add
                    i32.load8_u
                    i32.const 10
                    i32.ne
                    br_if 0 (;@8;)
                    local.get 0
                    local.set 12
                    br 2 (;@6;)
                  end
                  local.get 11
                  local.get 0
                  i32.const 1
                  i32.add
                  local.tee 0
                  i32.ne
                  br_if 0 (;@7;)
                end
                local.get 2
                local.set 8
                br 2 (;@4;)
              end
              local.get 8
              local.get 12
              i32.add
              local.tee 0
              i32.const 1
              i32.add
              local.set 8
              block ;; label = @6
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
                local.set 9
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
          local.set 9
          local.get 7
          local.set 13
          local.get 2
          local.set 0
          local.get 7
          local.get 2
          i32.eq
          br_if 2 (;@1;)
        end
        block ;; label = @3
          block ;; label = @4
            local.get 5
            i32.load8_u
            i32.eqz
            br_if 0 (;@4;)
            local.get 4
            i32.const 1054716
            i32.const 4
            local.get 3
            i32.load offset=12
            call_indirect (type 4)
            br_if 1 (;@3;)
          end
          local.get 1
          local.get 7
          i32.add
          local.set 12
          local.get 0
          local.get 7
          i32.sub
          local.set 10
          i32.const 0
          local.set 11
          block ;; label = @4
            local.get 0
            local.get 7
            i32.eq
            br_if 0 (;@4;)
            local.get 10
            local.get 12
            i32.add
            i32.const -1
            i32.add
            i32.load8_u
            i32.const 10
            i32.eq
            local.set 11
          end
          local.get 5
          local.get 11
          i32.store8
          local.get 13
          local.set 7
          local.get 4
          local.get 12
          local.get 10
          local.get 3
          i32.load offset=12
          call_indirect (type 4)
          i32.eqz
          br_if 1 (;@2;)
        end
      end
      i32.const 1
      local.set 6
    end
    local.get 6
  )
  (func $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$10write_char17hbd55d212d661fb27E (;173;) (type 2) (param i32 i32) (result i32)
    (local i32 i32)
    local.get 0
    i32.load offset=4
    local.set 2
    local.get 0
    i32.load
    local.set 3
    block ;; label = @1
      local.get 0
      i32.load offset=8
      local.tee 0
      i32.load8_u
      i32.eqz
      br_if 0 (;@1;)
      local.get 3
      i32.const 1054716
      i32.const 4
      local.get 2
      i32.load offset=12
      call_indirect (type 4)
      i32.eqz
      br_if 0 (;@1;)
      i32.const 1
      return
    end
    local.get 0
    local.get 1
    i32.const 10
    i32.eq
    i32.store8
    local.get 3
    local.get 1
    local.get 2
    i32.load offset=16
    call_indirect (type 2)
  )
  (func $_ZN4core3fmt8builders10DebugTuple5field17h41fba5d2ad0adcf6E (;174;) (type 4) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i64)
    global.get $__stack_pointer
    i32.const 64
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 0
    i32.load
    local.set 4
    i32.const 1
    local.set 5
    block ;; label = @1
      local.get 0
      i32.load8_u offset=8
      br_if 0 (;@1;)
      block ;; label = @2
        local.get 0
        i32.load offset=4
        local.tee 6
        i32.load offset=28
        local.tee 7
        i32.const 4
        i32.and
        br_if 0 (;@2;)
        i32.const 1
        local.set 5
        local.get 6
        i32.load offset=20
        i32.const 1054723
        i32.const 1054733
        local.get 4
        select
        i32.const 2
        i32.const 1
        local.get 4
        select
        local.get 6
        i32.const 24
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 4)
        br_if 1 (;@1;)
        local.get 1
        local.get 6
        local.get 2
        i32.load offset=12
        call_indirect (type 2)
        local.set 5
        br 1 (;@1;)
      end
      block ;; label = @2
        local.get 4
        br_if 0 (;@2;)
        i32.const 1
        local.set 5
        local.get 6
        i32.load offset=20
        i32.const 1054734
        i32.const 2
        local.get 6
        i32.const 24
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 4)
        br_if 1 (;@1;)
        local.get 6
        i32.load offset=28
        local.set 7
      end
      i32.const 1
      local.set 5
      local.get 3
      i32.const 1
      i32.store8 offset=27
      local.get 3
      i32.const 52
      i32.add
      i32.const 1054692
      i32.store
      local.get 3
      local.get 6
      i64.load offset=20 align=4
      i64.store offset=12 align=4
      local.get 3
      local.get 3
      i32.const 27
      i32.add
      i32.store offset=20
      local.get 3
      local.get 6
      i64.load offset=8 align=4
      i64.store offset=36 align=4
      local.get 6
      i64.load align=4
      local.set 8
      local.get 3
      local.get 7
      i32.store offset=56
      local.get 3
      local.get 6
      i32.load offset=16
      i32.store offset=44
      local.get 3
      local.get 6
      i32.load8_u offset=32
      i32.store8 offset=60
      local.get 3
      local.get 8
      i64.store offset=28 align=4
      local.get 3
      local.get 3
      i32.const 12
      i32.add
      i32.store offset=48
      local.get 1
      local.get 3
      i32.const 28
      i32.add
      local.get 2
      i32.load offset=12
      call_indirect (type 2)
      br_if 0 (;@1;)
      local.get 3
      i32.load offset=48
      i32.const 1054728
      i32.const 2
      local.get 3
      i32.load offset=52
      i32.load offset=12
      call_indirect (type 4)
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
    local.get 0
  )
  (func $_ZN4core3fmt9Formatter12pad_integral17he11b5068b3cc231fE (;175;) (type 14) (param i32 i32 i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    block ;; label = @1
      block ;; label = @2
        local.get 1
        br_if 0 (;@2;)
        local.get 5
        i32.const 1
        i32.add
        local.set 6
        local.get 0
        i32.load offset=28
        local.set 7
        i32.const 45
        local.set 8
        br 1 (;@1;)
      end
      i32.const 43
      i32.const 1114112
      local.get 0
      i32.load offset=28
      local.tee 7
      i32.const 1
      i32.and
      local.tee 1
      select
      local.set 8
      local.get 1
      local.get 5
      i32.add
      local.set 6
    end
    block ;; label = @1
      block ;; label = @2
        local.get 7
        i32.const 4
        i32.and
        br_if 0 (;@2;)
        i32.const 0
        local.set 2
        br 1 (;@1;)
      end
      block ;; label = @2
        block ;; label = @3
          local.get 3
          i32.const 16
          i32.lt_u
          br_if 0 (;@3;)
          local.get 2
          local.get 3
          call $_ZN4core3str5count14do_count_chars17h56d3a936cd8fd6a4E
          local.set 1
          br 1 (;@2;)
        end
        block ;; label = @3
          local.get 3
          br_if 0 (;@3;)
          i32.const 0
          local.set 1
          br 1 (;@2;)
        end
        local.get 3
        i32.const 3
        i32.and
        local.set 9
        block ;; label = @3
          block ;; label = @4
            local.get 3
            i32.const 4
            i32.ge_u
            br_if 0 (;@4;)
            i32.const 0
            local.set 1
            i32.const 0
            local.set 10
            br 1 (;@3;)
          end
          local.get 3
          i32.const -4
          i32.and
          local.set 11
          i32.const 0
          local.set 1
          i32.const 0
          local.set 10
          loop ;; label = @4
            local.get 1
            local.get 2
            local.get 10
            i32.add
            local.tee 12
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.get 12
            i32.const 1
            i32.add
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.get 12
            i32.const 2
            i32.add
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.get 12
            i32.const 3
            i32.add
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.set 1
            local.get 11
            local.get 10
            i32.const 4
            i32.add
            local.tee 10
            i32.ne
            br_if 0 (;@4;)
          end
        end
        local.get 9
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        local.get 10
        i32.add
        local.set 12
        loop ;; label = @3
          local.get 1
          local.get 12
          i32.load8_s
          i32.const -65
          i32.gt_s
          i32.add
          local.set 1
          local.get 12
          i32.const 1
          i32.add
          local.set 12
          local.get 9
          i32.const -1
          i32.add
          local.tee 9
          br_if 0 (;@3;)
        end
      end
      local.get 1
      local.get 6
      i32.add
      local.set 6
    end
    block ;; label = @1
      block ;; label = @2
        local.get 0
        i32.load
        br_if 0 (;@2;)
        i32.const 1
        local.set 1
        local.get 0
        i32.load offset=20
        local.tee 12
        local.get 0
        i32.load offset=24
        local.tee 10
        local.get 8
        local.get 2
        local.get 3
        call $_ZN4core3fmt9Formatter12pad_integral12write_prefix17h236de0d43a6baa56E
        br_if 1 (;@1;)
        local.get 12
        local.get 4
        local.get 5
        local.get 10
        i32.load offset=12
        call_indirect (type 4)
        return
      end
      block ;; label = @2
        local.get 0
        i32.load offset=4
        local.tee 9
        local.get 6
        i32.gt_u
        br_if 0 (;@2;)
        i32.const 1
        local.set 1
        local.get 0
        i32.load offset=20
        local.tee 12
        local.get 0
        i32.load offset=24
        local.tee 10
        local.get 8
        local.get 2
        local.get 3
        call $_ZN4core3fmt9Formatter12pad_integral12write_prefix17h236de0d43a6baa56E
        br_if 1 (;@1;)
        local.get 12
        local.get 4
        local.get 5
        local.get 10
        i32.load offset=12
        call_indirect (type 4)
        return
      end
      block ;; label = @2
        local.get 7
        i32.const 8
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        i32.load offset=16
        local.set 11
        local.get 0
        i32.const 48
        i32.store offset=16
        local.get 0
        i32.load8_u offset=32
        local.set 7
        i32.const 1
        local.set 1
        local.get 0
        i32.const 1
        i32.store8 offset=32
        local.get 0
        i32.load offset=20
        local.tee 12
        local.get 0
        i32.load offset=24
        local.tee 10
        local.get 8
        local.get 2
        local.get 3
        call $_ZN4core3fmt9Formatter12pad_integral12write_prefix17h236de0d43a6baa56E
        br_if 1 (;@1;)
        local.get 9
        local.get 6
        i32.sub
        i32.const 1
        i32.add
        local.set 1
        block ;; label = @3
          loop ;; label = @4
            local.get 1
            i32.const -1
            i32.add
            local.tee 1
            i32.eqz
            br_if 1 (;@3;)
            local.get 12
            i32.const 48
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
        local.get 12
        local.get 4
        local.get 5
        local.get 10
        i32.load offset=12
        call_indirect (type 4)
        br_if 1 (;@1;)
        local.get 0
        local.get 7
        i32.store8 offset=32
        local.get 0
        local.get 11
        i32.store offset=16
        i32.const 0
        local.set 1
        br 1 (;@1;)
      end
      local.get 9
      local.get 6
      i32.sub
      local.set 6
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            local.get 0
            i32.load8_u offset=32
            local.tee 1
            br_table 2 (;@2;) 0 (;@4;) 1 (;@3;) 0 (;@4;) 2 (;@2;)
          end
          local.get 6
          local.set 1
          i32.const 0
          local.set 6
          br 1 (;@2;)
        end
        local.get 6
        i32.const 1
        i32.shr_u
        local.set 1
        local.get 6
        i32.const 1
        i32.add
        i32.const 1
        i32.shr_u
        local.set 6
      end
      local.get 1
      i32.const 1
      i32.add
      local.set 1
      local.get 0
      i32.const 24
      i32.add
      i32.load
      local.set 12
      local.get 0
      i32.load offset=16
      local.set 9
      local.get 0
      i32.load offset=20
      local.set 10
      block ;; label = @2
        loop ;; label = @3
          local.get 1
          i32.const -1
          i32.add
          local.tee 1
          i32.eqz
          br_if 1 (;@2;)
          local.get 10
          local.get 9
          local.get 12
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
      local.get 10
      local.get 12
      local.get 8
      local.get 2
      local.get 3
      call $_ZN4core3fmt9Formatter12pad_integral12write_prefix17h236de0d43a6baa56E
      br_if 0 (;@1;)
      local.get 10
      local.get 4
      local.get 5
      local.get 12
      i32.load offset=12
      call_indirect (type 4)
      br_if 0 (;@1;)
      i32.const 0
      local.set 1
      loop ;; label = @2
        block ;; label = @3
          local.get 6
          local.get 1
          i32.ne
          br_if 0 (;@3;)
          local.get 6
          local.get 6
          i32.lt_u
          return
        end
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        local.get 10
        local.get 9
        local.get 12
        i32.load offset=16
        call_indirect (type 2)
        i32.eqz
        br_if 0 (;@2;)
      end
      local.get 1
      i32.const -1
      i32.add
      local.get 6
      i32.lt_u
      return
    end
    local.get 1
  )
  (func $_ZN4core3fmt5Write9write_fmt17hfb72976f356d8ad6E (;176;) (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.const 1054692
    local.get 1
    call $_ZN4core3fmt5write17h2d8708cc7ff40a92E
  )
  (func $_ZN4core3str5count14do_count_chars17h56d3a936cd8fd6a4E (;177;) (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32)
    block ;; label = @1
      block ;; label = @2
        local.get 1
        local.get 0
        i32.const 3
        i32.add
        i32.const -4
        i32.and
        local.tee 2
        local.get 0
        i32.sub
        local.tee 3
        i32.lt_u
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
        block ;; label = @3
          local.get 2
          local.get 0
          i32.eq
          local.tee 7
          br_if 0 (;@3;)
          i32.const 0
          local.set 1
          block ;; label = @4
            block ;; label = @5
              local.get 2
              local.get 0
              i32.const -1
              i32.xor
              i32.add
              i32.const 3
              i32.ge_u
              br_if 0 (;@5;)
              i32.const 0
              local.set 8
              br 1 (;@4;)
            end
            i32.const 0
            local.set 8
            loop ;; label = @5
              local.get 1
              local.get 0
              local.get 8
              i32.add
              local.tee 9
              i32.load8_s
              i32.const -65
              i32.gt_s
              i32.add
              local.get 9
              i32.const 1
              i32.add
              i32.load8_s
              i32.const -65
              i32.gt_s
              i32.add
              local.get 9
              i32.const 2
              i32.add
              i32.load8_s
              i32.const -65
              i32.gt_s
              i32.add
              local.get 9
              i32.const 3
              i32.add
              i32.load8_s
              i32.const -65
              i32.gt_s
              i32.add
              local.set 1
              local.get 8
              i32.const 4
              i32.add
              local.tee 8
              br_if 0 (;@5;)
            end
          end
          local.get 7
          br_if 0 (;@3;)
          local.get 0
          local.get 2
          i32.sub
          local.set 2
          local.get 0
          local.get 8
          i32.add
          local.set 9
          loop ;; label = @4
            local.get 1
            local.get 9
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.set 1
            local.get 9
            i32.const 1
            i32.add
            local.set 9
            local.get 2
            i32.const 1
            i32.add
            local.tee 2
            br_if 0 (;@4;)
          end
        end
        local.get 0
        local.get 3
        i32.add
        local.set 8
        block ;; label = @3
          local.get 5
          i32.eqz
          br_if 0 (;@3;)
          local.get 8
          local.get 4
          i32.const -4
          i32.and
          i32.add
          local.tee 9
          i32.load8_s
          i32.const -65
          i32.gt_s
          local.set 6
          local.get 5
          i32.const 1
          i32.eq
          br_if 0 (;@3;)
          local.get 6
          local.get 9
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
          local.get 9
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
        local.set 2
        loop ;; label = @3
          local.get 8
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
          local.set 7
          local.get 4
          i32.const 2
          i32.shl
          local.set 5
          i32.const 0
          local.set 9
          block ;; label = @4
            local.get 4
            i32.const 4
            i32.lt_u
            br_if 0 (;@4;)
            local.get 6
            local.get 5
            i32.const 1008
            i32.and
            i32.add
            local.set 0
            i32.const 0
            local.set 9
            local.get 6
            local.set 1
            loop ;; label = @5
              local.get 1
              i32.const 12
              i32.add
              i32.load
              local.tee 8
              i32.const -1
              i32.xor
              i32.const 7
              i32.shr_u
              local.get 8
              i32.const 6
              i32.shr_u
              i32.or
              i32.const 16843009
              i32.and
              local.get 1
              i32.const 8
              i32.add
              i32.load
              local.tee 8
              i32.const -1
              i32.xor
              i32.const 7
              i32.shr_u
              local.get 8
              i32.const 6
              i32.shr_u
              i32.or
              i32.const 16843009
              i32.and
              local.get 1
              i32.const 4
              i32.add
              i32.load
              local.tee 8
              i32.const -1
              i32.xor
              i32.const 7
              i32.shr_u
              local.get 8
              i32.const 6
              i32.shr_u
              i32.or
              i32.const 16843009
              i32.and
              local.get 1
              i32.load
              local.tee 8
              i32.const -1
              i32.xor
              i32.const 7
              i32.shr_u
              local.get 8
              i32.const 6
              i32.shr_u
              i32.or
              i32.const 16843009
              i32.and
              local.get 9
              i32.add
              i32.add
              i32.add
              i32.add
              local.set 9
              local.get 1
              i32.const 16
              i32.add
              local.tee 1
              local.get 0
              i32.ne
              br_if 0 (;@5;)
            end
          end
          local.get 3
          local.get 4
          i32.sub
          local.set 3
          local.get 6
          local.get 5
          i32.add
          local.set 8
          local.get 9
          i32.const 8
          i32.shr_u
          i32.const 16711935
          i32.and
          local.get 9
          i32.const 16711935
          i32.and
          i32.add
          i32.const 65537
          i32.mul
          i32.const 16
          i32.shr_u
          local.get 2
          i32.add
          local.set 2
          local.get 7
          i32.eqz
          br_if 0 (;@3;)
        end
        local.get 6
        local.get 4
        i32.const 252
        i32.and
        i32.const 2
        i32.shl
        i32.add
        local.tee 9
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
        local.set 1
        block ;; label = @3
          local.get 7
          i32.const 1
          i32.eq
          br_if 0 (;@3;)
          local.get 9
          i32.load offset=4
          local.tee 8
          i32.const -1
          i32.xor
          i32.const 7
          i32.shr_u
          local.get 8
          i32.const 6
          i32.shr_u
          i32.or
          i32.const 16843009
          i32.and
          local.get 1
          i32.add
          local.set 1
          local.get 7
          i32.const 2
          i32.eq
          br_if 0 (;@3;)
          local.get 9
          i32.load offset=8
          local.tee 9
          i32.const -1
          i32.xor
          i32.const 7
          i32.shr_u
          local.get 9
          i32.const 6
          i32.shr_u
          i32.or
          i32.const 16843009
          i32.and
          local.get 1
          i32.add
          local.set 1
        end
        local.get 1
        i32.const 8
        i32.shr_u
        i32.const 459007
        i32.and
        local.get 1
        i32.const 16711935
        i32.and
        i32.add
        i32.const 65537
        i32.mul
        i32.const 16
        i32.shr_u
        local.get 2
        i32.add
        return
      end
      block ;; label = @2
        local.get 1
        br_if 0 (;@2;)
        i32.const 0
        return
      end
      local.get 1
      i32.const 3
      i32.and
      local.set 8
      block ;; label = @2
        block ;; label = @3
          local.get 1
          i32.const 4
          i32.ge_u
          br_if 0 (;@3;)
          i32.const 0
          local.set 2
          i32.const 0
          local.set 9
          br 1 (;@2;)
        end
        local.get 1
        i32.const -4
        i32.and
        local.set 3
        i32.const 0
        local.set 2
        i32.const 0
        local.set 9
        loop ;; label = @3
          local.get 2
          local.get 0
          local.get 9
          i32.add
          local.tee 1
          i32.load8_s
          i32.const -65
          i32.gt_s
          i32.add
          local.get 1
          i32.const 1
          i32.add
          i32.load8_s
          i32.const -65
          i32.gt_s
          i32.add
          local.get 1
          i32.const 2
          i32.add
          i32.load8_s
          i32.const -65
          i32.gt_s
          i32.add
          local.get 1
          i32.const 3
          i32.add
          i32.load8_s
          i32.const -65
          i32.gt_s
          i32.add
          local.set 2
          local.get 3
          local.get 9
          i32.const 4
          i32.add
          local.tee 9
          i32.ne
          br_if 0 (;@3;)
        end
      end
      local.get 8
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      local.get 9
      i32.add
      local.set 1
      loop ;; label = @2
        local.get 2
        local.get 1
        i32.load8_s
        i32.const -65
        i32.gt_s
        i32.add
        local.set 2
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        local.get 8
        i32.const -1
        i32.add
        local.tee 8
        br_if 0 (;@2;)
      end
    end
    local.get 2
  )
  (func $_ZN4core3fmt9Formatter12pad_integral12write_prefix17h236de0d43a6baa56E (;178;) (type 11) (param i32 i32 i32 i32 i32) (result i32)
    (local i32)
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
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
    call_indirect (type 4)
  )
  (func $_ZN4core3fmt9Formatter9write_str17h8cfbd807d1bba582E (;179;) (type 4) (param i32 i32 i32) (result i32)
    local.get 0
    i32.load offset=20
    local.get 1
    local.get 2
    local.get 0
    i32.const 24
    i32.add
    i32.load
    i32.load offset=12
    call_indirect (type 4)
  )
  (func $_ZN4core3fmt9Formatter9write_fmt17h4aae3abbd0cb752aE (;180;) (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load offset=20
    local.get 0
    i32.const 24
    i32.add
    i32.load
    local.get 1
    call $_ZN4core3fmt5write17h2d8708cc7ff40a92E
  )
  (func $_ZN4core3fmt9Formatter26debug_struct_field2_finish17h355b9e13692e645bE (;181;) (type 15) (param i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 11
    global.set $__stack_pointer
    local.get 0
    i32.load offset=20
    local.get 1
    local.get 2
    local.get 0
    i32.const 24
    i32.add
    i32.load
    i32.load offset=12
    call_indirect (type 4)
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
    call $_ZN4core3fmt8builders11DebugStruct5field17h8f9ba4edc21dc2d3E
    local.get 7
    local.get 8
    local.get 9
    local.get 10
    call $_ZN4core3fmt8builders11DebugStruct5field17h8f9ba4edc21dc2d3E
    local.set 1
    local.get 11
    i32.load8_u offset=12
    local.set 2
    block ;; label = @1
      block ;; label = @2
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
      block ;; label = @2
        local.get 1
        i32.load
        local.tee 0
        i32.load8_u offset=28
        i32.const 4
        i32.and
        br_if 0 (;@2;)
        local.get 0
        i32.load offset=20
        i32.const 1054731
        i32.const 2
        local.get 0
        i32.load offset=24
        i32.load offset=12
        call_indirect (type 4)
        local.set 0
        br 1 (;@1;)
      end
      local.get 0
      i32.load offset=20
      i32.const 1054730
      i32.const 1
      local.get 0
      i32.load offset=24
      i32.load offset=12
      call_indirect (type 4)
      local.set 0
    end
    local.get 11
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 0
  )
  (func $_ZN4core3fmt9Formatter25debug_tuple_field1_finish17hc9e36c10b865cf5dE (;182;) (type 11) (param i32 i32 i32 i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 5
    global.set $__stack_pointer
    local.get 5
    local.get 0
    i32.load offset=20
    local.get 1
    local.get 2
    local.get 0
    i32.const 24
    i32.add
    i32.load
    i32.load offset=12
    call_indirect (type 4)
    i32.store8 offset=12
    local.get 5
    local.get 0
    i32.store offset=8
    local.get 5
    local.get 2
    i32.eqz
    i32.store8 offset=13
    local.get 5
    i32.const 0
    i32.store offset=4
    local.get 5
    i32.const 4
    i32.add
    local.get 3
    local.get 4
    call $_ZN4core3fmt8builders10DebugTuple5field17h41fba5d2ad0adcf6E
    local.set 0
    local.get 5
    i32.load8_u offset=12
    local.set 2
    block ;; label = @1
      block ;; label = @2
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
      i32.load offset=8
      local.set 2
      block ;; label = @2
        local.get 1
        i32.const 1
        i32.ne
        br_if 0 (;@2;)
        local.get 5
        i32.load8_u offset=13
        i32.const 255
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        i32.load8_u offset=28
        i32.const 4
        i32.and
        br_if 0 (;@2;)
        i32.const 1
        local.set 0
        local.get 2
        i32.load offset=20
        i32.const 1054736
        i32.const 1
        local.get 2
        i32.const 24
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 4)
        br_if 1 (;@1;)
      end
      local.get 2
      i32.load offset=20
      i32.const 1054416
      i32.const 1
      local.get 2
      i32.const 24
      i32.add
      i32.load
      i32.load offset=12
      call_indirect (type 4)
      local.set 0
    end
    local.get 5
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 0
  )
  (func $_ZN43_$LT$bool$u20$as$u20$core..fmt..Display$GT$3fmt17hfba4e42b73edb2ecE (;183;) (type 2) (param i32 i32) (result i32)
    block ;; label = @1
      local.get 0
      i32.load8_u
      br_if 0 (;@1;)
      local.get 1
      i32.const 1054982
      i32.const 5
      call $_ZN4core3fmt9Formatter3pad17h38d826cfb428dcf3E
      return
    end
    local.get 1
    i32.const 1054987
    i32.const 4
    call $_ZN4core3fmt9Formatter3pad17h38d826cfb428dcf3E
  )
  (func $_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17hbfdf58a1d1fe1e23E (;184;) (type 4) (param i32 i32 i32) (result i32)
    local.get 2
    local.get 0
    local.get 1
    call $_ZN4core3fmt9Formatter3pad17h38d826cfb428dcf3E
  )
  (func $_ZN4core5slice6memchr14memchr_aligned17hc6d2e1527adf972eE (;185;) (type 7) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32)
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            local.get 2
            i32.const 3
            i32.add
            i32.const -4
            i32.and
            local.tee 4
            local.get 2
            i32.eq
            br_if 0 (;@4;)
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
            br_if 0 (;@4;)
            i32.const 0
            local.set 5
            local.get 1
            i32.const 255
            i32.and
            local.set 6
            i32.const 1
            local.set 7
            loop ;; label = @5
              local.get 2
              local.get 5
              i32.add
              i32.load8_u
              local.get 6
              i32.eq
              br_if 4 (;@1;)
              local.get 4
              local.get 5
              i32.const 1
              i32.add
              local.tee 5
              i32.ne
              br_if 0 (;@5;)
            end
            local.get 4
            local.get 3
            i32.const -8
            i32.add
            local.tee 8
            i32.gt_u
            br_if 2 (;@2;)
            br 1 (;@3;)
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
        loop ;; label = @3
          local.get 2
          local.get 4
          i32.add
          local.tee 6
          i32.const 4
          i32.add
          i32.load
          local.get 5
          i32.xor
          local.tee 7
          i32.const -16843009
          i32.add
          local.get 7
          i32.const -1
          i32.xor
          i32.and
          local.get 6
          i32.load
          local.get 5
          i32.xor
          local.tee 6
          i32.const -16843009
          i32.add
          local.get 6
          i32.const -1
          i32.xor
          i32.and
          i32.or
          i32.const -2139062144
          i32.and
          br_if 1 (;@2;)
          local.get 4
          i32.const 8
          i32.add
          local.tee 4
          local.get 8
          i32.le_u
          br_if 0 (;@3;)
        end
      end
      i32.const 0
      local.set 7
      block ;; label = @2
        local.get 4
        local.get 3
        i32.eq
        br_if 0 (;@2;)
        local.get 1
        i32.const 255
        i32.and
        local.set 5
        loop ;; label = @3
          block ;; label = @4
            local.get 2
            local.get 4
            i32.add
            i32.load8_u
            local.get 5
            i32.ne
            br_if 0 (;@4;)
            local.get 4
            local.set 5
            i32.const 1
            local.set 7
            br 3 (;@1;)
          end
          local.get 3
          local.get 4
          i32.const 1
          i32.add
          local.tee 4
          i32.ne
          br_if 0 (;@3;)
        end
      end
      local.get 3
      local.set 5
    end
    local.get 0
    local.get 5
    i32.store offset=4
    local.get 0
    local.get 7
    i32.store
  )
  (func $_ZN4core5slice6memchr7memrchr17h6febb542c92544afE (;186;) (type 7) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    local.get 3
    local.set 4
    local.get 3
    local.set 5
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              local.get 3
              local.get 2
              i32.const 3
              i32.add
              i32.const -4
              i32.and
              local.get 2
              i32.sub
              local.tee 6
              i32.lt_u
              br_if 0 (;@5;)
              local.get 3
              local.get 3
              local.get 6
              i32.sub
              i32.const 7
              i32.and
              local.tee 7
              i32.sub
              local.set 4
              local.get 3
              local.get 7
              i32.lt_u
              br_if 1 (;@4;)
              local.get 6
              local.set 5
            end
            i32.const 0
            local.get 4
            i32.sub
            local.set 8
            local.get 2
            i32.const -1
            i32.add
            local.set 9
            local.get 1
            i32.const 255
            i32.and
            local.set 10
            local.get 3
            local.set 6
            loop ;; label = @5
              local.get 8
              local.get 6
              i32.add
              i32.eqz
              br_if 2 (;@3;)
              local.get 9
              local.get 6
              i32.add
              local.set 7
              local.get 6
              i32.const -1
              i32.add
              local.set 6
              local.get 7
              i32.load8_u
              local.get 10
              i32.ne
              br_if 0 (;@5;)
              br 3 (;@2;)
            end
          end
          local.get 4
          local.get 3
          i32.const 1055040
          call $_ZN4core5slice5index26slice_start_index_len_fail17h46dad5e8fa8e060bE
          unreachable
        end
        local.get 1
        i32.const 255
        i32.and
        i32.const 16843009
        i32.mul
        local.set 7
        block ;; label = @3
          loop ;; label = @4
            local.get 4
            local.tee 6
            local.get 5
            i32.le_u
            br_if 1 (;@3;)
            local.get 6
            i32.const -8
            i32.add
            local.set 4
            local.get 2
            local.get 6
            i32.add
            local.tee 8
            i32.const -4
            i32.add
            i32.load
            local.get 7
            i32.xor
            local.tee 9
            i32.const -16843009
            i32.add
            local.get 9
            i32.const -1
            i32.xor
            i32.and
            local.get 8
            i32.const -8
            i32.add
            i32.load
            local.get 7
            i32.xor
            local.tee 8
            i32.const -16843009
            i32.add
            local.get 8
            i32.const -1
            i32.xor
            i32.and
            i32.or
            i32.const -2139062144
            i32.and
            i32.eqz
            br_if 0 (;@4;)
          end
        end
        block ;; label = @3
          local.get 6
          local.get 3
          i32.gt_u
          br_if 0 (;@3;)
          local.get 2
          i32.const -1
          i32.add
          local.set 4
          local.get 1
          i32.const 255
          i32.and
          local.set 8
          loop ;; label = @4
            block ;; label = @5
              local.get 6
              br_if 0 (;@5;)
              i32.const 0
              local.set 7
              br 4 (;@1;)
            end
            local.get 4
            local.get 6
            i32.add
            local.set 7
            local.get 6
            i32.const -1
            i32.add
            local.set 6
            local.get 7
            i32.load8_u
            local.get 8
            i32.eq
            br_if 2 (;@2;)
            br 0 (;@4;)
          end
        end
        local.get 6
        local.get 3
        i32.const 1055024
        call $_ZN4core5slice5index24slice_end_index_len_fail17hdd28afa5458bc752E
        unreachable
      end
      i32.const 1
      local.set 7
    end
    local.get 0
    local.get 6
    i32.store offset=4
    local.get 0
    local.get 7
    i32.store
  )
  (func $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i8$GT$3fmt17h06b29036c0557900E (;187;) (type 2) (param i32 i32) (result i32)
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
    loop ;; label = @1
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
      i32.const 16
      i32.ge_u
      br_if 0 (;@1;)
    end
    block ;; label = @1
      local.get 0
      i32.const 128
      i32.add
      local.tee 3
      i32.const 128
      i32.le_u
      br_if 0 (;@1;)
      local.get 3
      i32.const 128
      i32.const 1054764
      call $_ZN4core5slice5index26slice_start_index_len_fail17h46dad5e8fa8e060bE
      unreachable
    end
    local.get 1
    i32.const 1
    i32.const 1054780
    i32.const 2
    local.get 2
    local.get 0
    i32.add
    i32.const 128
    i32.add
    i32.const 0
    local.get 0
    i32.sub
    call $_ZN4core3fmt9Formatter12pad_integral17he11b5068b3cc231fE
    local.set 0
    local.get 2
    i32.const 128
    i32.add
    global.set $__stack_pointer
    local.get 0
  )
  (func $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i8$GT$3fmt17hb1f2095a70886ad5E (;188;) (type 2) (param i32 i32) (result i32)
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
    loop ;; label = @1
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
      i32.const 16
      i32.ge_u
      br_if 0 (;@1;)
    end
    block ;; label = @1
      local.get 0
      i32.const 128
      i32.add
      local.tee 3
      i32.const 128
      i32.le_u
      br_if 0 (;@1;)
      local.get 3
      i32.const 128
      i32.const 1054764
      call $_ZN4core5slice5index26slice_start_index_len_fail17h46dad5e8fa8e060bE
      unreachable
    end
    local.get 1
    i32.const 1
    i32.const 1054780
    i32.const 2
    local.get 2
    local.get 0
    i32.add
    i32.const 128
    i32.add
    i32.const 0
    local.get 0
    i32.sub
    call $_ZN4core3fmt9Formatter12pad_integral17he11b5068b3cc231fE
    local.set 0
    local.get 2
    i32.const 128
    i32.add
    global.set $__stack_pointer
    local.get 0
  )
  (func $_ZN4core3fmt3num3imp7fmt_u6417h4c54fdb4bba2a591E (;189;) (type 16) (param i64 i32 i32) (result i32)
    (local i32 i32 i64 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    i32.const 39
    local.set 4
    block ;; label = @1
      block ;; label = @2
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
      loop ;; label = @2
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
        i32.const 1054782
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
        i32.const 1054782
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
    block ;; label = @1
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
      i32.const 1054782
      i32.add
      i32.load16_u align=1
      i32.store16 align=1
    end
    block ;; label = @1
      block ;; label = @2
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
        i32.const 1054782
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
    i32.const 1054416
    i32.const 0
    local.get 3
    i32.const 9
    i32.add
    local.get 4
    i32.add
    i32.const 39
    local.get 4
    i32.sub
    call $_ZN4core3fmt9Formatter12pad_integral17he11b5068b3cc231fE
    local.set 4
    local.get 3
    i32.const 48
    i32.add
    global.set $__stack_pointer
    local.get 4
  )
  (func $_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i32$GT$3fmt17hcc3212d2d4ae287dE (;190;) (type 2) (param i32 i32) (result i32)
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
    loop ;; label = @1
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
      i32.const 16
      i32.lt_u
      local.set 4
      local.get 0
      i32.const 4
      i32.shr_u
      local.set 0
      local.get 4
      i32.eqz
      br_if 0 (;@1;)
    end
    block ;; label = @1
      local.get 3
      i32.const 128
      i32.add
      local.tee 0
      i32.const 128
      i32.le_u
      br_if 0 (;@1;)
      local.get 0
      i32.const 128
      i32.const 1054764
      call $_ZN4core5slice5index26slice_start_index_len_fail17h46dad5e8fa8e060bE
      unreachable
    end
    local.get 1
    i32.const 1
    i32.const 1054780
    i32.const 2
    local.get 2
    local.get 3
    i32.add
    i32.const 128
    i32.add
    i32.const 0
    local.get 3
    i32.sub
    call $_ZN4core3fmt9Formatter12pad_integral17he11b5068b3cc231fE
    local.set 0
    local.get 2
    i32.const 128
    i32.add
    global.set $__stack_pointer
    local.get 0
  )
  (func $_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17heca7deb48076c08aE (;191;) (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.tee 0
    i64.extend_i32_u
    local.get 0
    i32.const -1
    i32.xor
    i64.extend_i32_s
    i64.const 1
    i64.add
    local.get 0
    i32.const -1
    i32.gt_s
    local.tee 0
    select
    local.get 0
    local.get 1
    call $_ZN4core3fmt3num3imp7fmt_u6417h4c54fdb4bba2a591E
  )
  (func $load.command_export (;192;) (type 6)
    call $load
    call $__wasm_call_dtors
  )
  (func $add.command_export (;193;) (type 2) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call $add
    call $__wasm_call_dtors
  )
  (func $print_run_as_root.command_export (;194;) (type 6)
    call $print_run_as_root
    call $__wasm_call_dtors
  )
  (table (;0;) 68 68 funcref)
  (memory (;0;) 17)
  (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
  (export "memory" (memory 0))
  (export "load" (func $load.command_export))
  (export "add" (func $add.command_export))
  (export "print_run_as_root" (func $print_run_as_root.command_export))
  (elem (;0;) (i32.const 1) func $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h4f9db6b1bee3dfa0E $_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17heca7deb48076c08aE $_ZN60_$LT$alloc..string..String$u20$as$u20$core..fmt..Display$GT$3fmt17hf5f49f3ddec890d5E $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h48cca9c67568fdc3E $_ZN60_$LT$std..io..error..Error$u20$as$u20$core..fmt..Display$GT$3fmt17h6c02f8ed609e15d3E $_ZN91_$LT$std..sys_common..backtrace.._print..DisplayBacktrace$u20$as$u20$core..fmt..Display$GT$3fmt17ha679cf9b2fd90b85E $_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17hbb0e20284f86c495E $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hd38fd9b0b0ba4b2eE $_ZN73_$LT$core..panic..panic_info..PanicInfo$u20$as$u20$core..fmt..Display$GT$3fmt17hdd12f2cd47938749E $_ZN3std5alloc24default_alloc_error_hook17h33eb21920aa52409E $_ZN4core3ptr122drop_in_place$LT$$RF$alloc..boxed..Box$LT$dyn$u20$core..error..Error$u2b$core..marker..Sync$u2b$core..marker..Send$GT$$GT$17hf364da63891e5e4bE $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hcc84fdff291ecc01E $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17ha103d6456042d7d4E $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hdcca3a40c8b93508E $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17had20c8f17260966eE $_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h349d8f91f5ab28e4E $_ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Write$GT$9write_str17h4f30c535d16fff94E $_ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Write$GT$10write_char17h828e0b2d8491c700E $_ZN4core3fmt5Write9write_fmt17h7e18207dce1cf897E $_ZN4core3ptr88drop_in_place$LT$std..io..Write..write_fmt..Adapter$LT$alloc..vec..Vec$LT$u8$GT$$GT$$GT$17hcdb72680c98ebe9dE $_ZN80_$LT$std..io..Write..write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17hdfe706db334d8d1fE $_ZN4core3fmt5Write10write_char17h87516656917da6b5E $_ZN4core3fmt5Write9write_fmt17h742432808baa5dd2E $_ZN80_$LT$std..io..Write..write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17h6768fb4712d63ad7E $_ZN4core3fmt5Write10write_char17hb08d0817fd21c14dE $_ZN4core3fmt5Write9write_fmt17h5539445f3fc48d14E $_ZN80_$LT$std..io..Write..write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17hc6c1bc1e398c2f1bE $_ZN4core3fmt5Write10write_char17hb4f3cb455ce5e67cE $_ZN4core3fmt5Write9write_fmt17h027f44fd7eec0aa4E $_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17ha807bba75500b076E $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$5write17h833ff2b5e1425b8eE $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$14write_vectored17hb19441c5f2107b2aE $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$17is_write_vectored17h937f916ed6b57ed4E $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$5flush17hb1cbd1fa24903956E $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$9write_all17h2e325221fa2d7c27E $_ZN3std2io5Write18write_all_vectored17habcb74d67e8f3963E $_ZN3std2io5Write9write_fmt17hf5db5a8e08f12dacE $_ZN4core3ptr29drop_in_place$LT$$LP$$RP$$GT$17h7fb52925df5f1f21E $_ZN64_$LT$std..sys..wasi..stdio..Stderr$u20$as$u20$std..io..Write$GT$5write17hbdd3c5ac45c1ded0E $_ZN64_$LT$std..sys..wasi..stdio..Stderr$u20$as$u20$std..io..Write$GT$14write_vectored17h6387938e85325605E $_ZN64_$LT$std..sys..wasi..stdio..Stderr$u20$as$u20$std..io..Write$GT$17is_write_vectored17h360e1c632c87e1f7E $_ZN64_$LT$std..sys..wasi..stdio..Stderr$u20$as$u20$std..io..Write$GT$5flush17h9046415f8d5293a1E $_ZN3std2io5Write9write_all17h52b2f76405bd61e5E $_ZN3std2io5Write18write_all_vectored17hb7d4ca586d7e3005E $_ZN3std2io5Write9write_fmt17hb2f06b1b320117c2E $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h142b3648bf87ee57E $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h1118ca1ed69c577bE $_ZN99_$LT$std..panicking..begin_panic_handler..StaticStrPayload$u20$as$u20$core..panic..PanicPayload$GT$8take_box17hdb2e0d205360a4b1E $_ZN99_$LT$std..panicking..begin_panic_handler..StaticStrPayload$u20$as$u20$core..panic..PanicPayload$GT$3get17h96c1e5b81ffc4d9cE $_ZN4core3ptr77drop_in_place$LT$std..panicking..begin_panic_handler..FormatStringPayload$GT$17h70cb05dd1c565003E $_ZN102_$LT$std..panicking..begin_panic_handler..FormatStringPayload$u20$as$u20$core..panic..PanicPayload$GT$8take_box17he3eba7e0d8affceeE $_ZN102_$LT$std..panicking..begin_panic_handler..FormatStringPayload$u20$as$u20$core..panic..PanicPayload$GT$3get17h16918555c6e4fc75E $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h59be72fef6d3ef6eE $_ZN64_$LT$core..str..error..Utf8Error$u20$as$u20$core..fmt..Debug$GT$3fmt17hec1ed09a2b6964b9E $_ZN4core3ptr37drop_in_place$LT$core..fmt..Error$GT$17h9c4ba54b1b1f88d7E $_ZN69_$LT$core..alloc..layout..LayoutError$u20$as$u20$core..fmt..Debug$GT$3fmt17hcd10cc24a0c59435E $_ZN4core3ops8function6FnOnce9call_once17hb10a2d73cc646a7dE $_ZN63_$LT$core..cell..BorrowMutError$u20$as$u20$core..fmt..Debug$GT$3fmt17h5fd6b2ac60091da4E $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hcb5869c8b536002bE $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h788844651b7cde2eE $_ZN59_$LT$core..fmt..Arguments$u20$as$u20$core..fmt..Display$GT$3fmt17hc607b93f50400119E $_ZN4core3ptr37drop_in_place$LT$core..fmt..Error$GT$17h7b0bdc3db1b1dfe9E $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h9742a3b61000964fE $_ZN4core3ptr25drop_in_place$LT$char$GT$17h46384f1420881d52E $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h200811bc6ac14f53E $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$10write_char17hbd55d212d661fb27E $_ZN4core3fmt5Write9write_fmt17hfb72976f356d8ad6E)
  (data $.rodata (;0;) (i32.const 1048576) "\0a\00\00\00\00\00\10\00\00\00\00\00\00\00\10\00\01\00\00\00N<\d4\7f\d4\a7\c3\d0\09\de~\ed\a7\97\00\7f\ff\f2\ab\00\0b\00\00\00\04\00\00\00\04\00\00\00\0c\00\00\00reentrant init\00\008\00\10\00\0e\00\00\00/rustc/2d7be73931e0978c8758a672cc7258b417a7e999/library/core/src/cell/once.rs\00\00\00P\00\10\00M\00\00\00\d9\00\00\00B\00\00\00\00\00\00\00\0b\00\00\00\04\00\00\00\04\00\00\00\0d\00\00\00called `Option::unwrap()` on a `None` valueinternal error: entered unreachable code/rustc/2d7be73931e0978c8758a672cc7258b417a7e999/library/alloc/src/vec/mod.rs\00\17\01\10\00L\00\00\00$\08\00\00$\00\00\00\0b\00\00\00\04\00\00\00\04\00\00\00\0e\00\00\00Utf8Errorvalid_up_toerror_len\00\00\00\0b\00\00\00\04\00\00\00\04\00\00\00\0f\00\00\00NoneSome\10\00\00\00\0c\00\00\00\04\00\00\00\11\00\00\00\12\00\00\00\13\00\00\00\14\00\00\00\0c\00\00\00\04\00\00\00\15\00\00\00\16\00\00\00\17\00\00\00\14\00\00\00\0c\00\00\00\04\00\00\00\18\00\00\00\19\00\00\00\1a\00\00\00\14\00\00\00\0c\00\00\00\04\00\00\00\1b\00\00\00\1c\00\00\00\1d\00\00\00\0alibrary/std/src/thread/mod.rsfailed to generate unique thread ID: bitspace exhausted\00\00\00:\02\10\007\00\00\00\1d\02\10\00\1d\00\00\00\98\04\00\00\0d\00\00\00RUST_BACKTRACEcalled `Result::unwrap()` on an `Err` valueassertion failed: mid <= self.len()failed to write the buffered data\00\00\00\e8\02\10\00!\00\00\00\17\00\00\008\00\10\00\00\00\00\00library/std/src/io/buffered/linewritershim.rs\00\00\00 \03\10\00-\00\00\00\01\01\00\00)\00\00\00entity not foundpermission deniedconnection refusedconnection resethost unreachablenetwork unreachableconnection abortednot connectedaddress in useaddress not availablenetwork downbroken pipeentity already existsoperation would blocknot a directoryis a directorydirectory not emptyread-only filesystem or storage mediumfilesystem loop or indirection limit (e.g. symlink loop)stale network file handleinvalid input parameterinvalid datatimed outwrite zerono storage spaceseek on unseekable filefilesystem quota exceededfile too largeresource busyexecutable file busydeadlockcross-device link or renametoo many linksinvalid filenameargument list too longoperation interruptedunsupportedunexpected end of fileout of memoryother erroruncategorized error (os error )\00\00\008\00\10\00\00\00\00\00M\06\10\00\0b\00\00\00X\06\10\00\01\00\00\00library/std/src/io/stdio.rs\00t\06\10\00\1b\00\00\00\eb\02\00\00\14\00\00\00failed printing to : \00\00\00\a0\06\10\00\13\00\00\00\b3\06\10\00\02\00\00\00t\06\10\00\1b\00\00\00\fd\03\00\00\09\00\00\00stdoutlibrary/std/src/io/mod.rs\00\de\06\10\00\19\00\00\00\a3\05\00\00 \00\00\00advancing io slices beyond their length\00\08\07\10\00'\00\00\00\de\06\10\00\19\00\00\00\a5\05\00\00\0d\00\00\00advancing IoSlice beyond its length\00H\07\10\00#\00\00\00library/std/src/sys/wasi/io.rs\00\00t\07\10\00\1e\00\00\00\17\00\00\00\0d\00\00\00failed to write whole buffer\a4\07\10\00\1c\00\00\00\17\00\00\00\de\06\10\00\19\00\00\00\8d\06\00\00$\00\00\00formatter error\00\dc\07\10\00\0f\00\00\00(\00\00\00library/std/src/panic.rs\f8\07\10\00\18\00\00\00\f5\00\00\00\12\00\00\00fullcannot recursively acquire mutex$\08\10\00 \00\00\00library/std/src/sys/wasi/../unsupported/locks/mutex.rs\00\00L\08\10\006\00\00\00\14\00\00\00\09\00\00\00library/std/src/sync/once.rs\94\08\10\00\1c\00\00\00\d0\00\00\00\14\00\00\00\94\08\10\00\1c\00\00\00\d0\00\00\001\00\00\00lock count overflow in reentrant mutexlibrary/std/src/sync/remutex.rs\00\00\00\f6\08\10\00\1f\00\00\00\91\00\00\00\0e\00\00\00file name contained an unexpected NUL byte\00\00(\09\10\00*\00\00\00\14\00\00\00\02\00\00\00T\09\10\00stack backtrace:\0a\00\00\00h\09\10\00\11\00\00\00note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.\0a\84\09\10\00X\00\00\00memory allocation of  bytes failed\0a\00\e4\09\10\00\15\00\00\00\f9\09\10\00\0e\00\00\00 bytes failed\00\00\00\e4\09\10\00\15\00\00\00\18\0a\10\00\0d\00\00\00library/std/src/alloc.rs8\0a\10\00\18\00\00\00b\01\00\00\09\00\00\00library/std/src/panicking.rsBox<dyn Any><unnamed>\00\00\00\1e\00\00\00\0c\00\00\00\04\00\00\00\1f\00\00\00 \00\00\00!\00\00\00\22\00\00\00#\00\00\00$\00\00\00%\00\00\00&\00\00\00\00\00\00\00\01\00\00\00'\00\00\00(\00\00\00)\00\00\00*\00\00\00+\00\00\00,\00\00\00-\00\00\00thread '' panicked at :\0a\e4\0a\10\00\08\00\00\00\ec\0a\10\00\0e\00\00\00\fa\0a\10\00\02\00\00\00\1c\02\10\00\01\00\00\00note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace\0a\00\00\1c\0b\10\00N\00\00\00`\0a\10\00\1c\00\00\00\84\02\00\00\1e\00\00\00\10\00\00\00\0c\00\00\00\04\00\00\00.\00\00\00\0b\00\00\00\08\00\00\00\04\00\00\00/\00\00\00\0b\00\00\00\08\00\00\00\04\00\00\000\00\00\001\00\00\002\00\00\00\10\00\00\00\04\00\00\003\00\00\004\00\00\00&\00\00\00\00\00\00\00\01\00\00\005\00\00\00\0apanicked after panic::always_abort(), aborting.\0a\00\00\008\00\10\00\00\00\00\00\dc\0b\10\001\00\00\00thread panicked while processing panic. aborting.\0a\00\00 \0c\10\002\00\00\00thread caused non-unwinding panic. aborting.\0a\00\00\00\5c\0c\10\00-\00\00\00fatal runtime error: failed to initiate panic, error \00\00\00\94\0c\10\005\00\00\00\1c\02\10\00\01\00\00\00\0b\00\00\00\08\00\00\00\04\00\00\006\00\00\00library/std/src/sys/wasi/os.rs\00\00\ec\0c\10\00\1e\00\00\00C\00\00\006\00\00\00strerror_r failure\00\00\1c\0d\10\00\12\00\00\00\ec\0c\10\00\1e\00\00\00A\00\00\00\0d\00\00\00fatal runtime error: rwlock locked for writing\0a\00H\0d\10\00/\00\00\00one-time initialization may not be performed recursively\80\0d\10\008\00\00\00\10\00\00\00\11\00\00\00\12\00\00\00\10\00\00\00\10\00\00\00\13\00\00\00\12\00\00\00\0d\00\00\00\0e\00\00\00\15\00\00\00\0c\00\00\00\0b\00\00\00\15\00\00\00\15\00\00\00\0f\00\00\00\0e\00\00\00\13\00\00\00&\00\00\008\00\00\00\19\00\00\00\17\00\00\00\0c\00\00\00\09\00\00\00\0a\00\00\00\10\00\00\00\17\00\00\00\19\00\00\00\0e\00\00\00\0d\00\00\00\14\00\00\00\08\00\00\00\1b\00\00\00\0e\00\00\00\10\00\00\00\16\00\00\00\15\00\00\00\0b\00\00\00\16\00\00\00\0d\00\00\00\0b\00\00\00\13\00\00\00`\03\10\00p\03\10\00\81\03\10\00\93\03\10\00\a3\03\10\00\b3\03\10\00\c6\03\10\00\d8\03\10\00\e5\03\10\00\f3\03\10\00\08\04\10\00\14\04\10\00\1f\04\10\004\04\10\00I\04\10\00X\04\10\00f\04\10\00y\04\10\00\9f\04\10\00\d7\04\10\00\f0\04\10\00\07\05\10\00\13\05\10\00\1c\05\10\00&\05\10\006\05\10\00M\05\10\00f\05\10\00t\05\10\00\81\05\10\00\95\05\10\00\9d\05\10\00\b8\05\10\00\c6\05\10\00\d6\05\10\00\ec\05\10\00\01\06\10\00\0c\06\10\00\22\06\10\00/\06\10\00:\06\10\00/\00Success\00Illegal byte sequence\00Domain error\00Result not representable\00Not a tty\00Permission denied\00Operation not permitted\00No such file or directory\00No such process\00File exists\00Value too large for data type\00No space left on device\00Out of memory\00Resource busy\00Interrupted system call\00Resource temporarily unavailable\00Invalid seek\00Cross-device link\00Read-only file system\00Directory not empty\00Connection reset by peer\00Operation timed out\00Connection refused\00Host is unreachable\00Address in use\00Broken pipe\00I/O error\00No such device or address\00No such device\00Not a directory\00Is a directory\00Text file busy\00Exec format error\00Invalid argument\00Argument list too long\00Symbolic link loop\00Filename too long\00Too many open files in system\00No file descriptors available\00Bad file descriptor\00No child process\00Bad address\00File too large\00Too many links\00No locks available\00Resource deadlock would occur\00State not recoverable\00Previous owner died\00Operation canceled\00Function not implemented\00No message of desired type\00Identifier removed\00Link has been severed\00Protocol error\00Bad message\00Not a socket\00Destination address required\00Message too large\00Protocol wrong type for socket\00Protocol not available\00Protocol not supported\00Not supported\00Address family not supported by protocol\00Address not available\00Network is down\00Network unreachable\00Connection reset by network\00Connection aborted\00No buffer space available\00Socket is connected\00Socket not connected\00Operation already in progress\00Operation in progress\00Stale file handle\00Quota exceeded\00Multihop attempted\00Capabilities insufficient\00\00\00\00\00\00\00\00\00u\02N\00\d6\01\e2\04\b9\04\18\01\8e\05\ed\02\16\04\f2\00\97\03\01\038\05\af\01\82\01O\03/\04\1e\00\d4\05\a2\00\12\03\1e\03\c2\01\de\03\08\00\ac\05\00\01d\02\f1\01e\054\02\8c\02\cf\02-\03L\04\e3\05\9f\02\f8\04\1c\05\08\05\b1\02K\05\15\02x\00R\02<\03\f1\03\e4\00\c3\03}\04\cc\00\aa\03y\05$\02n\01m\03\22\04\ab\04D\00\fb\01\ae\00\83\03`\00\e5\01\07\04\94\04^\04+\00X\019\01\92\00\c2\05\9b\01C\02F\01\f6\05\00\00LayoutErrorlibrary/alloc/src/raw_vec.rscapacity overflow\e3\15\10\00\11\00\00\00\c7\15\10\00\1c\00\00\00;\02\00\00\05\00\00\00called `Option::unwrap()` on a `None` valuelibrary/alloc/src/ffi/c_str.rs\00\00\007\16\10\00\1e\00\00\00\1b\01\00\007\00\00\00called `Result::unwrap()` on an `Err` value\007\00\00\00\00\00\00\00\01\00\00\008\00\00\00library/alloc/src/sync.rs\00\00\00\a4\16\10\00\19\00\00\00o\01\00\002\00\00\00)BorrowMutErroralready borrowed: \00\00\00\df\16\10\00\12\00\00\00\d0\16\10\00\00\00\00\00:\00\00\00\d0\16\10\00\00\00\00\00\04\17\10\00\01\00\00\00\04\17\10\00\01\00\00\00panicked at :\0a\00\00>\00\00\00\00\00\00\00\01\00\00\00?\00\00\00==!=matchesassertion `left  right` failed\0a  left: \0a right: \00K\17\10\00\10\00\00\00[\17\10\00\17\00\00\00r\17\10\00\09\00\00\00 right` failed: \0a  left: \00\00\00K\17\10\00\10\00\00\00\94\17\10\00\10\00\00\00\a4\17\10\00\09\00\00\00r\17\10\00\09\00\00\00: \00\00\d0\16\10\00\00\00\00\00\d0\17\10\00\02\00\00\00@\00\00\00\0c\00\00\00\04\00\00\00A\00\00\00B\00\00\00C\00\00\00     { ,  {\0a,\0a} }((\0a,library/core/src/fmt/num.rs\11\18\10\00\1b\00\00\00i\00\00\00\17\00\00\000x00010203040506070809101112131415161718192021222324252627282930313233343536373839404142434445464748495051525354555657585960616263646566676869707172737475767778798081828384858687888990919293949596979899falsetruelibrary/core/src/slice/memchr.rs\00\0f\19\10\00 \00\00\00\9f\00\00\00\09\00\00\00\0f\19\10\00 \00\00\00\83\00\00\00\1e\00\00\00range start index  out of range for slice of length P\19\10\00\12\00\00\00b\19\10\00\22\00\00\00range end index \94\19\10\00\10\00\00\00b\19\10\00\22\00\00\00\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\03\03\03\03\03\03\03\03\03\03\03\03\03\03\03\03\04\04\04\04\04\00\00\00\00\00\00\00\00\00\00\00")
  (data $.data (;1;) (i32.const 1055412) "\01\00\00\00\ff\ff\ff\ff\08\0f\10\00")
  (@producers
    (language "Rust" "")
    (processed-by "rustc" "1.77.0-nightly (2d7be7393 2023-12-23)")
    (processed-by "clang" "16.0.4 (https://github.com/llvm/llvm-project ae42196bc493ffe877a7e3dff8be32035dea4d07)")
  )
)