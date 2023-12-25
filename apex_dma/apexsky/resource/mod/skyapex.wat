(module
  (type (;0;) (func (param i32)))
  (type (;1;) (func (param i32 i32)))
  (type (;2;) (func (param i32 i32) (result i32)))
  (type (;3;) (func (param i32 i32 i32)))
  (type (;4;) (func (param i32 i32 i32) (result i32)))
  (type (;5;) (func (param i32 i32 i32 i32) (result i32)))
  (type (;6;) (func (param i32 i64 i32 i32 i32) (result i32)))
  (type (;7;) (func (param i32 i32 i32 i32)))
  (type (;8;) (func (param i32 i32 i32 i32 i32)))
  (type (;9;) (func (param i64)))
  (type (;10;) (func (param i64 f32)))
  (type (;11;) (func (param i64) (result f32)))
  (type (;12;) (func (param i32) (result i32)))
  (type (;13;) (func))
  (type (;14;) (func (param i32 i32 i64)))
  (type (;15;) (func (result i32)))
  (type (;16;) (func (param i32 i32 i32 i32 i32 i32)))
  (type (;17;) (func (param i32 i32 i32 i32 i32) (result i32)))
  (type (;18;) (func (param i32 i32 i32 i32 i32 i32 i32)))
  (type (;19;) (func (param i32 i32 i32 i32 i32 i32) (result i32)))
  (type (;20;) (func (param i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32) (result i32)))
  (type (;21;) (func (param i32 i32 i32 i32 i32 i32 i32) (result i32)))
  (type (;22;) (func (param i64 i32 i32) (result i32)))
  (import "wasi_snapshot_preview1" "fd_write" (func $_ZN4wasi13lib_generated22wasi_snapshot_preview18fd_write17hc1e6e60059cf136aE (;0;) (type 5)))
  (import "wasi_snapshot_preview1" "random_get" (func $_ZN4wasi13lib_generated22wasi_snapshot_preview110random_get17he04df62476182fdcE (;1;) (type 2)))
  (import "wasi_snapshot_preview1" "environ_get" (func $__imported_wasi_snapshot_preview1_environ_get (;2;) (type 2)))
  (import "wasi_snapshot_preview1" "environ_sizes_get" (func $__imported_wasi_snapshot_preview1_environ_sizes_get (;3;) (type 2)))
  (import "wasi_snapshot_preview1" "proc_exit" (func $__imported_wasi_snapshot_preview1_proc_exit (;4;) (type 0)))
  (func $_ZN9hashbrown3raw5inner21RawTable$LT$T$C$A$GT$14reserve_rehash17h9ffe16f953d8c85aE (;5;) (type 4) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i64 i32 i32 i32 i32 i32 i64 i64)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        local.get 0
        i32.load offset=12
        local.tee 4
        i32.const 1
        i32.add
        local.tee 5
        br_if 0 (;@2;)
        local.get 3
        i32.const 1
        call $_ZN9hashbrown3raw5inner11Fallibility17capacity_overflow17h98ef64c65897ab00E
        local.get 3
        i32.load
        local.set 5
        br 1 (;@1;)
      end
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                local.get 5
                local.get 0
                i32.load offset=4
                local.tee 6
                local.get 6
                i32.const 1
                i32.add
                local.tee 7
                i32.const 3
                i32.shr_u
                local.tee 8
                i32.const 7
                i32.mul
                local.get 6
                i32.const 8
                i32.lt_u
                select
                local.tee 9
                i32.const 1
                i32.shr_u
                i32.le_u
                br_if 0 (;@6;)
                local.get 5
                local.get 9
                i32.const 1
                i32.add
                local.tee 8
                local.get 5
                local.get 8
                i32.gt_u
                select
                local.tee 5
                i32.const 8
                i32.lt_u
                br_if 1 (;@5;)
                block ;; label = @7
                  block ;; label = @8
                    local.get 5
                    i32.const 536870912
                    i32.ge_u
                    br_if 0 (;@8;)
                    i32.const 1
                    local.set 9
                    local.get 5
                    i32.const 3
                    i32.shl
                    local.tee 5
                    i32.const 14
                    i32.lt_u
                    br_if 4 (;@4;)
                    i32.const -1
                    local.get 5
                    i32.const 7
                    i32.div_u
                    i32.const -1
                    i32.add
                    i32.clz
                    i32.shr_u
                    i32.const 1
                    i32.add
                    local.set 9
                    br 1 (;@7;)
                  end
                  local.get 3
                  i32.const 24
                  i32.add
                  i32.const 1
                  call $_ZN9hashbrown3raw5inner11Fallibility17capacity_overflow17h98ef64c65897ab00E
                  local.get 3
                  i32.load offset=24
                  local.tee 5
                  i32.const -2147483647
                  i32.ne
                  br_if 6 (;@1;)
                  local.get 3
                  i32.load offset=28
                  local.set 9
                end
                local.get 9
                i32.const 1073741823
                i32.gt_u
                br_if 3 (;@3;)
                br 2 (;@4;)
              end
              i32.const 0
              local.set 5
              local.get 0
              i32.load
              local.set 10
              block ;; label = @6
                local.get 8
                local.get 7
                i32.const 7
                i32.and
                i32.const 0
                i32.ne
                i32.add
                local.tee 8
                i32.eqz
                br_if 0 (;@6;)
                local.get 8
                i32.const 1
                i32.and
                local.set 11
                block ;; label = @7
                  local.get 8
                  i32.const 1
                  i32.eq
                  br_if 0 (;@7;)
                  local.get 8
                  i32.const 1073741822
                  i32.and
                  local.set 12
                  i32.const 0
                  local.set 5
                  loop ;; label = @8
                    local.get 10
                    local.get 5
                    i32.add
                    local.tee 8
                    local.get 8
                    i64.load
                    local.tee 13
                    i64.const -1
                    i64.xor
                    i64.const 7
                    i64.shr_u
                    i64.const 72340172838076673
                    i64.and
                    local.get 13
                    i64.const 9187201950435737471
                    i64.or
                    i64.add
                    i64.store
                    local.get 8
                    i32.const 8
                    i32.add
                    local.tee 8
                    local.get 8
                    i64.load
                    local.tee 13
                    i64.const -1
                    i64.xor
                    i64.const 7
                    i64.shr_u
                    i64.const 72340172838076673
                    i64.and
                    local.get 13
                    i64.const 9187201950435737471
                    i64.or
                    i64.add
                    i64.store
                    local.get 5
                    i32.const 16
                    i32.add
                    local.set 5
                    local.get 12
                    i32.const -2
                    i32.add
                    local.tee 12
                    br_if 0 (;@8;)
                  end
                end
                local.get 11
                i32.eqz
                br_if 0 (;@6;)
                local.get 10
                local.get 5
                i32.add
                local.tee 5
                local.get 5
                i64.load
                local.tee 13
                i64.const -1
                i64.xor
                i64.const 7
                i64.shr_u
                i64.const 72340172838076673
                i64.and
                local.get 13
                i64.const 9187201950435737471
                i64.or
                i64.add
                i64.store
              end
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    local.get 7
                    i32.const 8
                    i32.lt_u
                    br_if 0 (;@8;)
                    local.get 10
                    local.get 7
                    i32.add
                    local.get 10
                    i64.load align=1
                    i64.store align=1
                    br 1 (;@7;)
                  end
                  local.get 10
                  i32.const 8
                  i32.add
                  local.get 10
                  local.get 7
                  call $memmove
                  drop
                  local.get 7
                  i32.eqz
                  br_if 1 (;@6;)
                end
                local.get 10
                i32.const -4
                i32.add
                local.set 14
                i32.const 0
                local.set 5
                loop ;; label = @7
                  block ;; label = @8
                    local.get 10
                    local.get 5
                    local.tee 15
                    i32.add
                    local.tee 16
                    i32.load8_u
                    i32.const 128
                    i32.ne
                    br_if 0 (;@8;)
                    block ;; label = @9
                      block ;; label = @10
                        block ;; label = @11
                          local.get 14
                          local.get 15
                          i32.const 2
                          i32.shl
                          local.tee 8
                          i32.sub
                          local.tee 17
                          i32.load
                          local.tee 5
                          local.get 2
                          i32.ge_u
                          br_if 0 (;@11;)
                          local.get 10
                          local.get 8
                          i32.sub
                          i32.const -4
                          i32.add
                          local.set 12
                          loop ;; label = @12
                            local.get 1
                            local.get 5
                            i32.const 5
                            i32.shl
                            i32.add
                            i32.load offset=24
                            local.tee 11
                            local.get 6
                            i32.and
                            local.tee 7
                            local.set 8
                            block ;; label = @13
                              local.get 10
                              local.get 7
                              i32.add
                              i64.load align=1
                              i64.const -9187201950435737472
                              i64.and
                              local.tee 13
                              i64.const 0
                              i64.ne
                              br_if 0 (;@13;)
                              i32.const 8
                              local.set 5
                              local.get 7
                              local.set 8
                              loop ;; label = @14
                                local.get 8
                                local.get 5
                                i32.add
                                local.set 8
                                local.get 5
                                i32.const 8
                                i32.add
                                local.set 5
                                local.get 10
                                local.get 8
                                local.get 6
                                i32.and
                                local.tee 8
                                i32.add
                                i64.load align=1
                                i64.const -9187201950435737472
                                i64.and
                                local.tee 13
                                i64.eqz
                                br_if 0 (;@14;)
                              end
                            end
                            block ;; label = @13
                              local.get 10
                              local.get 13
                              i64.ctz
                              i32.wrap_i64
                              i32.const 3
                              i32.shr_u
                              local.get 8
                              i32.add
                              local.get 6
                              i32.and
                              local.tee 5
                              i32.add
                              i32.load8_s
                              i32.const 0
                              i32.lt_s
                              br_if 0 (;@13;)
                              local.get 10
                              i64.load
                              i64.const -9187201950435737472
                              i64.and
                              i64.ctz
                              i32.wrap_i64
                              i32.const 3
                              i32.shr_u
                              local.set 5
                            end
                            local.get 5
                            local.get 7
                            i32.sub
                            local.get 15
                            local.get 7
                            i32.sub
                            i32.xor
                            local.get 6
                            i32.and
                            i32.const 8
                            i32.lt_u
                            br_if 3 (;@9;)
                            local.get 10
                            local.get 5
                            i32.add
                            local.tee 8
                            i32.load8_u
                            local.set 7
                            local.get 8
                            local.get 11
                            i32.const 25
                            i32.shr_u
                            local.tee 11
                            i32.store8
                            local.get 5
                            i32.const -8
                            i32.add
                            local.get 6
                            i32.and
                            local.get 10
                            i32.add
                            i32.const 8
                            i32.add
                            local.get 11
                            i32.store8
                            local.get 10
                            local.get 5
                            i32.const 2
                            i32.shl
                            i32.sub
                            i32.const -4
                            i32.add
                            local.set 5
                            local.get 7
                            i32.const 255
                            i32.eq
                            br_if 2 (;@10;)
                            local.get 12
                            i32.load8_u
                            local.set 8
                            local.get 12
                            local.get 5
                            i32.load8_u
                            i32.store8
                            local.get 5
                            local.get 8
                            i32.store8
                            local.get 12
                            i32.load8_u offset=1
                            local.set 8
                            local.get 12
                            local.get 5
                            i32.load8_u offset=1
                            i32.store8 offset=1
                            local.get 5
                            local.get 8
                            i32.store8 offset=1
                            local.get 12
                            i32.load8_u offset=2
                            local.set 8
                            local.get 12
                            local.get 5
                            i32.load8_u offset=2
                            i32.store8 offset=2
                            local.get 5
                            local.get 8
                            i32.store8 offset=2
                            local.get 12
                            i32.load8_u offset=3
                            local.set 8
                            local.get 12
                            local.get 5
                            i32.load8_u offset=3
                            i32.store8 offset=3
                            local.get 5
                            local.get 8
                            i32.store8 offset=3
                            local.get 17
                            i32.load
                            local.tee 5
                            local.get 2
                            i32.lt_u
                            br_if 0 (;@12;)
                          end
                        end
                        local.get 5
                        local.get 2
                        i32.const 1048668
                        call $_ZN4core9panicking18panic_bounds_check17he2ca869b88362af2E
                        unreachable
                      end
                      local.get 16
                      i32.const 255
                      i32.store8
                      local.get 15
                      i32.const -8
                      i32.add
                      local.get 6
                      i32.and
                      local.get 10
                      i32.add
                      i32.const 8
                      i32.add
                      i32.const 255
                      i32.store8
                      local.get 5
                      local.get 12
                      i32.load align=1
                      i32.store align=1
                      br 1 (;@8;)
                    end
                    local.get 16
                    local.get 11
                    i32.const 25
                    i32.shr_u
                    local.tee 5
                    i32.store8
                    local.get 15
                    i32.const -8
                    i32.add
                    local.get 6
                    i32.and
                    local.get 10
                    i32.add
                    i32.const 8
                    i32.add
                    local.get 5
                    i32.store8
                  end
                  local.get 15
                  i32.const 1
                  i32.add
                  local.set 5
                  local.get 15
                  local.get 6
                  i32.ne
                  br_if 0 (;@7;)
                end
              end
              local.get 0
              local.get 9
              local.get 4
              i32.sub
              i32.store offset=8
              i32.const -2147483647
              local.set 5
              br 4 (;@1;)
            end
            i32.const 4
            i32.const 8
            local.get 5
            i32.const 4
            i32.lt_u
            select
            local.set 9
          end
          local.get 9
          i32.const 2
          i32.shl
          local.tee 5
          i32.const 7
          i32.add
          local.tee 8
          local.get 5
          i32.lt_u
          br_if 0 (;@3;)
          local.get 8
          i32.const -8
          i32.and
          local.tee 8
          local.get 9
          i32.const 8
          i32.add
          local.tee 10
          i32.add
          local.tee 5
          local.get 8
          i32.lt_u
          br_if 0 (;@3;)
          local.get 5
          i32.const 2147483641
          i32.lt_u
          br_if 1 (;@2;)
        end
        local.get 3
        i32.const 8
        i32.add
        i32.const 1
        call $_ZN9hashbrown3raw5inner11Fallibility17capacity_overflow17h98ef64c65897ab00E
        local.get 3
        i32.load offset=8
        local.set 5
        br 1 (;@1;)
      end
      i32.const 0
      i32.load8_u offset=1063681
      drop
      block ;; label = @2
        local.get 5
        i32.const 8
        call $__rust_alloc
        local.tee 12
        br_if 0 (;@2;)
        local.get 3
        i32.const 16
        i32.add
        i32.const 1
        i32.const 8
        local.get 5
        call $_ZN9hashbrown3raw5inner11Fallibility9alloc_err17h5d5bef80305c3a93E
        local.get 3
        i32.load offset=16
        local.set 5
        br 1 (;@1;)
      end
      local.get 12
      local.get 8
      i32.add
      i32.const 255
      local.get 10
      call $memset
      local.set 12
      local.get 9
      i32.const -1
      i32.add
      local.set 7
      local.get 0
      i32.load
      local.set 14
      block ;; label = @2
        local.get 4
        i32.eqz
        br_if 0 (;@2;)
        local.get 14
        i32.const -4
        i32.add
        local.set 18
        local.get 14
        i64.load
        i64.const -1
        i64.xor
        i64.const -9187201950435737472
        i64.and
        local.set 13
        local.get 14
        local.set 11
        local.get 4
        local.set 15
        i32.const 0
        local.set 10
        loop ;; label = @3
          block ;; label = @4
            local.get 13
            i64.const 0
            i64.ne
            br_if 0 (;@4;)
            local.get 11
            local.set 5
            loop ;; label = @5
              local.get 10
              i32.const 8
              i32.add
              local.set 10
              local.get 5
              i64.load offset=8
              local.set 13
              local.get 5
              i32.const 8
              i32.add
              local.tee 11
              local.set 5
              local.get 13
              i64.const -1
              i64.xor
              i64.const -9187201950435737472
              i64.and
              local.tee 13
              i64.eqz
              br_if 0 (;@5;)
            end
          end
          block ;; label = @4
            local.get 18
            local.get 13
            i64.ctz
            i32.wrap_i64
            i32.const 3
            i32.shr_u
            local.get 10
            i32.add
            i32.const 2
            i32.shl
            local.tee 17
            i32.sub
            i32.load
            local.tee 5
            local.get 2
            i32.lt_u
            br_if 0 (;@4;)
            local.get 5
            local.get 2
            i32.const 1048668
            call $_ZN4core9panicking18panic_bounds_check17he2ca869b88362af2E
            unreachable
          end
          block ;; label = @4
            local.get 12
            local.get 1
            local.get 5
            i32.const 5
            i32.shl
            i32.add
            i32.load offset=24
            local.tee 16
            local.get 7
            i32.and
            local.tee 8
            i32.add
            i64.load align=1
            i64.const -9187201950435737472
            i64.and
            local.tee 19
            i64.const 0
            i64.ne
            br_if 0 (;@4;)
            i32.const 8
            local.set 5
            loop ;; label = @5
              local.get 8
              local.get 5
              i32.add
              local.set 8
              local.get 5
              i32.const 8
              i32.add
              local.set 5
              local.get 12
              local.get 8
              local.get 7
              i32.and
              local.tee 8
              i32.add
              i64.load align=1
              i64.const -9187201950435737472
              i64.and
              local.tee 19
              i64.eqz
              br_if 0 (;@5;)
            end
          end
          local.get 13
          i64.const -1
          i64.add
          local.set 20
          block ;; label = @4
            local.get 12
            local.get 19
            i64.ctz
            i32.wrap_i64
            i32.const 3
            i32.shr_u
            local.get 8
            i32.add
            local.get 7
            i32.and
            local.tee 5
            i32.add
            i32.load8_s
            i32.const 0
            i32.lt_s
            br_if 0 (;@4;)
            local.get 12
            i64.load
            i64.const -9187201950435737472
            i64.and
            i64.ctz
            i32.wrap_i64
            i32.const 3
            i32.shr_u
            local.set 5
          end
          local.get 20
          local.get 13
          i64.and
          local.set 13
          local.get 12
          local.get 5
          i32.add
          local.get 16
          i32.const 25
          i32.shr_u
          local.tee 8
          i32.store8
          local.get 5
          i32.const -8
          i32.add
          local.get 7
          i32.and
          local.get 12
          i32.add
          i32.const 8
          i32.add
          local.get 8
          i32.store8
          local.get 12
          local.get 5
          i32.const 2
          i32.shl
          i32.sub
          i32.const -4
          i32.add
          local.get 14
          local.get 17
          i32.sub
          i32.const -4
          i32.add
          i32.load align=1
          i32.store
          local.get 15
          i32.const -1
          i32.add
          local.tee 15
          br_if 0 (;@3;)
        end
      end
      local.get 0
      local.get 7
      i32.store offset=4
      local.get 0
      local.get 12
      i32.store
      local.get 0
      local.get 7
      local.get 9
      i32.const 3
      i32.shr_u
      i32.const 7
      i32.mul
      local.get 7
      i32.const 8
      i32.lt_u
      select
      local.get 4
      i32.sub
      i32.store offset=8
      i32.const -2147483647
      local.set 5
      local.get 6
      i32.eqz
      br_if 0 (;@1;)
      local.get 14
      local.get 6
      i32.const 2
      i32.shl
      i32.const 11
      i32.add
      i32.const -8
      i32.and
      local.tee 8
      i32.sub
      local.get 6
      local.get 8
      i32.add
      i32.const 9
      i32.add
      i32.const 8
      call $__rust_dealloc
    end
    local.get 3
    i32.const 32
    i32.add
    global.set $__stack_pointer
    local.get 5
  )
  (func $_ZN9hashbrown3raw5inner21RawTable$LT$T$C$A$GT$6insert17hc002d8abdf0218a4E (;6;) (type 6) (param i32 i64 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32)
    block ;; label = @1
      local.get 0
      i32.load
      local.tee 5
      local.get 0
      i32.load offset=4
      local.tee 6
      local.get 1
      i32.wrap_i64
      local.tee 7
      i32.and
      local.tee 8
      i32.add
      i64.load align=1
      i64.const -9187201950435737472
      i64.and
      local.tee 1
      i64.const 0
      i64.ne
      br_if 0 (;@1;)
      i32.const 8
      local.set 9
      loop ;; label = @2
        local.get 8
        local.get 9
        i32.add
        local.set 8
        local.get 9
        i32.const 8
        i32.add
        local.set 9
        local.get 5
        local.get 8
        local.get 6
        i32.and
        local.tee 8
        i32.add
        i64.load align=1
        i64.const -9187201950435737472
        i64.and
        local.tee 1
        i64.eqz
        br_if 0 (;@2;)
      end
    end
    block ;; label = @1
      local.get 5
      local.get 1
      i64.ctz
      i32.wrap_i64
      i32.const 3
      i32.shr_u
      local.get 8
      i32.add
      local.get 6
      i32.and
      local.tee 9
      i32.add
      i32.load8_s
      i32.const 0
      i32.lt_s
      br_if 0 (;@1;)
      local.get 5
      i64.load
      i64.const -9187201950435737472
      i64.and
      i64.ctz
      i32.wrap_i64
      i32.const 3
      i32.shr_u
      local.set 9
    end
    block ;; label = @1
      local.get 0
      i32.load offset=8
      br_if 0 (;@1;)
      local.get 5
      local.get 9
      i32.add
      i32.load8_u
      i32.const 1
      i32.and
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      local.get 3
      local.get 4
      call $_ZN9hashbrown3raw5inner21RawTable$LT$T$C$A$GT$14reserve_rehash17h9ffe16f953d8c85aE
      drop
      block ;; label = @2
        local.get 0
        i32.load
        local.tee 5
        local.get 0
        i32.load offset=4
        local.tee 6
        local.get 7
        i32.and
        local.tee 8
        i32.add
        i64.load align=1
        i64.const -9187201950435737472
        i64.and
        local.tee 1
        i64.const 0
        i64.ne
        br_if 0 (;@2;)
        i32.const 8
        local.set 9
        loop ;; label = @3
          local.get 8
          local.get 9
          i32.add
          local.set 8
          local.get 9
          i32.const 8
          i32.add
          local.set 9
          local.get 5
          local.get 8
          local.get 6
          i32.and
          local.tee 8
          i32.add
          i64.load align=1
          i64.const -9187201950435737472
          i64.and
          local.tee 1
          i64.eqz
          br_if 0 (;@3;)
        end
      end
      local.get 5
      local.get 1
      i64.ctz
      i32.wrap_i64
      i32.const 3
      i32.shr_u
      local.get 8
      i32.add
      local.get 6
      i32.and
      local.tee 9
      i32.add
      i32.load8_s
      i32.const 0
      i32.lt_s
      br_if 0 (;@1;)
      local.get 5
      i64.load
      i64.const -9187201950435737472
      i64.and
      i64.ctz
      i32.wrap_i64
      i32.const 3
      i32.shr_u
      local.set 9
    end
    local.get 5
    local.get 9
    i32.add
    local.tee 8
    i32.load8_u
    local.set 4
    local.get 8
    local.get 7
    i32.const 25
    i32.shr_u
    local.tee 7
    i32.store8
    local.get 9
    i32.const -8
    i32.add
    local.get 6
    i32.and
    local.get 5
    i32.add
    i32.const 8
    i32.add
    local.get 7
    i32.store8
    local.get 0
    local.get 0
    i32.load offset=12
    i32.const 1
    i32.add
    i32.store offset=12
    local.get 0
    local.get 0
    i32.load offset=8
    local.get 4
    i32.const 1
    i32.and
    i32.sub
    i32.store offset=8
    local.get 5
    local.get 9
    i32.const 2
    i32.shl
    i32.sub
    local.tee 9
    i32.const -4
    i32.add
    local.get 2
    i32.store
    local.get 9
  )
  (func $_ZN3std3sys4wasi4once4Once4call17h3585a68f7569d435E (;7;) (type 7) (param i32 i32 i32 i32)
    (local i32 i32 i64 i32)
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
                local.get 0
                i32.load8_u
                br_table 1 (;@5;) 0 (;@6;) 5 (;@1;) 2 (;@4;) 1 (;@5;)
              end
              local.get 1
              i32.eqz
              br_if 2 (;@3;)
            end
            local.get 0
            i32.const 2
            i32.store8
            local.get 2
            i32.load
            local.tee 1
            i32.load
            local.set 2
            local.get 1
            i32.const 0
            i32.store
            local.get 2
            i32.eqz
            br_if 2 (;@2;)
            local.get 2
            i32.load
            local.tee 2
            i32.const 0
            i32.store8 offset=12
            local.get 2
            i32.const 0
            i32.store offset=8
            local.get 2
            i32.load offset=32
            local.set 5
            local.get 2
            i32.const -2147483648
            i32.store offset=32
            local.get 2
            i64.load
            local.set 6
            local.get 2
            i64.const 1
            i64.store
            block ;; label = @5
              local.get 6
              i64.eqz
              br_if 0 (;@5;)
              local.get 5
              i32.const -2147483648
              i32.eq
              br_if 0 (;@5;)
              local.get 2
              i32.load offset=40
              local.set 1
              local.get 2
              i32.load offset=36
              local.set 7
              block ;; label = @6
                local.get 2
                i32.load offset=48
                local.tee 3
                i32.eqz
                br_if 0 (;@6;)
                local.get 2
                i32.load offset=44
                local.get 3
                i32.const 2
                i32.shl
                i32.const 11
                i32.add
                i32.const -8
                i32.and
                local.tee 2
                i32.sub
                local.get 3
                local.get 2
                i32.add
                i32.const 9
                i32.add
                i32.const 8
                call $__rust_dealloc
              end
              block ;; label = @6
                local.get 1
                i32.eqz
                br_if 0 (;@6;)
                local.get 7
                local.set 2
                loop ;; label = @7
                  block ;; label = @8
                    local.get 2
                    i32.load
                    local.tee 3
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 2
                    i32.const 4
                    i32.add
                    i32.load
                    local.get 3
                    i32.const 2
                    i32.shl
                    i32.const 4
                    call $__rust_dealloc
                  end
                  local.get 2
                  i32.const 32
                  i32.add
                  local.set 2
                  local.get 1
                  i32.const -1
                  i32.add
                  local.tee 1
                  br_if 0 (;@7;)
                end
              end
              local.get 5
              i32.eqz
              br_if 0 (;@5;)
              local.get 7
              local.get 5
              i32.const 5
              i32.shl
              i32.const 8
              call $__rust_dealloc
            end
            local.get 0
            i32.const 3
            i32.store8
          end
          local.get 4
          i32.const 32
          i32.add
          global.set $__stack_pointer
          return
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
        i32.const 1048728
        i32.store offset=8
        local.get 4
        i32.const 1048684
        i32.store offset=16
        local.get 4
        i32.const 8
        i32.add
        local.get 3
        call $_ZN4core9panicking9panic_fmt17h4c9c94dcfde83250E
        unreachable
      end
      i32.const 1048800
      i32.const 43
      i32.const 1048920
      call $_ZN4core9panicking5panic17h5f3201ae514c7bcbE
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
    i32.const 1048792
    i32.store offset=8
    local.get 4
    i32.const 1048684
    i32.store offset=16
    local.get 4
    i32.const 8
    i32.add
    local.get 3
    call $_ZN4core9panicking9panic_fmt17h4c9c94dcfde83250E
    unreachable
  )
  (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hdc74b241136d2562E (;8;) (type 2) (param i32 i32) (result i32)
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
        call $_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17he6e3ccd6b1fca402E
        return
      end
      local.get 0
      local.get 1
      call $_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i32$GT$3fmt17h6acce4b91e41041fE
      return
    end
    local.get 0
    local.get 1
    call $_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i32$GT$3fmt17hff8d8204aa9c9138E
  )
  (func $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hb954ea6cfc180865E (;9;) (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 0
    i32.load offset=4
    local.get 1
    call $_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17h33dd62bf170497e3E
  )
  (func $_ZN4core3fmt5Write10write_char17hf85733f50d686e5cE (;10;) (type 2) (param i32 i32) (result i32)
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
    call $_ZN3std2io5Write9write_all17h686446650d647b1cE
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
  (func $_ZN4core3fmt5Write9write_fmt17h28e824752ff5ab22E (;11;) (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.const 1048952
    local.get 1
    call $_ZN4core3fmt5write17h890955524eea605cE
  )
  (func $_ZN4core3ptr30drop_in_place$LT$$RF$isize$GT$17hbf34798627917637E (;12;) (type 0) (param i32))
  (func $_ZN4core3ptr92drop_in_place$LT$std..io..Write..write_fmt..Adapter$LT$std..sys..wasi..stdio..Stderr$GT$$GT$17h7b29167fe273dc74E (;13;) (type 0) (param i32)
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
  (func $_ZN4core9panicking13assert_failed17h3e38dd4287751d1bE (;14;) (type 8) (param i32 i32 i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 5
    global.set $__stack_pointer
    local.get 5
    local.get 2
    i32.store offset=12
    local.get 5
    local.get 1
    i32.store offset=8
    local.get 0
    local.get 5
    i32.const 8
    i32.add
    i32.const 1048936
    local.get 5
    i32.const 12
    i32.add
    i32.const 1048936
    local.get 3
    local.get 4
    call $_ZN4core9panicking19assert_failed_inner17h028fb57387c98e3fE
    unreachable
  )
  (func $_ZN5alloc11collections9vec_deque21VecDeque$LT$T$C$A$GT$4grow17h528733cfd2a529d5E.llvm.2107575808539744562 (;15;) (type 0) (param i32)
    (local i32 i32 i32 i32 i32)
    local.get 0
    local.get 0
    i32.load
    local.tee 1
    call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17h1b7828678c283b4cE
    block ;; label = @1
      local.get 0
      i32.load offset=8
      local.tee 2
      local.get 1
      local.get 0
      i32.load offset=12
      local.tee 3
      i32.sub
      i32.le_u
      br_if 0 (;@1;)
      local.get 0
      i32.load
      local.set 4
      block ;; label = @2
        block ;; label = @3
          local.get 1
          local.get 2
          i32.sub
          local.tee 5
          local.get 3
          local.get 5
          i32.sub
          local.tee 3
          i32.le_u
          br_if 0 (;@3;)
          local.get 4
          local.get 1
          i32.sub
          local.get 3
          i32.ge_u
          br_if 1 (;@2;)
        end
        local.get 0
        i32.load offset=4
        local.tee 1
        local.get 4
        local.get 5
        i32.sub
        local.tee 3
        i32.const 2
        i32.shl
        i32.add
        local.get 1
        local.get 2
        i32.const 2
        i32.shl
        i32.add
        local.get 5
        i32.const 2
        i32.shl
        call $memmove
        drop
        local.get 0
        local.get 3
        i32.store offset=8
        return
      end
      local.get 0
      i32.load offset=4
      local.tee 0
      local.get 1
      i32.const 2
      i32.shl
      i32.add
      local.get 0
      local.get 3
      i32.const 2
      i32.shl
      call $memcpy
      drop
    end
  )
  (func $_ZN76_$LT$std..sync..poison..PoisonError$LT$T$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17h1b50841a791d287fE (;16;) (type 2) (param i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    i32.const 8
    i32.add
    local.get 1
    i32.const 1048976
    i32.const 11
    call $_ZN4core3fmt9Formatter12debug_struct17h8dcb4ffc7ee470c3E
    local.get 2
    i32.const 8
    i32.add
    call $_ZN4core3fmt8builders11DebugStruct21finish_non_exhaustive17h99ed4ae75088b9afE
    local.set 1
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 1
  )
  (func $_ZN80_$LT$std..io..Write..write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17h7a631dd30797a730E (;17;) (type 4) (param i32 i32 i32) (result i32)
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
    call $_ZN3std2io5Write9write_all17h686446650d647b1cE
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
  (func $_ZN98_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter..SpecFromIter$LT$T$C$I$GT$$GT$9from_iter17hf38725006e016fdaE (;18;) (type 1) (param i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 1
    i32.load offset=4
    local.set 3
    local.get 1
    i32.load
    local.set 4
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            local.get 1
            i32.load offset=16
            local.tee 5
            i32.eqz
            br_if 0 (;@4;)
            block ;; label = @5
              local.get 1
              i32.const 12
              i32.add
              i32.load
              local.tee 6
              local.get 1
              i32.load offset=8
              local.tee 7
              i32.sub
              i32.const 2
              i32.shr_u
              local.get 3
              local.get 4
              i32.sub
              i32.const 2
              i32.shr_u
              i32.add
              local.tee 1
              local.get 5
              local.get 1
              local.get 5
              i32.lt_u
              select
              local.tee 1
              br_if 0 (;@5;)
              i32.const 4
              local.set 8
              br 3 (;@2;)
            end
            local.get 1
            i32.const 536870911
            i32.gt_u
            br_if 1 (;@3;)
            local.get 1
            i32.const 2
            i32.shl
            local.tee 9
            i32.const -1
            i32.le_s
            br_if 1 (;@3;)
            i32.const 0
            i32.load8_u offset=1063681
            drop
            local.get 9
            i32.const 4
            call $__rust_alloc
            local.tee 8
            br_if 2 (;@2;)
            i32.const 4
            local.get 9
            call $_ZN5alloc5alloc18handle_alloc_error17h722a53e8070ff93aE
            unreachable
          end
          local.get 2
          i64.const 17179869184
          i64.store offset=4 align=4
          local.get 1
          i32.load offset=12
          local.set 6
          local.get 1
          i32.load offset=8
          local.set 7
          i32.const 0
          local.set 10
          i32.const 4
          local.set 8
          br 2 (;@1;)
        end
        call $_ZN5alloc7raw_vec17capacity_overflow17h5006d534427a0a2bE
        unreachable
      end
      i32.const 0
      local.set 10
      local.get 2
      i32.const 0
      i32.store offset=12
      local.get 2
      local.get 8
      i32.store offset=8
      local.get 2
      local.get 1
      i32.store offset=4
      local.get 1
      local.get 1
      i32.ge_u
      br_if 0 (;@1;)
      local.get 2
      i32.const 4
      i32.add
      i32.const 0
      local.get 1
      call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17ha32478853dfe9654E
      local.get 2
      i32.load offset=8
      local.set 8
      local.get 2
      i32.load offset=12
      local.set 10
    end
    block ;; label = @1
      local.get 5
      local.get 6
      local.get 7
      i32.sub
      i32.const 2
      i32.shr_u
      local.get 3
      local.get 4
      i32.sub
      i32.const 2
      i32.shr_u
      local.tee 9
      i32.add
      local.tee 1
      local.get 5
      local.get 1
      i32.lt_u
      select
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      i32.const 1
      i32.and
      local.set 11
      block ;; label = @2
        block ;; label = @3
          local.get 1
          i32.const 1
          i32.ne
          br_if 0 (;@3;)
          i32.const 0
          local.set 1
          br 1 (;@2;)
        end
        local.get 1
        i32.const 2147483646
        i32.and
        local.set 12
        local.get 7
        local.get 9
        i32.const 2
        i32.shl
        i32.sub
        local.set 5
        local.get 8
        local.get 10
        i32.const 2
        i32.shl
        i32.add
        local.set 3
        i32.const 0
        local.set 1
        local.get 4
        local.set 6
        loop ;; label = @3
          local.get 3
          local.get 6
          local.get 5
          local.get 9
          local.get 1
          i32.gt_u
          select
          f32.load
          f32.store
          local.get 3
          i32.const 4
          i32.add
          local.get 6
          local.get 5
          local.get 9
          local.get 1
          i32.const 1
          i32.add
          i32.gt_u
          select
          i32.const 4
          i32.add
          f32.load
          f32.store
          local.get 5
          i32.const 8
          i32.add
          local.set 5
          local.get 6
          i32.const 8
          i32.add
          local.set 6
          local.get 3
          i32.const 8
          i32.add
          local.set 3
          local.get 1
          i32.const 2
          i32.add
          local.tee 1
          local.get 12
          i32.ne
          br_if 0 (;@3;)
        end
        local.get 10
        local.get 1
        i32.add
        local.set 10
      end
      local.get 11
      i32.eqz
      br_if 0 (;@1;)
      local.get 8
      local.get 10
      i32.const 2
      i32.shl
      i32.add
      local.get 4
      local.get 1
      i32.const 2
      i32.shl
      i32.add
      local.get 7
      local.get 1
      local.get 9
      i32.sub
      i32.const 2
      i32.shl
      i32.add
      local.get 1
      local.get 9
      i32.lt_u
      select
      f32.load
      f32.store
      local.get 10
      i32.const 1
      i32.add
      local.set 10
    end
    local.get 0
    local.get 2
    i64.load offset=4 align=4
    i64.store align=4
    local.get 0
    i32.const 8
    i32.add
    local.get 10
    i32.store
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
  )
  (func $_ZN4core3ptr161drop_in_place$LT$std..sync..poison..PoisonError$LT$std..sync..rwlock..RwLockReadGuard$LT$core..option..Option$LT$skyapex..spec_check..SpecChecker$GT$$GT$$GT$$GT$17hc6e2ea90168da319E (;19;) (type 0) (param i32)
    local.get 0
    i32.load offset=4
    local.tee 0
    local.get 0
    i32.load
    i32.const -1
    i32.add
    i32.store
  )
  (func $_ZN4core3ptr162drop_in_place$LT$std..sync..poison..PoisonError$LT$std..sync..rwlock..RwLockWriteGuard$LT$core..option..Option$LT$skyapex..spec_check..SpecChecker$GT$$GT$$GT$$GT$17ha38c2f9a20d04a09E (;20;) (type 0) (param i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    local.get 0
    i32.load
    local.set 2
    block ;; label = @1
      local.get 0
      i32.load8_u offset=4
      br_if 0 (;@1;)
      i32.const 0
      i32.load offset=1063748
      i32.const 2147483647
      i32.and
      i32.eqz
      br_if 0 (;@1;)
      call $_ZN3std9panicking11panic_count17is_zero_slow_path17hcdb989fd2253dff7E
      br_if 0 (;@1;)
      local.get 2
      i32.const 1
      i32.store8 offset=4
    end
    local.get 1
    local.get 2
    i32.load
    i32.store offset=4
    local.get 2
    i32.const 0
    i32.store
    block ;; label = @1
      local.get 1
      i32.load offset=4
      i32.const -1
      i32.eq
      br_if 0 (;@1;)
      local.get 1
      i32.const 0
      i32.store offset=8
      i32.const 0
      local.get 1
      i32.const 4
      i32.add
      i32.const 1049912
      local.get 1
      i32.const 8
      i32.add
      i32.const 1050020
      call $_ZN4core9panicking13assert_failed17h3e38dd4287751d1bE
      unreachable
    end
    local.get 1
    i32.const 32
    i32.add
    global.set $__stack_pointer
  )
  (func $_ZN4core3ptr81drop_in_place$LT$core..result..Result$LT$$LP$$RP$$C$std..io..error..Error$GT$$GT$17h87083997074bfe11E (;21;) (type 1) (param i32 i32)
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
  (func $_init_spec_checker (;22;) (type 9) (param i64)
    (local i32 i64 i64 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 64
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        i32.const 0
        i64.load offset=1063784
        i64.eqz
        br_if 0 (;@2;)
        i32.const 0
        i64.load offset=1063800
        local.set 2
        i32.const 0
        i64.load offset=1063792
        local.set 3
        br 1 (;@1;)
      end
      local.get 1
      i32.const 8
      i32.add
      call $_ZN3std3sys4wasi19hashmap_random_keys17hbdc0b948ce1cb89eE
      i32.const 0
      i64.const 1
      i64.store offset=1063784
      i32.const 0
      local.get 1
      i64.load offset=16
      local.tee 2
      i64.store offset=1063800
      local.get 1
      i64.load offset=8
      local.set 3
    end
    i32.const 0
    local.get 3
    i64.const 1
    i64.add
    i64.store offset=1063792
    i32.const 1063600
    local.set 4
    local.get 1
    i32.const 1063600
    i32.store offset=28
    block ;; label = @1
      i32.const 0
      i32.load8_u offset=1063672
      i32.const 3
      i32.eq
      br_if 0 (;@1;)
      local.get 1
      local.get 1
      i32.const 28
      i32.add
      i32.store offset=32
      local.get 1
      local.get 1
      i32.const 32
      i32.add
      i32.store offset=40
      i32.const 1063672
      i32.const 0
      local.get 1
      i32.const 40
      i32.add
      i32.const 1049548
      call $_ZN3std3sys4wasi4once4Once4call17h3585a68f7569d435E
      local.get 1
      i32.load offset=28
      local.set 4
    end
    local.get 4
    i32.load offset=8
    local.set 5
    local.get 4
    i32.const -1
    i32.store offset=8
    block ;; label = @1
      block ;; label = @2
        local.get 5
        br_if 0 (;@2;)
        i32.const 0
        local.set 6
        block ;; label = @3
          i32.const 0
          i32.load offset=1063748
          i32.const 2147483647
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          call $_ZN3std9panicking11panic_count17is_zero_slow_path17hcdb989fd2253dff7E
          i32.const 1
          i32.xor
          local.set 6
        end
        local.get 4
        i32.const 8
        i32.add
        local.set 7
        block ;; label = @3
          local.get 4
          i32.const 12
          i32.add
          i32.load8_u
          br_if 0 (;@3;)
          block ;; label = @4
            local.get 4
            i32.const 32
            i32.add
            i32.load
            i32.const -2147483648
            i32.eq
            br_if 0 (;@4;)
            block ;; label = @5
              local.get 4
              i32.const 48
              i32.add
              i32.load
              local.tee 5
              i32.eqz
              br_if 0 (;@5;)
              local.get 4
              i32.const 44
              i32.add
              i32.load
              local.get 5
              i32.const 2
              i32.shl
              i32.const 11
              i32.add
              i32.const -8
              i32.and
              local.tee 8
              i32.sub
              local.get 5
              local.get 8
              i32.add
              i32.const 9
              i32.add
              i32.const 8
              call $__rust_dealloc
            end
            block ;; label = @5
              local.get 4
              i32.const 40
              i32.add
              i32.load
              local.tee 8
              i32.eqz
              br_if 0 (;@5;)
              local.get 4
              i32.const 36
              i32.add
              i32.load
              local.set 5
              loop ;; label = @6
                block ;; label = @7
                  local.get 5
                  i32.load
                  local.tee 9
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 5
                  i32.const 4
                  i32.add
                  i32.load
                  local.get 9
                  i32.const 2
                  i32.shl
                  i32.const 4
                  call $__rust_dealloc
                end
                local.get 5
                i32.const 32
                i32.add
                local.set 5
                local.get 8
                i32.const -1
                i32.add
                local.tee 8
                br_if 0 (;@6;)
              end
            end
            local.get 4
            i32.load offset=32
            local.tee 5
            i32.eqz
            br_if 0 (;@4;)
            local.get 4
            i32.load offset=36
            local.get 5
            i32.const 5
            i32.shl
            i32.const 8
            call $__rust_dealloc
          end
          local.get 4
          i64.const 34359738368
          i64.store offset=32
          local.get 4
          i32.const 40
          i32.add
          i32.const 0
          i32.store
          local.get 4
          i32.const 24
          i32.add
          local.get 2
          i64.store
          local.get 4
          i32.const 16
          i32.add
          local.get 3
          i64.store
          local.get 4
          i32.const 44
          i32.add
          i32.const 0
          i64.load offset=1049216
          i64.store align=4
          local.get 4
          i32.const 52
          i32.add
          i32.const 0
          i64.load offset=1049224
          i64.store align=4
          local.get 4
          i32.const 64
          i32.add
          local.get 0
          i64.store
          block ;; label = @4
            local.get 6
            br_if 0 (;@4;)
            i32.const 0
            i32.load offset=1063748
            i32.const 2147483647
            i32.and
            i32.eqz
            br_if 0 (;@4;)
            call $_ZN3std9panicking11panic_count17is_zero_slow_path17hcdb989fd2253dff7E
            br_if 0 (;@4;)
            local.get 4
            i32.const 1
            i32.store8 offset=12
          end
          local.get 7
          i32.load
          local.set 5
          local.get 7
          i32.const 0
          i32.store
          local.get 1
          local.get 5
          i32.store offset=32
          local.get 5
          i32.const -1
          i32.ne
          br_if 2 (;@1;)
          local.get 1
          i32.const 64
          i32.add
          global.set $__stack_pointer
          return
        end
        local.get 1
        local.get 6
        i32.store8 offset=44
        local.get 1
        local.get 7
        i32.store offset=40
        i32.const 1049232
        i32.const 43
        local.get 1
        i32.const 40
        i32.add
        i32.const 1049276
        i32.const 1049312
        call $_ZN4core6result13unwrap_failed17h7812484c33dfa842E
        unreachable
      end
      local.get 1
      i32.const 52
      i32.add
      i64.const 0
      i64.store align=4
      local.get 1
      i32.const 1
      i32.store offset=44
      local.get 1
      i32.const 1049200
      i32.store offset=40
      local.get 1
      local.get 1
      i32.const 28
      i32.add
      i32.store offset=48
      local.get 1
      i32.const 32
      i32.add
      local.get 1
      i32.const 28
      i32.add
      local.get 1
      i32.const 40
      i32.add
      call $_ZN3std2io5Write9write_fmt17ha8e89627b8866681E
      local.get 1
      i32.load8_u offset=32
      local.get 1
      i32.load offset=36
      call $_ZN4core3ptr81drop_in_place$LT$core..result..Result$LT$$LP$$RP$$C$std..io..error..Error$GT$$GT$17h87083997074bfe11E
      call $_ZN3std3sys4wasi14abort_internal17h97c20099b339f774E
      unreachable
    end
    local.get 1
    i32.const 0
    i32.store offset=40
    i32.const 0
    local.get 1
    i32.const 32
    i32.add
    i32.const 1049912
    local.get 1
    i32.const 40
    i32.add
    i32.const 1050020
    call $_ZN4core9panicking13assert_failed17h3e38dd4287751d1bE
    unreachable
  )
  (func $_tick_yew (;23;) (type 10) (param i64 f32)
    (local i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    i32.const 1063600
    local.set 3
    local.get 2
    i32.const 1063600
    i32.store offset=12
    block ;; label = @1
      i32.const 0
      i32.load8_u offset=1063672
      i32.const 3
      i32.eq
      br_if 0 (;@1;)
      local.get 2
      local.get 2
      i32.const 12
      i32.add
      i32.store offset=16
      local.get 2
      local.get 2
      i32.const 16
      i32.add
      i32.store offset=24
      i32.const 1063672
      i32.const 0
      local.get 2
      i32.const 24
      i32.add
      i32.const 1049548
      call $_ZN3std3sys4wasi4once4Once4call17h3585a68f7569d435E
      local.get 2
      i32.load offset=12
      local.set 3
    end
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                local.get 3
                i32.load offset=8
                local.tee 4
                i32.const -1
                i32.le_s
                br_if 0 (;@6;)
                local.get 3
                local.get 4
                i32.const 1
                i32.add
                i32.store offset=8
                local.get 3
                i32.const 12
                i32.add
                i32.load8_u
                br_if 1 (;@5;)
                local.get 3
                local.get 4
                i32.store offset=8
                block ;; label = @7
                  local.get 3
                  i32.const 32
                  i32.add
                  i32.load
                  i32.const -2147483648
                  i32.eq
                  br_if 0 (;@7;)
                  i32.const 1063600
                  local.set 4
                  local.get 2
                  i32.const 1063600
                  i32.store offset=12
                  block ;; label = @8
                    i32.const 0
                    i32.load8_u offset=1063672
                    i32.const 3
                    i32.eq
                    br_if 0 (;@8;)
                    local.get 2
                    local.get 2
                    i32.const 12
                    i32.add
                    i32.store offset=16
                    local.get 2
                    local.get 2
                    i32.const 16
                    i32.add
                    i32.store offset=24
                    i32.const 1063672
                    i32.const 0
                    local.get 2
                    i32.const 24
                    i32.add
                    i32.const 1049548
                    call $_ZN3std3sys4wasi4once4Once4call17h3585a68f7569d435E
                    local.get 2
                    i32.load offset=12
                    local.set 4
                  end
                  local.get 4
                  i32.load offset=8
                  local.set 3
                  local.get 4
                  i32.const -1
                  i32.store offset=8
                  local.get 3
                  br_if 3 (;@4;)
                  i32.const 0
                  local.set 5
                  block ;; label = @8
                    i32.const 0
                    i32.load offset=1063748
                    i32.const 2147483647
                    i32.and
                    i32.eqz
                    br_if 0 (;@8;)
                    call $_ZN3std9panicking11panic_count17is_zero_slow_path17hcdb989fd2253dff7E
                    i32.const 1
                    i32.xor
                    local.set 5
                  end
                  local.get 4
                  i32.const 8
                  i32.add
                  local.set 6
                  local.get 4
                  i32.const 12
                  i32.add
                  i32.load8_u
                  br_if 4 (;@3;)
                  local.get 4
                  i32.const 32
                  i32.add
                  i32.load
                  i32.const -2147483648
                  i32.eq
                  br_if 5 (;@2;)
                  local.get 2
                  i32.const 24
                  i32.add
                  local.get 4
                  i32.const 16
                  i32.add
                  local.get 0
                  call $_ZN8indexmap3map25IndexMap$LT$K$C$V$C$S$GT$5entry17hb87923ab1c041b7aE
                  block ;; label = @8
                    local.get 2
                    i32.const 24
                    i32.add
                    call $_ZN8indexmap3map4core18Entry$LT$K$C$V$GT$14or_insert_with17h711c8f6d7c432ce2E
                    local.tee 3
                    i32.load offset=12
                    local.tee 7
                    local.get 3
                    i32.load
                    local.tee 8
                    i32.ne
                    br_if 0 (;@8;)
                    local.get 3
                    call $_ZN5alloc11collections9vec_deque21VecDeque$LT$T$C$A$GT$4grow17h528733cfd2a529d5E.llvm.2107575808539744562
                    local.get 3
                    i32.load
                    local.set 8
                    local.get 3
                    i32.load offset=12
                    local.set 7
                  end
                  local.get 3
                  i32.load offset=4
                  local.get 3
                  i32.load offset=8
                  local.get 7
                  i32.add
                  local.tee 7
                  i32.const 0
                  local.get 8
                  local.get 7
                  local.get 8
                  i32.lt_u
                  select
                  i32.sub
                  i32.const 2
                  i32.shl
                  i32.add
                  local.get 1
                  f32.store
                  local.get 3
                  local.get 3
                  i32.load offset=12
                  local.tee 7
                  i32.const 1
                  i32.add
                  local.tee 8
                  i32.store offset=12
                  block ;; label = @8
                    local.get 8
                    i32.const 51
                    i32.lt_u
                    br_if 0 (;@8;)
                    local.get 3
                    local.get 7
                    i32.store offset=12
                    local.get 3
                    local.get 3
                    i32.load offset=8
                    i32.const 1
                    i32.add
                    local.tee 8
                    i32.const 0
                    local.get 3
                    i32.load
                    local.tee 7
                    local.get 8
                    local.get 7
                    i32.lt_u
                    select
                    i32.sub
                    i32.store offset=8
                  end
                  block ;; label = @8
                    local.get 5
                    br_if 0 (;@8;)
                    i32.const 0
                    i32.load offset=1063748
                    i32.const 2147483647
                    i32.and
                    i32.eqz
                    br_if 0 (;@8;)
                    call $_ZN3std9panicking11panic_count17is_zero_slow_path17hcdb989fd2253dff7E
                    br_if 0 (;@8;)
                    local.get 4
                    i32.const 1
                    i32.store8 offset=12
                  end
                  local.get 6
                  i32.load
                  local.set 3
                  local.get 6
                  i32.const 0
                  i32.store
                  local.get 2
                  local.get 3
                  i32.store offset=16
                  local.get 3
                  i32.const -1
                  i32.ne
                  br_if 6 (;@1;)
                end
                local.get 2
                i32.const 48
                i32.add
                global.set $__stack_pointer
                return
              end
              local.get 2
              i32.const 36
              i32.add
              i64.const 0
              i64.store align=4
              local.get 2
              i32.const 1
              i32.store offset=28
              local.get 2
              i32.const 1049144
              i32.store offset=24
              local.get 2
              local.get 2
              i32.const 12
              i32.add
              i32.store offset=32
              local.get 2
              i32.const 16
              i32.add
              local.get 2
              i32.const 12
              i32.add
              local.get 2
              i32.const 24
              i32.add
              call $_ZN3std2io5Write9write_fmt17ha8e89627b8866681E
              local.get 2
              i32.load8_u offset=16
              local.get 2
              i32.load offset=20
              call $_ZN4core3ptr81drop_in_place$LT$core..result..Result$LT$$LP$$RP$$C$std..io..error..Error$GT$$GT$17h87083997074bfe11E
              call $_ZN3std3sys4wasi14abort_internal17h97c20099b339f774E
              unreachable
            end
            local.get 2
            local.get 3
            i32.const 8
            i32.add
            i32.store offset=28
            local.get 2
            local.get 3
            i32.const 16
            i32.add
            i32.store offset=24
            i32.const 1049232
            i32.const 43
            local.get 2
            i32.const 24
            i32.add
            i32.const 1049404
            i32.const 1049420
            call $_ZN4core6result13unwrap_failed17h7812484c33dfa842E
            unreachable
          end
          local.get 2
          i32.const 36
          i32.add
          i64.const 0
          i64.store align=4
          local.get 2
          i32.const 1
          i32.store offset=28
          local.get 2
          i32.const 1049200
          i32.store offset=24
          local.get 2
          local.get 2
          i32.const 12
          i32.add
          i32.store offset=32
          local.get 2
          i32.const 16
          i32.add
          local.get 2
          i32.const 12
          i32.add
          local.get 2
          i32.const 24
          i32.add
          call $_ZN3std2io5Write9write_fmt17ha8e89627b8866681E
          local.get 2
          i32.load8_u offset=16
          local.get 2
          i32.load offset=20
          call $_ZN4core3ptr81drop_in_place$LT$core..result..Result$LT$$LP$$RP$$C$std..io..error..Error$GT$$GT$17h87083997074bfe11E
          call $_ZN3std3sys4wasi14abort_internal17h97c20099b339f774E
          unreachable
        end
        local.get 2
        local.get 5
        i32.store8 offset=28
        local.get 2
        local.get 6
        i32.store offset=24
        i32.const 1049232
        i32.const 43
        local.get 2
        i32.const 24
        i32.add
        i32.const 1049276
        i32.const 1049388
        call $_ZN4core6result13unwrap_failed17h7812484c33dfa842E
        unreachable
      end
      i32.const 1049328
      i32.const 43
      i32.const 1049372
      call $_ZN4core9panicking5panic17h5f3201ae514c7bcbE
      unreachable
    end
    local.get 2
    i32.const 0
    i32.store offset=24
    i32.const 0
    local.get 2
    i32.const 16
    i32.add
    i32.const 1049912
    local.get 2
    i32.const 24
    i32.add
    i32.const 1050020
    call $_ZN4core9panicking13assert_failed17h3e38dd4287751d1bE
    unreachable
  )
  (func $_check_spec (;24;) (type 11) (param i64) (result f32)
    (local i32 i32 i32 i32 i32 f32 i32 i32 i32 i32 i32 i32 f32 f32 f32 f32)
    global.get $__stack_pointer
    i32.const 80
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    i32.const 1063600
    local.set 2
    local.get 1
    i32.const 1063600
    i32.store offset=68
    block ;; label = @1
      i32.const 0
      i32.load8_u offset=1063672
      i32.const 3
      i32.eq
      br_if 0 (;@1;)
      local.get 1
      local.get 1
      i32.const 68
      i32.add
      i32.store offset=48
      local.get 1
      local.get 1
      i32.const 48
      i32.add
      i32.store
      i32.const 1063672
      i32.const 0
      local.get 1
      i32.const 1049548
      call $_ZN3std3sys4wasi4once4Once4call17h3585a68f7569d435E
      local.get 1
      i32.load offset=68
      local.set 2
    end
    block ;; label = @1
      block ;; label = @2
        local.get 2
        i32.load offset=8
        local.tee 3
        i32.const -1
        i32.le_s
        br_if 0 (;@2;)
        local.get 2
        i32.const 8
        i32.add
        local.set 4
        local.get 2
        local.get 3
        i32.const 1
        i32.add
        i32.store offset=8
        local.get 2
        i32.const 16
        i32.add
        local.set 5
        local.get 2
        i32.const 12
        i32.add
        i32.load8_u
        br_if 1 (;@1;)
        f32.const 0x0p+0 (;=0;)
        local.set 6
        block ;; label = @3
          local.get 2
          i32.const 32
          i32.add
          i32.load
          i32.const -2147483648
          i32.eq
          br_if 0 (;@3;)
          local.get 1
          local.get 0
          i64.store offset=24
          f32.const 0x0p+0 (;=0;)
          local.set 6
          block ;; label = @4
            local.get 5
            local.get 1
            i32.const 24
            i32.add
            call $_ZN8indexmap3map25IndexMap$LT$K$C$V$C$S$GT$3get17h0db345b1678808f3E
            local.tee 3
            i32.eqz
            br_if 0 (;@4;)
            local.get 5
            local.get 2
            i32.const 64
            i32.add
            call $_ZN8indexmap3map25IndexMap$LT$K$C$V$C$S$GT$3get17h0db345b1678808f3E
            local.tee 2
            i32.eqz
            br_if 0 (;@4;)
            local.get 3
            i32.load offset=12
            local.tee 5
            local.get 2
            i32.load offset=12
            local.tee 7
            local.get 5
            local.get 7
            i32.lt_u
            select
            local.set 8
            i32.const 0
            local.set 9
            i32.const 0
            local.set 10
            i32.const 0
            local.set 7
            i32.const 0
            local.set 11
            block ;; label = @5
              local.get 5
              i32.eqz
              br_if 0 (;@5;)
              i32.const 0
              local.set 11
              block ;; label = @6
                local.get 5
                local.get 3
                i32.load
                local.tee 7
                local.get 3
                i32.load offset=8
                local.tee 10
                i32.const 0
                local.get 7
                local.get 10
                local.get 7
                i32.lt_u
                select
                i32.sub
                local.tee 10
                i32.sub
                local.tee 12
                i32.le_u
                br_if 0 (;@6;)
                local.get 5
                local.get 12
                i32.sub
                local.set 11
                br 1 (;@5;)
              end
              local.get 10
              local.get 5
              i32.add
              local.set 7
            end
            local.get 1
            i32.const 48
            i32.add
            i32.const 12
            i32.add
            local.get 3
            i32.load offset=4
            local.tee 3
            local.get 11
            i32.const 2
            i32.shl
            i32.add
            i32.store
            local.get 1
            local.get 8
            i32.store offset=64
            local.get 1
            local.get 3
            i32.store offset=56
            local.get 1
            local.get 3
            local.get 7
            i32.const 2
            i32.shl
            i32.add
            i32.store offset=52
            local.get 1
            local.get 3
            local.get 10
            i32.const 2
            i32.shl
            i32.add
            i32.store offset=48
            local.get 1
            i32.const 36
            i32.add
            local.get 1
            i32.const 48
            i32.add
            call $_ZN98_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter..SpecFromIter$LT$T$C$I$GT$$GT$9from_iter17hf38725006e016fdaE
            i32.const 0
            local.set 3
            i32.const 0
            local.set 5
            block ;; label = @5
              local.get 2
              i32.load offset=12
              local.tee 7
              i32.eqz
              br_if 0 (;@5;)
              i32.const 0
              local.set 5
              block ;; label = @6
                local.get 7
                local.get 2
                i32.load
                local.tee 3
                local.get 2
                i32.load offset=8
                local.tee 9
                i32.const 0
                local.get 3
                local.get 9
                local.get 3
                i32.lt_u
                select
                i32.sub
                local.tee 9
                i32.sub
                local.tee 10
                i32.le_u
                br_if 0 (;@6;)
                local.get 7
                local.get 10
                i32.sub
                local.set 5
                br 1 (;@5;)
              end
              local.get 9
              local.get 7
              i32.add
              local.set 3
            end
            local.get 1
            i32.const 12
            i32.add
            local.get 2
            i32.load offset=4
            local.tee 2
            local.get 5
            i32.const 2
            i32.shl
            i32.add
            i32.store
            local.get 1
            local.get 8
            i32.store offset=16
            local.get 1
            local.get 2
            i32.store offset=8
            local.get 1
            local.get 2
            local.get 3
            i32.const 2
            i32.shl
            i32.add
            i32.store offset=4
            local.get 1
            local.get 2
            local.get 9
            i32.const 2
            i32.shl
            i32.add
            i32.store
            local.get 1
            i32.const 68
            i32.add
            local.get 1
            call $_ZN98_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter..SpecFromIter$LT$T$C$I$GT$$GT$9from_iter17hf38725006e016fdaE
            f32.const 0x0p+0 (;=0;)
            local.set 6
            local.get 1
            i32.load offset=40
            local.set 10
            f32.const 0x0p+0 (;=0;)
            local.set 13
            block ;; label = @5
              local.get 1
              i32.load offset=44
              local.tee 8
              i32.eqz
              br_if 0 (;@5;)
              local.get 8
              i32.const 3
              i32.and
              local.set 3
              block ;; label = @6
                block ;; label = @7
                  local.get 8
                  i32.const 4
                  i32.ge_u
                  br_if 0 (;@7;)
                  i32.const 0
                  local.set 5
                  f32.const 0x0p+0 (;=0;)
                  local.set 13
                  br 1 (;@6;)
                end
                local.get 8
                i32.const -4
                i32.and
                local.set 7
                i32.const 0
                local.set 5
                f32.const 0x0p+0 (;=0;)
                local.set 13
                local.get 10
                local.set 2
                loop ;; label = @7
                  local.get 13
                  local.get 2
                  f32.load
                  f32.add
                  local.get 2
                  i32.const 4
                  i32.add
                  f32.load
                  f32.add
                  local.get 2
                  i32.const 8
                  i32.add
                  f32.load
                  f32.add
                  local.get 2
                  i32.const 12
                  i32.add
                  f32.load
                  f32.add
                  local.set 13
                  local.get 2
                  i32.const 16
                  i32.add
                  local.set 2
                  local.get 7
                  local.get 5
                  i32.const 4
                  i32.add
                  local.tee 5
                  i32.ne
                  br_if 0 (;@7;)
                end
              end
              local.get 3
              i32.eqz
              br_if 0 (;@5;)
              local.get 10
              local.get 5
              i32.const 2
              i32.shl
              i32.add
              local.set 2
              loop ;; label = @6
                local.get 13
                local.get 2
                f32.load
                f32.add
                local.set 13
                local.get 2
                i32.const 4
                i32.add
                local.set 2
                local.get 3
                i32.const -1
                i32.add
                local.tee 3
                br_if 0 (;@6;)
              end
            end
            local.get 8
            f32.convert_i32_u
            local.set 14
            local.get 1
            i32.load offset=72
            local.set 11
            block ;; label = @5
              local.get 1
              i32.load offset=76
              local.tee 9
              i32.eqz
              br_if 0 (;@5;)
              local.get 9
              i32.const 3
              i32.and
              local.set 3
              block ;; label = @6
                block ;; label = @7
                  local.get 9
                  i32.const 4
                  i32.ge_u
                  br_if 0 (;@7;)
                  i32.const 0
                  local.set 5
                  f32.const 0x0p+0 (;=0;)
                  local.set 6
                  br 1 (;@6;)
                end
                local.get 9
                i32.const -4
                i32.and
                local.set 7
                i32.const 0
                local.set 5
                f32.const 0x0p+0 (;=0;)
                local.set 6
                local.get 11
                local.set 2
                loop ;; label = @7
                  local.get 6
                  local.get 2
                  f32.load
                  f32.add
                  local.get 2
                  i32.const 4
                  i32.add
                  f32.load
                  f32.add
                  local.get 2
                  i32.const 8
                  i32.add
                  f32.load
                  f32.add
                  local.get 2
                  i32.const 12
                  i32.add
                  f32.load
                  f32.add
                  local.set 6
                  local.get 2
                  i32.const 16
                  i32.add
                  local.set 2
                  local.get 7
                  local.get 5
                  i32.const 4
                  i32.add
                  local.tee 5
                  i32.ne
                  br_if 0 (;@7;)
                end
              end
              local.get 3
              i32.eqz
              br_if 0 (;@5;)
              local.get 11
              local.get 5
              i32.const 2
              i32.shl
              i32.add
              local.set 2
              loop ;; label = @6
                local.get 6
                local.get 2
                f32.load
                f32.add
                local.set 6
                local.get 2
                i32.const 4
                i32.add
                local.set 2
                local.get 3
                i32.const -1
                i32.add
                local.tee 3
                br_if 0 (;@6;)
              end
            end
            local.get 13
            local.get 14
            f32.div
            local.set 13
            local.get 6
            local.get 9
            f32.convert_i32_u
            f32.div
            local.set 6
            f32.const 0x0p+0 (;=0;)
            local.set 14
            f32.const 0x0p+0 (;=0;)
            local.set 15
            block ;; label = @5
              local.get 8
              local.get 9
              local.get 8
              local.get 9
              i32.lt_u
              select
              local.tee 2
              i32.eqz
              br_if 0 (;@5;)
              local.get 2
              i32.const 1
              i32.and
              local.set 12
              block ;; label = @6
                block ;; label = @7
                  local.get 2
                  i32.const 1
                  i32.ne
                  br_if 0 (;@7;)
                  i32.const 0
                  local.set 5
                  f32.const 0x0p+0 (;=0;)
                  local.set 15
                  br 1 (;@6;)
                end
                local.get 2
                i32.const -2
                i32.and
                local.set 7
                i32.const 0
                local.set 5
                f32.const 0x0p+0 (;=0;)
                local.set 15
                local.get 11
                local.set 2
                local.get 10
                local.set 3
                loop ;; label = @7
                  local.get 15
                  local.get 3
                  f32.load
                  local.get 13
                  f32.sub
                  local.get 2
                  f32.load
                  local.get 6
                  f32.sub
                  f32.mul
                  f32.add
                  local.get 3
                  i32.const 4
                  i32.add
                  f32.load
                  local.get 13
                  f32.sub
                  local.get 2
                  i32.const 4
                  i32.add
                  f32.load
                  local.get 6
                  f32.sub
                  f32.mul
                  f32.add
                  local.set 15
                  local.get 2
                  i32.const 8
                  i32.add
                  local.set 2
                  local.get 3
                  i32.const 8
                  i32.add
                  local.set 3
                  local.get 7
                  local.get 5
                  i32.const 2
                  i32.add
                  local.tee 5
                  i32.ne
                  br_if 0 (;@7;)
                end
              end
              local.get 12
              i32.eqz
              br_if 0 (;@5;)
              local.get 15
              local.get 10
              local.get 5
              i32.const 2
              i32.shl
              local.tee 2
              i32.add
              f32.load
              local.get 13
              f32.sub
              local.get 11
              local.get 2
              i32.add
              f32.load
              local.get 6
              f32.sub
              f32.mul
              f32.add
              local.set 15
            end
            block ;; label = @5
              local.get 8
              i32.eqz
              br_if 0 (;@5;)
              local.get 8
              i32.const 3
              i32.and
              local.set 3
              block ;; label = @6
                block ;; label = @7
                  local.get 8
                  i32.const 4
                  i32.ge_u
                  br_if 0 (;@7;)
                  i32.const 0
                  local.set 5
                  f32.const 0x0p+0 (;=0;)
                  local.set 14
                  br 1 (;@6;)
                end
                local.get 8
                i32.const -4
                i32.and
                local.set 7
                i32.const 0
                local.set 5
                f32.const 0x0p+0 (;=0;)
                local.set 14
                local.get 10
                local.set 2
                loop ;; label = @7
                  local.get 14
                  local.get 2
                  f32.load
                  local.get 13
                  f32.sub
                  local.tee 16
                  local.get 16
                  f32.mul
                  f32.add
                  local.get 2
                  i32.const 4
                  i32.add
                  f32.load
                  local.get 13
                  f32.sub
                  local.tee 14
                  local.get 14
                  f32.mul
                  f32.add
                  local.get 2
                  i32.const 8
                  i32.add
                  f32.load
                  local.get 13
                  f32.sub
                  local.tee 14
                  local.get 14
                  f32.mul
                  f32.add
                  local.get 2
                  i32.const 12
                  i32.add
                  f32.load
                  local.get 13
                  f32.sub
                  local.tee 14
                  local.get 14
                  f32.mul
                  f32.add
                  local.set 14
                  local.get 2
                  i32.const 16
                  i32.add
                  local.set 2
                  local.get 7
                  local.get 5
                  i32.const 4
                  i32.add
                  local.tee 5
                  i32.ne
                  br_if 0 (;@7;)
                end
              end
              local.get 3
              i32.eqz
              br_if 0 (;@5;)
              local.get 10
              local.get 5
              i32.const 2
              i32.shl
              i32.add
              local.set 2
              loop ;; label = @6
                local.get 14
                local.get 2
                f32.load
                local.get 13
                f32.sub
                local.tee 16
                local.get 16
                f32.mul
                f32.add
                local.set 14
                local.get 2
                i32.const 4
                i32.add
                local.set 2
                local.get 3
                i32.const -1
                i32.add
                local.tee 3
                br_if 0 (;@6;)
              end
            end
            block ;; label = @5
              block ;; label = @6
                local.get 9
                br_if 0 (;@6;)
                f32.const 0x0p+0 (;=0;)
                local.set 6
                br 1 (;@5;)
              end
              local.get 9
              i32.const 3
              i32.and
              local.set 3
              block ;; label = @6
                block ;; label = @7
                  local.get 9
                  i32.const 4
                  i32.ge_u
                  br_if 0 (;@7;)
                  i32.const 0
                  local.set 5
                  f32.const 0x0p+0 (;=0;)
                  local.set 13
                  br 1 (;@6;)
                end
                local.get 9
                i32.const -4
                i32.and
                local.set 7
                i32.const 0
                local.set 5
                f32.const 0x0p+0 (;=0;)
                local.set 13
                local.get 11
                local.set 2
                loop ;; label = @7
                  local.get 13
                  local.get 2
                  f32.load
                  local.get 6
                  f32.sub
                  local.tee 16
                  local.get 16
                  f32.mul
                  f32.add
                  local.get 2
                  i32.const 4
                  i32.add
                  f32.load
                  local.get 6
                  f32.sub
                  local.tee 13
                  local.get 13
                  f32.mul
                  f32.add
                  local.get 2
                  i32.const 8
                  i32.add
                  f32.load
                  local.get 6
                  f32.sub
                  local.tee 13
                  local.get 13
                  f32.mul
                  f32.add
                  local.get 2
                  i32.const 12
                  i32.add
                  f32.load
                  local.get 6
                  f32.sub
                  local.tee 13
                  local.get 13
                  f32.mul
                  f32.add
                  local.set 13
                  local.get 2
                  i32.const 16
                  i32.add
                  local.set 2
                  local.get 7
                  local.get 5
                  i32.const 4
                  i32.add
                  local.tee 5
                  i32.ne
                  br_if 0 (;@7;)
                end
              end
              block ;; label = @6
                local.get 3
                i32.eqz
                br_if 0 (;@6;)
                local.get 11
                local.get 5
                i32.const 2
                i32.shl
                i32.add
                local.set 2
                loop ;; label = @7
                  local.get 13
                  local.get 2
                  f32.load
                  local.get 6
                  f32.sub
                  local.tee 16
                  local.get 16
                  f32.mul
                  f32.add
                  local.set 13
                  local.get 2
                  i32.const 4
                  i32.add
                  local.set 2
                  local.get 3
                  i32.const -1
                  i32.add
                  local.tee 3
                  br_if 0 (;@7;)
                end
              end
              f32.const 0x0p+0 (;=0;)
              local.set 6
              local.get 14
              f32.const 0x0p+0 (;=0;)
              f32.eq
              br_if 0 (;@5;)
              local.get 13
              f32.const 0x0p+0 (;=0;)
              f32.eq
              br_if 0 (;@5;)
              local.get 15
              local.get 14
              local.get 13
              f32.mul
              f32.sqrt
              f32.div
              local.set 6
            end
            block ;; label = @5
              local.get 1
              i32.load offset=68
              local.tee 2
              i32.eqz
              br_if 0 (;@5;)
              local.get 11
              local.get 2
              i32.const 2
              i32.shl
              i32.const 4
              call $__rust_dealloc
            end
            local.get 1
            i32.load offset=36
            local.tee 2
            i32.eqz
            br_if 0 (;@4;)
            local.get 10
            local.get 2
            i32.const 2
            i32.shl
            i32.const 4
            call $__rust_dealloc
          end
          local.get 4
          i32.load
          i32.const -1
          i32.add
          local.set 3
        end
        local.get 4
        local.get 3
        i32.store
        local.get 1
        i32.const 80
        i32.add
        global.set $__stack_pointer
        local.get 6
        return
      end
      local.get 1
      i32.const 12
      i32.add
      i64.const 0
      i64.store align=4
      local.get 1
      i32.const 1
      i32.store offset=4
      local.get 1
      i32.const 1049144
      i32.store
      local.get 1
      local.get 1
      i32.const 68
      i32.add
      i32.store offset=8
      local.get 1
      i32.const 48
      i32.add
      local.get 1
      i32.const 68
      i32.add
      local.get 1
      call $_ZN3std2io5Write9write_fmt17ha8e89627b8866681E
      local.get 1
      i32.load8_u offset=48
      local.get 1
      i32.load offset=52
      call $_ZN4core3ptr81drop_in_place$LT$core..result..Result$LT$$LP$$RP$$C$std..io..error..Error$GT$$GT$17h87083997074bfe11E
      call $_ZN3std3sys4wasi14abort_internal17h97c20099b339f774E
      unreachable
    end
    local.get 1
    local.get 4
    i32.store offset=4
    local.get 1
    local.get 5
    i32.store
    i32.const 1049232
    i32.const 43
    local.get 1
    i32.const 1049404
    i32.const 1049436
    call $_ZN4core6result13unwrap_failed17h7812484c33dfa842E
    unreachable
  )
  (func $_ZN6obfstr4xref5inner17h13d4727490ca636fE (;25;) (type 2) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.const 1163171164
    i32.add
    local.tee 1
    i32.const 6
    i32.rotl
    local.get 1
    i32.xor
    i32.const 65535
    i32.and
    i32.add
  )
  (func $_ZN8indexmap3map4core18Entry$LT$K$C$V$GT$14or_insert_with17h711c8f6d7c432ce2E (;26;) (type 12) (param i32) (result i32)
    (local i32 i32 i32 i32 i64 i32 i32 i32 i32)
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
              block ;; label = @6
                block ;; label = @7
                  local.get 0
                  i64.load
                  i64.const 0
                  i64.ne
                  br_if 0 (;@7;)
                  local.get 0
                  i32.const 20
                  i32.add
                  i32.load
                  i32.const -4
                  i32.add
                  i32.load
                  local.tee 2
                  local.get 0
                  i32.const 16
                  i32.add
                  i32.load
                  local.tee 3
                  i32.load offset=8
                  local.tee 0
                  i32.ge_u
                  br_if 1 (;@6;)
                  local.get 3
                  i32.const 4
                  i32.add
                  local.set 4
                  br 6 (;@1;)
                end
                local.get 0
                i64.load offset=8
                local.set 5
                local.get 0
                i32.const 20
                i32.add
                i32.load
                local.tee 3
                i32.const 12
                i32.add
                local.get 0
                i32.const 16
                i32.add
                i32.load
                local.tee 6
                i64.extend_i32_u
                local.get 3
                i32.const 24
                i32.add
                i32.load
                local.tee 2
                local.get 3
                i32.load offset=4
                local.get 3
                i32.load offset=8
                call $_ZN9hashbrown3raw5inner21RawTable$LT$T$C$A$GT$6insert17hc002d8abdf0218a4E
                drop
                local.get 3
                i32.const 4
                i32.add
                local.set 4
                local.get 3
                i32.load offset=8
                local.tee 0
                local.get 3
                i32.load
                i32.ne
                br_if 3 (;@3;)
                local.get 3
                i32.const 20
                i32.add
                i32.load
                local.get 3
                i32.load offset=24
                i32.add
                local.tee 7
                i32.const 67108863
                local.get 7
                i32.const 67108863
                i32.lt_u
                select
                local.get 0
                i32.sub
                local.tee 7
                i32.const 1
                i32.le_u
                br_if 1 (;@5;)
                local.get 0
                local.get 7
                i32.add
                local.tee 7
                local.get 0
                i32.lt_u
                br_if 1 (;@5;)
                local.get 7
                i32.const 5
                i32.shl
                local.set 8
                local.get 7
                i32.const 67108864
                i32.lt_u
                i32.const 3
                i32.shl
                local.set 9
                block ;; label = @7
                  block ;; label = @8
                    local.get 0
                    br_if 0 (;@8;)
                    local.get 1
                    i32.const 0
                    i32.store offset=24
                    br 1 (;@7;)
                  end
                  local.get 1
                  i32.const 8
                  i32.store offset=24
                  local.get 1
                  local.get 0
                  i32.const 5
                  i32.shl
                  i32.store offset=28
                  local.get 1
                  local.get 4
                  i32.load
                  i32.store offset=20
                end
                local.get 1
                i32.const 8
                i32.add
                local.get 9
                local.get 8
                local.get 1
                i32.const 20
                i32.add
                call $_ZN5alloc7raw_vec11finish_grow17hbe562a5f1e52356fE.llvm.18277955098383861047
                local.get 1
                i32.load offset=12
                local.set 0
                block ;; label = @7
                  local.get 1
                  i32.load offset=8
                  br_if 0 (;@7;)
                  local.get 3
                  local.get 7
                  i32.store
                  local.get 3
                  local.get 0
                  i32.store offset=4
                  br 4 (;@3;)
                end
                local.get 0
                i32.const -2147483647
                i32.eq
                br_if 3 (;@3;)
                local.get 3
                i32.load
                local.set 0
                local.get 3
                i32.load offset=8
                local.set 7
                br 2 (;@4;)
              end
              local.get 2
              local.get 0
              i32.const 1049080
              call $_ZN4core9panicking18panic_bounds_check17he2ca869b88362af2E
              unreachable
            end
            local.get 0
            local.set 7
          end
          local.get 0
          local.get 7
          i32.sub
          br_if 0 (;@3;)
          local.get 7
          i32.const 1
          i32.add
          local.tee 7
          i32.eqz
          br_if 1 (;@2;)
          local.get 7
          i32.const 5
          i32.shl
          local.set 8
          local.get 7
          i32.const 67108864
          i32.lt_u
          i32.const 3
          i32.shl
          local.set 9
          block ;; label = @4
            block ;; label = @5
              local.get 0
              br_if 0 (;@5;)
              local.get 1
              i32.const 0
              i32.store offset=24
              br 1 (;@4;)
            end
            local.get 1
            i32.const 8
            i32.store offset=24
            local.get 1
            local.get 0
            i32.const 5
            i32.shl
            i32.store offset=28
            local.get 1
            local.get 4
            i32.load
            i32.store offset=20
          end
          local.get 1
          i32.const 8
          i32.add
          local.get 9
          local.get 8
          local.get 1
          i32.const 20
          i32.add
          call $_ZN5alloc7raw_vec11finish_grow17hbe562a5f1e52356fE.llvm.18277955098383861047
          local.get 1
          i32.load offset=12
          local.set 0
          block ;; label = @4
            local.get 1
            i32.load offset=8
            br_if 0 (;@4;)
            local.get 3
            local.get 7
            i32.store
            local.get 3
            local.get 0
            i32.store offset=4
            br 1 (;@3;)
          end
          local.get 0
          i32.const -2147483647
          i32.eq
          br_if 0 (;@3;)
          local.get 0
          i32.eqz
          br_if 1 (;@2;)
          local.get 0
          local.get 1
          i32.const 16
          i32.add
          i32.load
          call $_ZN5alloc5alloc18handle_alloc_error17h722a53e8070ff93aE
          unreachable
        end
        block ;; label = @3
          local.get 3
          i32.load offset=8
          local.tee 0
          local.get 3
          i32.load
          i32.ne
          br_if 0 (;@3;)
          local.get 3
          local.get 0
          call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17h128e9d7724bd49f6E
          local.get 3
          i32.load offset=8
          local.set 0
        end
        local.get 3
        i32.load offset=4
        local.get 0
        i32.const 5
        i32.shl
        i32.add
        local.tee 0
        local.get 6
        i32.store offset=24
        local.get 0
        local.get 5
        i64.store offset=16
        local.get 0
        i64.const 0
        i64.store offset=8
        local.get 0
        i64.const 17179869184
        i64.store
        local.get 3
        local.get 3
        i32.load offset=8
        i32.const 1
        i32.add
        local.tee 0
        i32.store offset=8
        local.get 2
        local.get 0
        i32.lt_u
        br_if 1 (;@1;)
        local.get 2
        local.get 0
        i32.const 1049672
        call $_ZN4core9panicking18panic_bounds_check17he2ca869b88362af2E
        unreachable
      end
      call $_ZN5alloc7raw_vec17capacity_overflow17h5006d534427a0a2bE
      unreachable
    end
    local.get 4
    i32.load
    local.set 3
    local.get 1
    i32.const 32
    i32.add
    global.set $__stack_pointer
    local.get 3
    local.get 2
    i32.const 5
    i32.shl
    i32.add
  )
  (func $load (;27;) (type 13))
  (func $add (;28;) (type 2) (param i32 i32) (result i32)
    local.get 1
    local.get 0
    i32.add
  )
  (func $print_run_as_root (;29;) (type 13)
    (local i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 64
    i32.sub
    local.tee 0
    global.set $__stack_pointer
    local.get 0
    i32.const 1005292
    i32.store offset=4
    local.get 0
    i32.const 4
    i32.add
    local.set 1
    local.get 0
    i32.load offset=4
    local.set 2
    local.get 0
    i32.const -1333232223
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
    call $_ZN6obfstr4xref5inner17h13d4727490ca636fE
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
    i32.const 149
    i32.xor
    i32.store8
    local.get 0
    i32.const 19
    i32.store offset=40
    local.get 0
    i32.const 10
    i32.store offset=32
    local.get 0
    i32.const 2
    i32.store offset=8
    local.get 0
    i32.const 1049692
    i32.store offset=4
    local.get 0
    local.get 2
    i32.const 1936437073
    i32.xor
    i32.store offset=44
    local.get 0
    local.get 3
    i32.const -693105103
    i32.xor
    i32.store offset=48
    local.get 0
    local.get 4
    i32.const 968815818
    i32.xor
    i32.store offset=52
    local.get 0
    local.get 5
    i32.const 64957561
    i32.xor
    i32.store offset=56
    local.get 0
    local.get 6
    i32.const 113
    i32.xor
    i32.store8 offset=60
    local.get 0
    local.get 1
    i32.const 18
    i32.add
    i32.load8_u
    i32.const 95
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
    call $_ZN3std2io5stdio6_print17h713f8969f4ff1952E
    local.get 0
    i32.const 64
    i32.add
    global.set $__stack_pointer
  )
  (func $_ZN5alloc7raw_vec11finish_grow17hbe562a5f1e52356fE.llvm.18277955098383861047 (;30;) (type 7) (param i32 i32 i32 i32)
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
                  i32.load8_u offset=1063681
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
              i32.load8_u offset=1063681
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
  (func $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17h128e9d7724bd49f6E (;31;) (type 1) (param i32 i32)
    (local i32 i32 i32 i32)
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
        i32.const 4
        local.get 1
        i32.const 4
        i32.gt_u
        select
        local.tee 1
        i32.const 5
        i32.shl
        local.set 4
        local.get 1
        i32.const 67108864
        i32.lt_u
        i32.const 3
        i32.shl
        local.set 5
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
          i32.const 8
          i32.store offset=24
          local.get 2
          local.get 3
          i32.const 5
          i32.shl
          i32.store offset=28
          local.get 2
          local.get 0
          i32.load offset=4
          i32.store offset=20
        end
        local.get 2
        i32.const 8
        i32.add
        local.get 5
        local.get 4
        local.get 2
        i32.const 20
        i32.add
        call $_ZN5alloc7raw_vec11finish_grow17hbe562a5f1e52356fE.llvm.18277955098383861047
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
        call $_ZN5alloc5alloc18handle_alloc_error17h722a53e8070ff93aE
        unreachable
      end
      call $_ZN5alloc7raw_vec17capacity_overflow17h5006d534427a0a2bE
      unreachable
    end
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
  )
  (func $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17h1b7828678c283b4cE (;32;) (type 1) (param i32 i32)
    (local i32 i32 i32 i32)
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
        i32.const 4
        local.get 1
        i32.const 4
        i32.gt_u
        select
        local.tee 1
        i32.const 2
        i32.shl
        local.set 4
        local.get 1
        i32.const 536870912
        i32.lt_u
        i32.const 2
        i32.shl
        local.set 5
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
          i32.const 4
          i32.store offset=24
          local.get 2
          local.get 3
          i32.const 2
          i32.shl
          i32.store offset=28
          local.get 2
          local.get 0
          i32.load offset=4
          i32.store offset=20
        end
        local.get 2
        i32.const 8
        i32.add
        local.get 5
        local.get 4
        local.get 2
        i32.const 20
        i32.add
        call $_ZN5alloc7raw_vec11finish_grow17hbe562a5f1e52356fE.llvm.18277955098383861047
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
        call $_ZN5alloc5alloc18handle_alloc_error17h722a53e8070ff93aE
        unreachable
      end
      call $_ZN5alloc7raw_vec17capacity_overflow17h5006d534427a0a2bE
      unreachable
    end
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
  )
  (func $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17ha32478853dfe9654E (;33;) (type 3) (param i32 i32 i32)
    (local i32 i32 i32)
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
        i32.const 4
        local.get 2
        i32.const 4
        i32.gt_u
        select
        local.tee 2
        i32.const 2
        i32.shl
        local.set 4
        local.get 2
        i32.const 536870912
        i32.lt_u
        i32.const 2
        i32.shl
        local.set 5
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
          i32.const 4
          i32.store offset=24
          local.get 3
          local.get 1
          i32.const 2
          i32.shl
          i32.store offset=28
          local.get 3
          local.get 0
          i32.load offset=4
          i32.store offset=20
        end
        local.get 3
        i32.const 8
        i32.add
        local.get 5
        local.get 4
        local.get 3
        i32.const 20
        i32.add
        call $_ZN5alloc7raw_vec11finish_grow17hbe562a5f1e52356fE.llvm.18277955098383861047
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
        call $_ZN5alloc5alloc18handle_alloc_error17h722a53e8070ff93aE
        unreachable
      end
      call $_ZN5alloc7raw_vec17capacity_overflow17h5006d534427a0a2bE
      unreachable
    end
    local.get 3
    i32.const 32
    i32.add
    global.set $__stack_pointer
  )
  (func $_ZN3std2io5Write9write_all17h686446650d647b1cE (;34;) (type 7) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        local.get 3
        i32.eqz
        br_if 0 (;@2;)
        loop ;; label = @3
          local.get 4
          i32.const 8
          i32.add
          local.get 1
          local.get 2
          local.get 3
          call $_ZN64_$LT$std..sys..wasi..stdio..Stderr$u20$as$u20$std..io..Write$GT$5write17h0c145d49840ec5beE
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    block ;; label = @9
                      block ;; label = @10
                        block ;; label = @11
                          local.get 4
                          i32.load8_u offset=8
                          local.tee 5
                          i32.const 4
                          i32.ne
                          br_if 0 (;@11;)
                          local.get 4
                          i32.load offset=12
                          local.tee 5
                          br_if 1 (;@10;)
                          local.get 0
                          i32.const 1049756
                          i32.store offset=4
                          local.get 0
                          i32.const 2
                          i32.store8
                          br 10 (;@1;)
                        end
                        block ;; label = @11
                          block ;; label = @12
                            local.get 5
                            br_table 1 (;@11;) 0 (;@12;) 4 (;@8;) 3 (;@9;) 1 (;@11;)
                          end
                          local.get 4
                          i32.load8_u offset=9
                          i32.const 35
                          i32.ne
                          br_if 6 (;@5;)
                          local.get 4
                          i32.load offset=12
                          local.set 6
                          br 4 (;@7;)
                        end
                        local.get 4
                        i32.load offset=12
                        local.tee 6
                        i32.const 27
                        i32.ne
                        br_if 5 (;@5;)
                        br 3 (;@7;)
                      end
                      local.get 3
                      local.get 5
                      i32.lt_u
                      br_if 3 (;@6;)
                      local.get 2
                      local.get 5
                      i32.add
                      local.set 2
                      local.get 3
                      local.get 5
                      i32.sub
                      local.set 3
                      br 5 (;@4;)
                    end
                    local.get 4
                    i32.load offset=12
                    local.tee 6
                    i32.load8_u offset=8
                    i32.const 35
                    i32.eq
                    br_if 1 (;@7;)
                    br 3 (;@5;)
                  end
                  local.get 4
                  i32.load offset=12
                  local.tee 6
                  i32.load8_u offset=8
                  i32.const 35
                  i32.ne
                  br_if 2 (;@5;)
                end
                local.get 5
                i32.const 3
                i32.lt_u
                br_if 2 (;@4;)
                local.get 6
                i32.load
                local.tee 7
                local.get 6
                i32.const 4
                i32.add
                i32.load
                local.tee 5
                i32.load
                call_indirect (type 0)
                block ;; label = @7
                  local.get 5
                  i32.load offset=4
                  local.tee 8
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 7
                  local.get 8
                  local.get 5
                  i32.load offset=8
                  call $__rust_dealloc
                end
                local.get 6
                i32.const 12
                i32.const 4
                call $__rust_dealloc
                br 2 (;@4;)
              end
              local.get 5
              local.get 3
              i32.const 1049844
              call $_ZN4core5slice5index26slice_start_index_len_fail17h43cf12d46dc03083E
              unreachable
            end
            local.get 0
            local.get 4
            i64.load offset=8
            i64.store align=4
            br 3 (;@1;)
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
    i32.const 16
    i32.add
    global.set $__stack_pointer
  )
  (func $_ZN3std2io5Write9write_fmt17ha8e89627b8866681E (;35;) (type 3) (param i32 i32 i32)
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
        i32.const 1049860
        local.get 2
        call $_ZN4core3fmt5write17h890955524eea605cE
        i32.eqz
        br_if 0 (;@2;)
        block ;; label = @3
          local.get 3
          i32.load8_u
          i32.const 4
          i32.ne
          br_if 0 (;@3;)
          local.get 0
          i32.const 1049900
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
  (func $_ZN4core3ptr92drop_in_place$LT$std..io..Write..write_fmt..Adapter$LT$std..sys..wasi..stdio..Stderr$GT$$GT$17h7b29167fe273dc74E.llvm.13741533063225024593 (;36;) (type 0) (param i32)
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
  (func $_ZN8indexmap3map25IndexMap$LT$K$C$V$C$S$GT$3get17h0db345b1678808f3E (;37;) (type 2) (param i32 i32) (result i32)
    (local i64 i64 i64 i64 i64 i64 i32 i32 i32 i32 i32 i32)
    block ;; label = @1
      local.get 0
      i32.const 40
      i32.add
      i32.load
      br_if 0 (;@1;)
      i32.const 0
      return
    end
    local.get 0
    i64.load offset=8
    local.tee 2
    local.get 1
    i64.load
    local.tee 3
    i64.xor
    i64.const 8387220255154660723
    i64.xor
    local.tee 4
    local.get 0
    i64.load
    local.tee 5
    i64.const 7816392313619706465
    i64.xor
    i64.add
    local.tee 6
    local.get 4
    i64.const 16
    i64.rotl
    i64.xor
    local.tee 4
    i64.const 21
    i64.rotl
    local.get 4
    local.get 2
    i64.const 7237128888997146477
    i64.xor
    local.tee 2
    local.get 5
    i64.const 8317987319222330741
    i64.xor
    i64.add
    local.tee 5
    i64.const 32
    i64.rotl
    i64.add
    local.tee 4
    i64.xor
    i64.const 576460752303423488
    i64.xor
    local.tee 7
    i64.const 16
    i64.rotl
    local.get 7
    local.get 6
    local.get 5
    local.get 2
    i64.const 13
    i64.rotl
    i64.xor
    local.tee 2
    i64.add
    local.tee 5
    i64.const 32
    i64.rotl
    i64.add
    local.tee 6
    i64.xor
    local.tee 7
    i64.const 21
    i64.rotl
    local.get 7
    local.get 4
    local.get 3
    i64.xor
    local.get 5
    local.get 2
    i64.const 17
    i64.rotl
    i64.xor
    local.tee 2
    i64.add
    local.tee 4
    i64.const 32
    i64.rotl
    i64.add
    local.tee 5
    i64.xor
    local.tee 7
    i64.const 16
    i64.rotl
    local.get 7
    local.get 6
    local.get 4
    local.get 2
    i64.const 13
    i64.rotl
    i64.xor
    local.tee 2
    i64.add
    local.tee 4
    i64.const 32
    i64.rotl
    i64.const 255
    i64.xor
    i64.add
    local.tee 6
    i64.xor
    local.tee 7
    i64.const 21
    i64.rotl
    local.get 7
    local.get 5
    i64.const 576460752303423488
    i64.xor
    local.get 4
    local.get 2
    i64.const 17
    i64.rotl
    i64.xor
    local.tee 2
    i64.add
    local.tee 4
    i64.const 32
    i64.rotl
    i64.add
    local.tee 5
    i64.xor
    local.tee 7
    i64.const 16
    i64.rotl
    local.get 7
    local.get 4
    local.get 2
    i64.const 13
    i64.rotl
    i64.xor
    local.tee 2
    local.get 6
    i64.add
    local.tee 4
    i64.const 32
    i64.rotl
    i64.add
    local.tee 6
    i64.xor
    local.tee 7
    i64.const 21
    i64.rotl
    local.get 7
    local.get 4
    local.get 2
    i64.const 17
    i64.rotl
    i64.xor
    local.tee 2
    local.get 5
    i64.add
    local.tee 4
    i64.const 32
    i64.rotl
    i64.add
    local.tee 5
    i64.xor
    local.tee 7
    i64.const 16
    i64.rotl
    local.get 7
    local.get 2
    i64.const 13
    i64.rotl
    local.get 4
    i64.xor
    local.tee 2
    local.get 6
    i64.add
    local.tee 4
    i64.const 32
    i64.rotl
    i64.add
    local.tee 6
    i64.xor
    i64.const 21
    i64.rotl
    local.get 2
    i64.const 17
    i64.rotl
    local.get 4
    i64.xor
    local.tee 2
    i64.const 13
    i64.rotl
    local.get 2
    local.get 5
    i64.add
    i64.xor
    local.tee 2
    i64.const 17
    i64.rotl
    i64.xor
    local.get 2
    local.get 6
    i64.add
    local.tee 2
    i64.const 32
    i64.shr_u
    i64.xor
    local.get 2
    i64.xor
    local.tee 2
    i64.const 25
    i64.shr_u
    i64.const 127
    i64.and
    i64.const 72340172838076673
    i64.mul
    local.set 5
    local.get 2
    i32.wrap_i64
    local.set 8
    local.get 0
    i32.const 28
    i32.add
    i32.load
    local.tee 9
    i32.const -4
    i32.add
    local.set 10
    local.get 0
    i32.const 32
    i32.add
    i32.load
    local.set 1
    local.get 0
    i32.const 24
    i32.add
    i32.load
    local.set 11
    local.get 0
    i32.const 20
    i32.add
    i32.load
    local.set 12
    i32.const 0
    local.set 13
    loop (result i32) ;; label = @1
      local.get 9
      local.get 8
      local.get 1
      i32.and
      local.tee 8
      i32.add
      i64.load align=1
      local.tee 4
      local.get 5
      i64.xor
      local.tee 2
      i64.const -1
      i64.xor
      local.get 2
      i64.const -72340172838076673
      i64.add
      i64.and
      i64.const -9187201950435737472
      i64.and
      local.set 2
      block ;; label = @2
        block ;; label = @3
          loop ;; label = @4
            block ;; label = @5
              local.get 2
              i64.const 0
              i64.ne
              br_if 0 (;@5;)
              local.get 4
              local.get 4
              i64.const 1
              i64.shl
              i64.and
              i64.const -9187201950435737472
              i64.and
              i64.eqz
              br_if 3 (;@2;)
              i32.const 0
              return
            end
            local.get 11
            local.get 10
            local.get 2
            i64.ctz
            i32.wrap_i64
            i32.const 3
            i32.shr_u
            local.get 8
            i32.add
            local.get 1
            i32.and
            i32.const 2
            i32.shl
            i32.sub
            i32.load
            local.tee 0
            i32.le_u
            br_if 1 (;@3;)
            local.get 2
            i64.const -1
            i64.add
            local.get 2
            i64.and
            local.set 2
            local.get 3
            local.get 12
            local.get 0
            i32.const 5
            i32.shl
            i32.add
            i64.load offset=16
            i64.ne
            br_if 0 (;@4;)
          end
          local.get 12
          local.get 0
          i32.const 5
          i32.shl
          i32.add
          return
        end
        local.get 0
        local.get 11
        i32.const 1049656
        call $_ZN4core9panicking18panic_bounds_check17he2ca869b88362af2E
        unreachable
      end
      local.get 8
      local.get 13
      i32.const 8
      i32.add
      local.tee 13
      i32.add
      local.set 8
      br 0 (;@1;)
    end
  )
  (func $_ZN8indexmap3map25IndexMap$LT$K$C$V$C$S$GT$5entry17hb87923ab1c041b7aE (;38;) (type 14) (param i32 i32 i64)
    (local i64 i64 i64 i64 i64 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    local.get 1
    i64.load offset=8
    local.tee 3
    local.get 2
    i64.xor
    i64.const 8387220255154660723
    i64.xor
    local.tee 4
    local.get 1
    i64.load
    local.tee 5
    i64.const 7816392313619706465
    i64.xor
    i64.add
    local.tee 6
    local.get 4
    i64.const 16
    i64.rotl
    i64.xor
    local.tee 4
    local.get 3
    i64.const 7237128888997146477
    i64.xor
    local.tee 3
    local.get 5
    i64.const 8317987319222330741
    i64.xor
    i64.add
    local.tee 5
    i64.const 32
    i64.rotl
    i64.add
    local.tee 7
    local.get 4
    i64.const 21
    i64.rotl
    i64.xor
    i64.const 576460752303423488
    i64.xor
    local.tee 4
    i64.const 16
    i64.rotl
    local.get 4
    local.get 5
    local.get 3
    i64.const 13
    i64.rotl
    i64.xor
    local.tee 3
    local.get 6
    i64.add
    local.tee 5
    i64.const 32
    i64.rotl
    i64.add
    local.tee 4
    i64.xor
    local.tee 6
    i64.const 21
    i64.rotl
    local.get 6
    local.get 7
    local.get 2
    i64.xor
    local.get 5
    local.get 3
    i64.const 17
    i64.rotl
    i64.xor
    local.tee 3
    i64.add
    local.tee 5
    i64.const 32
    i64.rotl
    i64.add
    local.tee 6
    i64.xor
    local.tee 7
    i64.const 16
    i64.rotl
    local.get 7
    local.get 4
    local.get 5
    local.get 3
    i64.const 13
    i64.rotl
    i64.xor
    local.tee 3
    i64.add
    local.tee 4
    i64.const 32
    i64.rotl
    i64.const 255
    i64.xor
    i64.add
    local.tee 5
    i64.xor
    local.tee 7
    i64.const 21
    i64.rotl
    local.get 7
    local.get 6
    i64.const 576460752303423488
    i64.xor
    local.get 4
    local.get 3
    i64.const 17
    i64.rotl
    i64.xor
    local.tee 3
    i64.add
    local.tee 4
    i64.const 32
    i64.rotl
    i64.add
    local.tee 6
    i64.xor
    local.tee 7
    i64.const 16
    i64.rotl
    local.get 7
    local.get 4
    local.get 3
    i64.const 13
    i64.rotl
    i64.xor
    local.tee 3
    local.get 5
    i64.add
    local.tee 4
    i64.const 32
    i64.rotl
    i64.add
    local.tee 5
    i64.xor
    local.tee 7
    i64.const 21
    i64.rotl
    local.get 7
    local.get 4
    local.get 3
    i64.const 17
    i64.rotl
    i64.xor
    local.tee 3
    local.get 6
    i64.add
    local.tee 4
    i64.const 32
    i64.rotl
    i64.add
    local.tee 6
    i64.xor
    local.tee 7
    i64.const 16
    i64.rotl
    local.get 7
    local.get 3
    i64.const 13
    i64.rotl
    local.get 4
    i64.xor
    local.tee 3
    local.get 5
    i64.add
    local.tee 4
    i64.const 32
    i64.rotl
    i64.add
    local.tee 5
    i64.xor
    i64.const 21
    i64.rotl
    local.get 3
    i64.const 17
    i64.rotl
    local.get 4
    i64.xor
    local.tee 3
    i64.const 13
    i64.rotl
    local.get 3
    local.get 6
    i64.add
    i64.xor
    local.tee 3
    i64.const 17
    i64.rotl
    i64.xor
    local.get 3
    local.get 5
    i64.add
    local.tee 3
    i64.const 32
    i64.shr_u
    i64.xor
    local.get 3
    i64.xor
    local.tee 3
    i64.const 25
    i64.shr_u
    i64.const 127
    i64.and
    i64.const 72340172838076673
    i64.mul
    local.set 5
    local.get 1
    i32.const 16
    i32.add
    local.set 8
    local.get 1
    i32.const 28
    i32.add
    i32.load
    local.tee 9
    i32.const -4
    i32.add
    local.set 10
    local.get 1
    i32.const 32
    i32.add
    i32.load
    local.set 11
    local.get 1
    i32.const 24
    i32.add
    i32.load
    local.set 12
    local.get 1
    i32.const 20
    i32.add
    i32.load
    local.set 13
    i32.const 0
    local.set 14
    local.get 3
    i32.wrap_i64
    local.tee 15
    local.set 1
    block ;; label = @1
      block ;; label = @2
        loop ;; label = @3
          local.get 9
          local.get 1
          local.get 11
          i32.and
          local.tee 16
          i32.add
          i64.load align=1
          local.tee 4
          local.get 5
          i64.xor
          local.tee 3
          i64.const -1
          i64.xor
          local.get 3
          i64.const -72340172838076673
          i64.add
          i64.and
          i64.const -9187201950435737472
          i64.and
          local.set 3
          loop ;; label = @4
            block ;; label = @5
              local.get 3
              i64.const 0
              i64.ne
              br_if 0 (;@5;)
              local.get 4
              local.get 4
              i64.const 1
              i64.shl
              i64.and
              i64.const -9187201950435737472
              i64.and
              i64.eqz
              i32.eqz
              br_if 4 (;@1;)
              local.get 16
              local.get 14
              i32.const 8
              i32.add
              local.tee 14
              i32.add
              local.set 1
              br 2 (;@3;)
            end
            local.get 12
            local.get 10
            local.get 3
            i64.ctz
            i32.wrap_i64
            i32.const 3
            i32.shr_u
            local.get 16
            i32.add
            local.get 11
            i32.and
            local.tee 17
            i32.const 2
            i32.shl
            i32.sub
            i32.load
            local.tee 1
            i32.le_u
            br_if 2 (;@2;)
            local.get 3
            i64.const -1
            i64.add
            local.get 3
            i64.and
            local.set 3
            local.get 13
            local.get 1
            i32.const 5
            i32.shl
            i32.add
            i64.load offset=16
            local.get 2
            i64.ne
            br_if 0 (;@4;)
          end
        end
        local.get 0
        local.get 2
        i64.store offset=8
        local.get 0
        i32.const 16
        i32.add
        local.get 8
        i32.store
        local.get 0
        i32.const 20
        i32.add
        local.get 9
        i32.const 0
        local.get 17
        i32.sub
        i32.const 2
        i32.shl
        i32.add
        i32.store
        local.get 0
        i64.const 0
        i64.store
        return
      end
      local.get 1
      local.get 12
      i32.const 1049656
      call $_ZN4core9panicking18panic_bounds_check17he2ca869b88362af2E
      unreachable
    end
    local.get 0
    local.get 2
    i64.store offset=8
    local.get 0
    i32.const 20
    i32.add
    local.get 8
    i32.store
    local.get 0
    i32.const 16
    i32.add
    local.get 15
    i32.store
    local.get 0
    i64.const 1
    i64.store
  )
  (func $__rust_alloc (;39;) (type 2) (param i32 i32) (result i32)
    (local i32)
    local.get 0
    local.get 1
    call $__rdl_alloc
    local.set 2
    local.get 2
    return
  )
  (func $__rust_dealloc (;40;) (type 3) (param i32 i32 i32)
    local.get 0
    local.get 1
    local.get 2
    call $__rdl_dealloc
    return
  )
  (func $__rust_realloc (;41;) (type 5) (param i32 i32 i32 i32) (result i32)
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
  (func $__rust_alloc_error_handler (;42;) (type 1) (param i32 i32)
    local.get 0
    local.get 1
    call $__rg_oom
    return
  )
  (func $_ZN9hashbrown3raw5inner11Fallibility17capacity_overflow17h98ef64c65897ab00E (;43;) (type 1) (param i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block ;; label = @1
      local.get 1
      br_if 0 (;@1;)
      local.get 0
      i32.const 0
      i32.store
      local.get 2
      i32.const 32
      i32.add
      global.set $__stack_pointer
      return
    end
    local.get 2
    i32.const 20
    i32.add
    i64.const 0
    i64.store align=4
    local.get 2
    i32.const 1
    i32.store offset=12
    local.get 2
    i32.const 1050064
    i32.store offset=8
    local.get 2
    i32.const 1050072
    i32.store offset=16
    local.get 2
    i32.const 8
    i32.add
    i32.const 1050164
    call $_ZN4core9panicking9panic_fmt17h4c9c94dcfde83250E
    unreachable
  )
  (func $_ZN9hashbrown3raw5inner11Fallibility9alloc_err17h5d5bef80305c3a93E (;44;) (type 7) (param i32 i32 i32 i32)
    block ;; label = @1
      local.get 1
      br_if 0 (;@1;)
      local.get 0
      local.get 3
      i32.store offset=4
      local.get 0
      local.get 2
      i32.store
      return
    end
    local.get 2
    local.get 3
    call $_ZN5alloc5alloc18handle_alloc_error17h722a53e8070ff93aE
    unreachable
  )
  (func $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h6c8635a3a4fc6b26E (;45;) (type 1) (param i32 i32)
    local.get 0
    i64.const 4507122837358743131
    i64.store offset=8
    local.get 0
    i64.const -2401257079958803507
    i64.store
  )
  (func $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17hca611893b35124d4E (;46;) (type 1) (param i32 i32)
    local.get 0
    i64.const -163230743173927068
    i64.store offset=8
    local.get 0
    i64.const -4493808902380553279
    i64.store
  )
  (func $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17he5809b74c70e51b3E (;47;) (type 1) (param i32 i32)
    local.get 0
    i64.const -6002009914241348162
    i64.store offset=8
    local.get 0
    i64.const 4845729891138127776
    i64.store
  )
  (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h3f68d0fccb7931f2E (;48;) (type 2) (param i32 i32) (result i32)
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
        i32.const 1050576
        i32.const 4
        call $_ZN4core3fmt9Formatter9write_str17h4bf1acaddf72a444E
        local.set 0
        br 1 (;@1;)
      end
      local.get 2
      local.get 0
      i32.const 1
      i32.add
      i32.store offset=12
      local.get 1
      i32.const 1050580
      i32.const 4
      local.get 2
      i32.const 12
      i32.add
      i32.const 1050180
      call $_ZN4core3fmt9Formatter25debug_tuple_field1_finish17h523899074bd3629cE
      local.set 0
    end
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 0
  )
  (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h97c11851c7cd90d9E (;49;) (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 1
    call $_ZN43_$LT$bool$u20$as$u20$core..fmt..Display$GT$3fmt17hd1b099dc6a4f6226E
  )
  (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17he2f815353add60f3E (;50;) (type 2) (param i32 i32) (result i32)
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
        call $_ZN4core3fmt3num3imp51_$LT$impl$u20$core..fmt..Display$u20$for$u20$u8$GT$3fmt17h429bcbf4930d92a2E
        return
      end
      local.get 0
      local.get 1
      call $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i8$GT$3fmt17he59c9ce34d177f0eE
      return
    end
    local.get 0
    local.get 1
    call $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i8$GT$3fmt17he2282454763b40d3E
  )
  (func $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h28f18327876a1111E (;51;) (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 1
    call $_ZN70_$LT$core..panic..location..Location$u20$as$u20$core..fmt..Display$GT$3fmt17h5a1f046e61dd743eE
  )
  (func $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h4935ef6be9273719E (;52;) (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 0
    i32.load offset=4
    local.get 1
    call $_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17h33dd62bf170497e3E
  )
  (func $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17he5cf1af9da646324E (;53;) (type 2) (param i32 i32) (result i32)
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
        call $_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17h66c9e2e594bd720fE
        return
      end
      local.get 0
      local.get 1
      call $_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i32$GT$3fmt17h6acce4b91e41041fE
      return
    end
    local.get 0
    local.get 1
    call $_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i32$GT$3fmt17hff8d8204aa9c9138E
  )
  (func $_ZN4core3fmt5Write10write_char17h906853a4beb697c3E (;54;) (type 2) (param i32 i32) (result i32)
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
      call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17he7f9a7b89ce4867fE
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
  (func $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17he7f9a7b89ce4867fE (;55;) (type 3) (param i32 i32 i32)
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
        call $_ZN5alloc7raw_vec11finish_grow17h0bd40e33419dc061E
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
        call $_ZN5alloc5alloc18handle_alloc_error17h722a53e8070ff93aE
        unreachable
      end
      call $_ZN5alloc7raw_vec17capacity_overflow17h5006d534427a0a2bE
      unreachable
    end
    local.get 3
    i32.const 32
    i32.add
    global.set $__stack_pointer
  )
  (func $_ZN4core3fmt5Write10write_char17hc13b02cd8db1d5f4E (;56;) (type 2) (param i32 i32) (result i32)
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
    call $_ZN61_$LT$std..io..stdio..StdoutLock$u20$as$u20$std..io..Write$GT$9write_all17h0bd9a33f327da1daE
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
  (func $_ZN61_$LT$std..io..stdio..StdoutLock$u20$as$u20$std..io..Write$GT$9write_all17h0bd9a33f327da1daE (;57;) (type 7) (param i32 i32 i32 i32)
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
                call $_ZN4core5slice6memchr7memrchr17hba53988255f8572cE
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
                  call $_ZN4wasi13lib_generated8fd_write17hf49b30128c2714d3E
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
                        i32.const 1050920
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
              i32.const 1051820
              call $_ZN4core4cell22panic_already_borrowed17h813146898ec049ddE
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
                      call $_ZN4wasi13lib_generated8fd_write17hf49b30128c2714d3E
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
                              i32.const 1052124
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
                        i32.const 1052136
                        call $_ZN4core5slice5index26slice_start_index_len_fail17h43cf12d46dc03083E
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
                    call $_ZN3std2io8buffered9bufwriter18BufWriter$LT$W$GT$14write_all_cold17h4c51e7e75db49116E
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
                    call $_ZN4wasi13lib_generated8fd_write17hf49b30128c2714d3E
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
                          i32.const 1050920
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
                  i32.const 1050496
                  call $_ZN4core5slice5index24slice_end_index_len_fail17h206e334eab3e7498E
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
                  call $_ZN3std2io8buffered9bufwriter18BufWriter$LT$W$GT$14write_all_cold17h4c51e7e75db49116E
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
            i32.const 1050849
            i32.const 35
            i32.const 1050988
            call $_ZN4core9panicking5panic17h5f3201ae514c7bcbE
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
          call $_ZN3std2io8buffered9bufwriter18BufWriter$LT$W$GT$14write_all_cold17h4c51e7e75db49116E
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
    i32.const 1050496
    call $_ZN4core5slice5index24slice_end_index_len_fail17h206e334eab3e7498E
    unreachable
  )
  (func $_ZN4core3fmt5Write10write_char17hf8c41fbbae2a3a44E (;58;) (type 2) (param i32 i32) (result i32)
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
    call $_ZN80_$LT$std..io..Write..write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17h3645be837ec99777E
    local.set 1
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 1
  )
  (func $_ZN80_$LT$std..io..Write..write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17h3645be837ec99777E (;59;) (type 4) (param i32 i32 i32) (result i32)
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
            call $_ZN4wasi13lib_generated8fd_write17hf49b30128c2714d3E
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
                i32.const 1052124
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
    i32.const 1052136
    call $_ZN4core5slice5index26slice_start_index_len_fail17h43cf12d46dc03083E
    unreachable
  )
  (func $_ZN4core3fmt5Write9write_fmt17h60aacf7e3630642cE (;60;) (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.const 1050584
    local.get 1
    call $_ZN4core3fmt5write17h890955524eea605cE
  )
  (func $_ZN4core3fmt5Write9write_fmt17h770044ce5f576af8E (;61;) (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.const 1050632
    local.get 1
    call $_ZN4core3fmt5write17h890955524eea605cE
  )
  (func $_ZN4core3fmt5Write9write_fmt17h914d32a557f83bb7E (;62;) (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.const 1050656
    local.get 1
    call $_ZN4core3fmt5write17h890955524eea605cE
  )
  (func $_ZN4core3fmt5Write9write_fmt17hfa56d2716ef32465E (;63;) (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.const 1050608
    local.get 1
    call $_ZN4core3fmt5write17h890955524eea605cE
  )
  (func $_ZN3std9panicking12default_hook17h8b36ebea1c16cf0dE (;64;) (type 0) (param i32)
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
          i32.load offset=1063760
          i32.const 1
          i32.gt_u
          br_if 0 (;@3;)
          local.get 1
          call $_ZN3std5panic19get_backtrace_style17hd02a20646cd34cc6E
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
          i32.const 1052828
          local.set 0
          local.get 1
          i64.load
          i64.const 4845729891138127776
          i64.xor
          local.get 1
          i32.const 8
          i32.add
          i64.load
          i64.const -6002009914241348162
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
                  i32.load offset=1063768
                  local.tee 0
                  br_if 0 (;@7;)
                  call $_ZN4core4cell4once17OnceCell$LT$T$GT$15get_or_try_init13outlined_call17hf2615006862ba1a4E
                  local.set 0
                  i32.const 0
                  i32.load offset=1063768
                  br_if 1 (;@6;)
                  i32.const 0
                  local.get 0
                  i32.store offset=1063768
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
                    i32.const 1052840
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
                  i32.load8_u offset=1063682
                  br_if 0 (;@7;)
                  local.get 1
                  i32.const 0
                  i32.store offset=76
                  br 4 (;@3;)
                end
                i32.const 0
                i32.const 1
                i32.store8 offset=1063682
                block ;; label = @7
                  i32.const 0
                  i32.load8_u offset=1063776
                  br_if 0 (;@7;)
                  i32.const 0
                  i32.const 1
                  i32.store8 offset=1063776
                  i32.const 0
                  i32.const 0
                  i32.store offset=1063780
                  local.get 1
                  i32.const 0
                  i32.store offset=76
                  br 4 (;@3;)
                end
                local.get 1
                i32.const 0
                i32.load offset=1063780
                local.tee 3
                i32.store offset=76
                i32.const 0
                i32.const 0
                i32.store offset=1063780
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
                      i32.load offset=1063748
                      i32.const 2147483647
                      i32.and
                      br_if 0 (;@9;)
                      local.get 1
                      i32.const 60
                      i32.add
                      local.get 3
                      i32.const 12
                      i32.add
                      i32.const 1052852
                      call $_ZN3std9panicking12default_hook28_$u7b$$u7b$closure$u7d$$u7d$17hc36b36b295329250E
                      local.get 3
                      i32.const 9
                      i32.add
                      local.set 0
                      br 1 (;@8;)
                    end
                    call $_ZN3std9panicking11panic_count17is_zero_slow_path17hcdb989fd2253dff7E
                    local.set 0
                    local.get 1
                    i32.const 60
                    i32.add
                    local.get 3
                    i32.const 12
                    i32.add
                    i32.const 1052852
                    call $_ZN3std9panicking12default_hook28_$u7b$$u7b$closure$u7d$$u7d$17hc36b36b295329250E
                    local.get 0
                    i32.eqz
                    br_if 1 (;@7;)
                    local.get 3
                    i32.const 9
                    i32.add
                    local.set 0
                  end
                  i32.const 0
                  i32.load offset=1063748
                  i32.const 2147483647
                  i32.and
                  i32.eqz
                  br_if 0 (;@7;)
                  call $_ZN3std9panicking11panic_count17is_zero_slow_path17hcdb989fd2253dff7E
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
                i32.store8 offset=1063682
                block ;; label = @7
                  block ;; label = @8
                    i32.const 0
                    i32.load8_u offset=1063776
                    br_if 0 (;@8;)
                    i32.const 0
                    local.get 3
                    i32.store offset=1063780
                    i32.const 0
                    i32.const 1
                    i32.store8 offset=1063776
                    br 1 (;@7;)
                  end
                  i32.const 0
                  i32.load offset=1063780
                  local.set 0
                  i32.const 0
                  local.get 3
                  i32.store offset=1063780
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
                  call $_ZN5alloc4sync16Arc$LT$T$C$A$GT$9drop_slow17h8144c3f827534ba0E
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
              i32.const 1050212
              i32.store offset=84
              local.get 1
              i32.const 1050196
              i32.store offset=92
              local.get 1
              i32.const 84
              i32.add
              i32.const 1050300
              call $_ZN4core9panicking9panic_fmt17h4c9c94dcfde83250E
              unreachable
            end
            unreachable
            unreachable
          end
          local.get 1
          i64.const 0
          i64.store offset=96 align=4
          local.get 1
          i32.const 1050196
          i32.store offset=92
          local.get 1
          i32.const 1
          i32.store offset=88
          local.get 1
          i32.const 1052256
          i32.store offset=84
          local.get 1
          i32.const 83
          i32.add
          local.get 1
          i32.const 84
          i32.add
          call $_ZN4core9panicking13assert_failed17hd26cd55f6f6f629dE
          unreachable
        end
        local.get 1
        i32.const 60
        i32.add
        local.get 1
        i32.const 111
        i32.add
        i32.const 1052892
        call $_ZN3std9panicking12default_hook28_$u7b$$u7b$closure$u7d$$u7d$17hc36b36b295329250E
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
        call $_ZN5alloc4sync16Arc$LT$T$C$A$GT$9drop_slow17h8348ad71bbcb420bE
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
      call $_ZN5alloc4sync16Arc$LT$T$C$A$GT$9drop_slow17h8144c3f827534ba0E
    end
    local.get 1
    i32.const 112
    i32.add
    global.set $__stack_pointer
  )
  (func $_ZN4core3ptr122drop_in_place$LT$$RF$alloc..boxed..Box$LT$dyn$u20$core..error..Error$u2b$core..marker..Sync$u2b$core..marker..Send$GT$$GT$17h97a573c8385cd3f3E (;65;) (type 0) (param i32))
  (func $_ZN3std9panicking11panic_count17is_zero_slow_path17hcdb989fd2253dff7E (;66;) (type 15) (result i32)
    i32.const 0
    i32.load offset=1063760
    i32.eqz
  )
  (func $_ZN4core3ptr29drop_in_place$LT$$LP$$RP$$GT$17h2cc8ce0a008ae539E (;67;) (type 0) (param i32))
  (func $_ZN5alloc4sync16Arc$LT$T$C$A$GT$9drop_slow17h8348ad71bbcb420bE (;68;) (type 0) (param i32)
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
  (func $_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h0584c21fe4121947E (;69;) (type 0) (param i32)
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
  (func $_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17h3468cfc708821600E (;70;) (type 0) (param i32)
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
  (func $_ZN4core3ptr47drop_in_place$LT$wasi..lib_generated..Errno$GT$17hbb8995281c09dab1E (;71;) (type 0) (param i32))
  (func $_ZN4core3ptr77drop_in_place$LT$std..panicking..begin_panic_handler..FormatStringPayload$GT$17h1c91699c70947622E (;72;) (type 0) (param i32)
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
  (func $_ZN4core3ptr81drop_in_place$LT$core..result..Result$LT$$LP$$RP$$C$std..io..error..Error$GT$$GT$17he1d07345407e5497E (;73;) (type 1) (param i32 i32)
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
  (func $_ZN4core3ptr88drop_in_place$LT$std..io..Write..write_fmt..Adapter$LT$alloc..vec..Vec$LT$u8$GT$$GT$$GT$17h95ab4550ec4744f1E (;74;) (type 0) (param i32)
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
  (func $_ZN4core4cell4once17OnceCell$LT$T$GT$15get_or_try_init13outlined_call17hf2615006862ba1a4E (;75;) (type 15) (result i32)
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
    call $_ZN5alloc4sync32arcinner_layout_for_value_layout17h1b6efaa745813bbfE
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
      i32.load8_u offset=1063681
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
        i64.load offset=1063752
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
          i64.load offset=1063752
          local.tee 6
          local.get 6
          local.get 4
          i64.eq
          local.tee 1
          select
          i64.store offset=1063752
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
      call $_ZN5alloc5alloc18handle_alloc_error17h722a53e8070ff93aE
      unreachable
    end
    call $_ZN3std6thread8ThreadId3new9exhausted17hed89bcfbdedc11cfE
    unreachable
  )
  (func $_ZN3std6thread8ThreadId3new9exhausted17hed89bcfbdedc11cfE (;76;) (type 13)
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
    i32.const 1050768
    i32.store offset=8
    local.get 0
    i32.const 1050196
    i32.store offset=16
    local.get 0
    i32.const 8
    i32.add
    i32.const 1050776
    call $_ZN4core9panicking9panic_fmt17h4c9c94dcfde83250E
    unreachable
  )
  (func $_ZN4core9panicking13assert_failed17hd26cd55f6f6f629dE (;77;) (type 1) (param i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    i32.const 1050316
    i32.store offset=12
    local.get 2
    local.get 0
    i32.store offset=8
    i32.const 0
    local.get 2
    i32.const 8
    i32.add
    i32.const 1050320
    local.get 2
    i32.const 12
    i32.add
    i32.const 1050320
    local.get 1
    i32.const 1052320
    call $_ZN4core9panicking19assert_failed_inner17h028fb57387c98e3fE
    unreachable
  )
  (func $_ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Write$GT$10write_char17hc8f5ed4cd4cff5c5E (;78;) (type 2) (param i32 i32) (result i32)
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
            call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17h686a7db31f766b88E
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
        call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17he7f9a7b89ce4867fE
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
  (func $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17h686a7db31f766b88E (;79;) (type 1) (param i32 i32)
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
        call $_ZN5alloc7raw_vec11finish_grow17h0bd40e33419dc061E
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
        call $_ZN5alloc5alloc18handle_alloc_error17h722a53e8070ff93aE
        unreachable
      end
      call $_ZN5alloc7raw_vec17capacity_overflow17h5006d534427a0a2bE
      unreachable
    end
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
  )
  (func $_ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Write$GT$9write_str17h4bc6421b4252011aE (;80;) (type 4) (param i32 i32 i32) (result i32)
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
      call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17he7f9a7b89ce4867fE
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
  (func $_ZN5alloc4sync16Arc$LT$T$C$A$GT$9drop_slow17h8144c3f827534ba0E (;81;) (type 0) (param i32)
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
  (func $_ZN5alloc7raw_vec11finish_grow17h0bd40e33419dc061E (;82;) (type 7) (param i32 i32 i32 i32)
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
                  i32.load8_u offset=1063681
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
              i32.load8_u offset=1063681
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
  (func $_ZN60_$LT$alloc..string..String$u20$as$u20$core..fmt..Display$GT$3fmt17h4f88562cb0fdb0b2E (;83;) (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load offset=4
    local.get 0
    i32.load offset=8
    local.get 1
    call $_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17h33dd62bf170497e3E
  )
  (func $_ZN64_$LT$core..str..error..Utf8Error$u20$as$u20$core..fmt..Debug$GT$3fmt17hc7b0f2dc31690f4dE (;84;) (type 2) (param i32 i32) (result i32)
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
    i32.const 1050528
    i32.const 9
    i32.const 1050537
    i32.const 11
    local.get 0
    i32.const 1050512
    i32.const 1050548
    i32.const 9
    local.get 2
    i32.const 12
    i32.add
    i32.const 1050560
    call $_ZN4core3fmt9Formatter26debug_struct_field2_finish17h44f11ac8f3608eacE
    local.set 0
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 0
  )
  (func $_ZN3std2io5Write9write_fmt17h93c1ecc6a742a6fcE (;85;) (type 3) (param i32 i32 i32)
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
        i32.const 1050608
        local.get 2
        call $_ZN4core3fmt5write17h890955524eea605cE
        i32.eqz
        br_if 0 (;@2;)
        block ;; label = @3
          local.get 3
          i32.load8_u
          i32.const 4
          i32.ne
          br_if 0 (;@3;)
          local.get 0
          i32.const 1052168
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
  (func $_ZN3std3sys4wasi14abort_internal17h97c20099b339f774E (;86;) (type 13)
    call $abort
    unreachable
  )
  (func $_ZN3std3env11current_dir17hf4d9dfe705a916e4E (;87;) (type 0) (param i32)
    (local i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    i32.const 0
    i32.load8_u offset=1063681
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
                  i32.load offset=1064304
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
                call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17he7f9a7b89ce4867fE
                local.get 1
                i32.load offset=8
                local.tee 3
                local.get 1
                i32.load offset=4
                local.tee 2
                call $getcwd
                br_if 3 (;@3;)
                i32.const 0
                i32.load offset=1064304
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
          call $_ZN5alloc5alloc18handle_alloc_error17h722a53e8070ff93aE
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
    call $_ZN5alloc5alloc18handle_alloc_error17h722a53e8070ff93aE
    unreachable
  )
  (func $_ZN3std3env7_var_os17h2fb82b8093724b92E (;88;) (type 3) (param i32 i32 i32)
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
            call $_ZN4core3ffi5c_str4CStr19from_bytes_with_nul17h5393327c07cb4c27E
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
                i32.load8_u offset=1063681
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
            i64.load offset=1052544
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
          call $_ZN3std3sys6common14small_c_string24run_with_cstr_allocating17hc1353fc399a0d377E
          local.get 3
          i32.load offset=8
          local.set 2
          br 2 (;@1;)
        end
        call $_ZN5alloc7raw_vec17capacity_overflow17h5006d534427a0a2bE
        unreachable
      end
      i32.const 1
      local.get 2
      call $_ZN5alloc5alloc18handle_alloc_error17h722a53e8070ff93aE
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
  (func $_ZN3std3sys6common14small_c_string24run_with_cstr_allocating17hc1353fc399a0d377E (;89;) (type 3) (param i32 i32 i32)
    (local i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    local.get 1
    local.get 2
    call $_ZN72_$LT$$RF$str$u20$as$u20$alloc..ffi..c_str..CString..new..SpecNewImpl$GT$13spec_new_impl17h357b044010dd6f22E
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
                i32.load8_u offset=1063681
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
          i64.load offset=1052544
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
        call $_ZN5alloc7raw_vec17capacity_overflow17h5006d534427a0a2bE
        unreachable
      end
      i32.const 1
      local.get 2
      call $_ZN5alloc5alloc18handle_alloc_error17h722a53e8070ff93aE
      unreachable
    end
    local.get 3
    i32.const 16
    i32.add
    global.set $__stack_pointer
  )
  (func $_ZN60_$LT$std..io..error..Error$u20$as$u20$core..fmt..Display$GT$3fmt17h69a01cca8cf82440E (;90;) (type 2) (param i32 i32) (result i32)
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
            call $_ZN3std3sys4wasi2os12error_string17h9ef441d109b1ea5dE
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
            i32.const 12
            i32.store
            local.get 2
            i32.const 3
            i32.store offset=44
            local.get 2
            i32.const 1051768
            i32.store offset=40
            local.get 2
            i32.const 13
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
            call $_ZN4core3fmt9Formatter9write_fmt17h83b5a1707d5b6e2cE
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
          i32.const 1050932
          i32.store offset=40
          local.get 2
          i32.const 14
          i32.store offset=16
          local.get 2
          local.get 0
          i32.const 2
          i32.shl
          local.tee 0
          i32.const 1053748
          i32.add
          i32.load
          i32.store offset=28
          local.get 2
          local.get 0
          i32.const 1053912
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
          call $_ZN4core3fmt9Formatter9write_fmt17h83b5a1707d5b6e2cE
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
        call $_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17h33dd62bf170497e3E
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
  (func $_ZN3std2io8buffered9bufwriter18BufWriter$LT$W$GT$14write_all_cold17h4c51e7e75db49116E (;91;) (type 7) (param i32 i32 i32 i32)
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
            call $_ZN4wasi13lib_generated8fd_write17hf49b30128c2714d3E
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
                  i32.const 1050920
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
            call $_ZN4wasi13lib_generated8fd_write17hf49b30128c2714d3E
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
                    i32.const 1052124
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
              i32.const 1052136
              call $_ZN4core5slice5index26slice_start_index_len_fail17h43cf12d46dc03083E
              unreachable
            end
            local.get 3
            br_if 0 (;@4;)
          end
          i32.const 1052124
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
    i32.const 1050496
    call $_ZN4core5slice5index24slice_end_index_len_fail17h206e334eab3e7498E
    unreachable
  )
  (func $_ZN3std3sys4wasi2os12error_string17h9ef441d109b1ea5dE (;92;) (type 1) (param i32 i32)
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
          call $_ZN4core3str8converts9from_utf817h3661e58cc325c35bE
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
              i32.load8_u offset=1063681
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
          i32.const 1050806
          i32.const 43
          local.get 2
          i32.const 1048
          i32.add
          i32.const 1053436
          i32.const 1053484
          call $_ZN4core6result13unwrap_failed17h7812484c33dfa842E
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
        i32.const 1053520
        i32.store offset=1024
        local.get 2
        i32.const 1050196
        i32.store offset=1032
        local.get 2
        i32.const 1024
        i32.add
        i32.const 1053528
        call $_ZN4core9panicking9panic_fmt17h4c9c94dcfde83250E
        unreachable
      end
      call $_ZN5alloc7raw_vec17capacity_overflow17h5006d534427a0a2bE
      unreachable
    end
    i32.const 1
    local.get 1
    call $_ZN5alloc5alloc18handle_alloc_error17h722a53e8070ff93aE
    unreachable
  )
  (func $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$5write17ha287c1051446cd88E (;93;) (type 7) (param i32 i32 i32 i32)
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
      call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17he7f9a7b89ce4867fE
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
  (func $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$14write_vectored17hd8f38df32697019bE (;94;) (type 7) (param i32 i32 i32 i32)
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
        call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17he7f9a7b89ce4867fE
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
          call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17he7f9a7b89ce4867fE
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
  (func $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$17is_write_vectored17h19984a8e54de7033E (;95;) (type 12) (param i32) (result i32)
    i32.const 1
  )
  (func $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$9write_all17hee48d19b9cfa6b30E (;96;) (type 7) (param i32 i32 i32 i32)
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
      call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17he7f9a7b89ce4867fE
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
  (func $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$5flush17hb26d9c6c4dfd9475E (;97;) (type 1) (param i32 i32)
    local.get 0
    i32.const 4
    i32.store8
  )
  (func $_ZN3std2io5Write18write_all_vectored17h6c11d8c926e24abaE (;98;) (type 7) (param i32 i32 i32 i32)
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
                  i32.const 1051980
                  i32.store offset=8
                  local.get 4
                  i32.const 1050196
                  i32.store offset=16
                  local.get 4
                  i32.const 8
                  i32.add
                  i32.const 1051988
                  call $_ZN4core9panicking9panic_fmt17h4c9c94dcfde83250E
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
                call $_ZN4wasi13lib_generated8fd_write17hf49b30128c2714d3E
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
                        i32.const 1052124
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
                    i32.const 1051980
                    i32.store offset=8
                    local.get 4
                    i32.const 1050196
                    i32.store offset=16
                    local.get 4
                    i32.const 8
                    i32.add
                    i32.const 1051988
                    call $_ZN4core9panicking9panic_fmt17h4c9c94dcfde83250E
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
                  i32.const 1052040
                  i32.store offset=8
                  local.get 4
                  i32.const 1050196
                  i32.store offset=16
                  local.get 4
                  i32.const 8
                  i32.add
                  i32.const 1052080
                  call $_ZN4core9panicking9panic_fmt17h4c9c94dcfde83250E
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
          i32.const 1051924
          call $_ZN4core5slice5index26slice_start_index_len_fail17h43cf12d46dc03083E
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
        i32.const 1052040
        i32.store offset=8
        local.get 4
        i32.const 1050196
        i32.store offset=16
        local.get 4
        i32.const 8
        i32.add
        i32.const 1052080
        call $_ZN4core9panicking9panic_fmt17h4c9c94dcfde83250E
        unreachable
      end
      local.get 8
      local.get 3
      i32.const 1051924
      call $_ZN4core5slice5index26slice_start_index_len_fail17h43cf12d46dc03083E
      unreachable
    end
    local.get 4
    i32.const 32
    i32.add
    global.set $__stack_pointer
  )
  (func $_ZN3std4sync9once_lock17OnceLock$LT$T$GT$10initialize17hfba237aae3bcdfd6E (;99;) (type 13)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 0
    global.set $__stack_pointer
    block ;; label = @1
      i32.const 0
      i32.load8_u offset=1063716
      i32.const 3
      i32.eq
      br_if 0 (;@1;)
      local.get 0
      i32.const 1063684
      i32.store offset=4
      local.get 0
      local.get 0
      i32.const 15
      i32.add
      i32.store offset=8
      local.get 0
      i32.const 4
      i32.add
      call $_ZN3std3sys4wasi4once4Once4call17h688e9a3a6c3bf921E
    end
    local.get 0
    i32.const 16
    i32.add
    global.set $__stack_pointer
  )
  (func $_ZN61_$LT$$RF$std..io..stdio..Stdout$u20$as$u20$std..io..Write$GT$9write_fmt17h54658cfe4485db11E (;100;) (type 3) (param i32 i32 i32)
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
            i32.const 1063772
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
            i32.const 1050196
            i32.store offset=16
            local.get 3
            i32.const 1
            i32.store offset=12
            local.get 3
            i32.const 1052256
            i32.store offset=8
            local.get 3
            i32.const 4
            i32.add
            local.get 3
            i32.const 8
            i32.add
            call $_ZN4core9panicking13assert_failed17hd26cd55f6f6f629dE
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
        i32.const 1063772
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
          i32.const 1050656
          local.get 2
          call $_ZN4core3fmt5write17h890955524eea605cE
          i32.eqz
          br_if 0 (;@3;)
          block ;; label = @4
            local.get 3
            i32.load8_u offset=8
            i32.const 4
            i32.ne
            br_if 0 (;@4;)
            local.get 0
            i32.const 1052168
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
    i32.const 1052396
    i32.const 38
    i32.const 1052468
    call $_ZN4core6option13expect_failed17h7690befb2bc651caE
    unreachable
  )
  (func $_ZN3std2io5stdio31print_to_buffer_if_capture_used17h1ed0a30e2c4d62bbE (;101;) (type 12) (param i32) (result i32)
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
        i32.load8_u offset=1063682
        i32.eqz
        br_if 0 (;@2;)
        i32.const 0
        local.set 2
        block ;; label = @3
          i32.const 0
          i32.load8_u offset=1063776
          br_if 0 (;@3;)
          i32.const 0
          i32.const 1
          i32.store8 offset=1063776
          i32.const 0
          i32.const 0
          i32.store offset=1063780
          br 1 (;@2;)
        end
        i32.const 0
        local.set 2
        i32.const 0
        i32.load offset=1063780
        local.set 3
        i32.const 0
        i32.const 0
        i32.store offset=1063780
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
          i32.load offset=1063748
          i32.const 2147483647
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          call $_ZN3std9panicking11panic_count17is_zero_slow_path17hcdb989fd2253dff7E
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
        i32.const 1050584
        local.get 0
        call $_ZN4core3fmt5write17h890955524eea605cE
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
          i32.load offset=1063748
          i32.const 2147483647
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          call $_ZN3std9panicking11panic_count17is_zero_slow_path17hcdb989fd2253dff7E
          br_if 0 (;@3;)
          local.get 3
          i32.const 1
          i32.store8 offset=9
        end
        local.get 3
        i32.const 0
        i32.store8 offset=8
        i32.const 0
        i32.load offset=1063780
        local.set 2
        i32.const 0
        local.get 3
        i32.store offset=1063780
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
          call $_ZN5alloc4sync16Arc$LT$T$C$A$GT$9drop_slow17h8144c3f827534ba0E
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
    i32.const 1050196
    i32.store offset=16
    local.get 1
    i32.const 1
    i32.store offset=12
    local.get 1
    i32.const 1052256
    i32.store offset=8
    local.get 1
    i32.const 7
    i32.add
    local.get 1
    i32.const 8
    i32.add
    call $_ZN4core9panicking13assert_failed17hd26cd55f6f6f629dE
    unreachable
  )
  (func $_ZN3std2io5stdio6_print17h713f8969f4ff1952E (;102;) (type 0) (param i32)
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
    i32.const 1051892
    i32.store offset=8
    block ;; label = @1
      block ;; label = @2
        local.get 0
        call $_ZN3std2io5stdio31print_to_buffer_if_capture_used17h1ed0a30e2c4d62bbE
        br_if 0 (;@2;)
        block ;; label = @3
          i32.const 0
          i32.load8_u offset=1063716
          i32.const 3
          i32.eq
          br_if 0 (;@3;)
          call $_ZN3std4sync9once_lock17OnceLock$LT$T$GT$10initialize17hfba237aae3bcdfd6E
        end
        local.get 1
        i32.const 1063684
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
        call $_ZN61_$LT$$RF$std..io..stdio..Stdout$u20$as$u20$std..io..Write$GT$9write_fmt17h54658cfe4485db11E
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
    i32.const 15
    i32.store
    local.get 1
    i32.const 2
    i32.store offset=44
    local.get 1
    i32.const 1051860
    i32.store offset=40
    local.get 1
    i32.const 14
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
    i32.const 1051876
    call $_ZN4core9panicking9panic_fmt17h4c9c94dcfde83250E
    unreachable
  )
  (func $_ZN3std2io5Write9write_all17hafc4d82fa322d20fE (;103;) (type 7) (param i32 i32 i32 i32)
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
              call $_ZN4wasi13lib_generated8fd_write17hf49b30128c2714d3E
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
                    i32.const 1052124
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
        i32.const 1052136
        call $_ZN4core5slice5index26slice_start_index_len_fail17h43cf12d46dc03083E
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
  (func $_ZN3std2io5Write18write_all_vectored17h19c40315f984a3a6E (;104;) (type 7) (param i32 i32 i32 i32)
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
                    i32.const 1051980
                    i32.store offset=8
                    local.get 4
                    i32.const 1050196
                    i32.store offset=16
                    local.get 4
                    i32.const 8
                    i32.add
                    i32.const 1051988
                    call $_ZN4core9panicking9panic_fmt17h4c9c94dcfde83250E
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
                    call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17he7f9a7b89ce4867fE
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
                      call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17he7f9a7b89ce4867fE
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
                    i32.const 1052124
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
                      i32.const 1051980
                      i32.store offset=8
                      local.get 4
                      i32.const 1050196
                      i32.store offset=16
                      local.get 4
                      i32.const 8
                      i32.add
                      i32.const 1051988
                      call $_ZN4core9panicking9panic_fmt17h4c9c94dcfde83250E
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
            i32.const 1051924
            call $_ZN4core5slice5index26slice_start_index_len_fail17h43cf12d46dc03083E
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
          i32.const 1052040
          i32.store offset=8
          local.get 4
          i32.const 1050196
          i32.store offset=16
          local.get 4
          i32.const 8
          i32.add
          i32.const 1052080
          call $_ZN4core9panicking9panic_fmt17h4c9c94dcfde83250E
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
        i32.const 1052040
        i32.store offset=8
        local.get 4
        i32.const 1050196
        i32.store offset=16
        local.get 4
        i32.const 8
        i32.add
        i32.const 1052080
        call $_ZN4core9panicking9panic_fmt17h4c9c94dcfde83250E
        unreachable
      end
      local.get 8
      local.get 3
      i32.const 1051924
      call $_ZN4core5slice5index26slice_start_index_len_fail17h43cf12d46dc03083E
      unreachable
    end
    local.get 4
    i32.const 32
    i32.add
    global.set $__stack_pointer
  )
  (func $_ZN3std2io5Write9write_fmt17h6324f22ae5513448E (;105;) (type 3) (param i32 i32 i32)
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
        i32.const 1050584
        local.get 2
        call $_ZN4core3fmt5write17h890955524eea605cE
        i32.eqz
        br_if 0 (;@2;)
        block ;; label = @3
          local.get 3
          i32.load8_u
          i32.const 4
          i32.ne
          br_if 0 (;@3;)
          local.get 0
          i32.const 1052168
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
  (func $_ZN80_$LT$std..io..Write..write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17hb70b43f3d4c03d02E (;106;) (type 4) (param i32 i32 i32) (result i32)
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
      call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17he7f9a7b89ce4867fE
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
  (func $_ZN80_$LT$std..io..Write..write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17hdb9e855babefd96dE (;107;) (type 4) (param i32 i32 i32) (result i32)
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
    call $_ZN61_$LT$std..io..stdio..StdoutLock$u20$as$u20$std..io..Write$GT$9write_all17h0bd9a33f327da1daE
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
  (func $_ZN3std5panic19get_backtrace_style17hd02a20646cd34cc6E (;108;) (type 15) (result i32)
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
              i32.load offset=1063720
              br_table 3 (;@2;) 4 (;@1;) 2 (;@3;) 1 (;@4;) 0 (;@5;)
            end
            i32.const 1050379
            i32.const 40
            i32.const 1052204
            call $_ZN4core9panicking5panic17h5f3201ae514c7bcbE
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
      i32.const 1050792
      i32.const 14
      call $_ZN3std3env7_var_os17h2fb82b8093724b92E
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
            i32.const 1052220
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
      i32.store offset=1063720
    end
    local.get 0
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 1
  )
  (func $_ZN3std7process5abort17h003913cedc7f2c6eE (;109;) (type 13)
    call $_ZN3std3sys4wasi14abort_internal17h97c20099b339f774E
    unreachable
  )
  (func $_ZN3std3sys4wasi4once4Once4call17h688e9a3a6c3bf921E (;110;) (type 0) (param i32)
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
              i32.load8_u offset=1063716
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
            i32.store8 offset=1063716
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
            i32.load8_u offset=1063681
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
            i32.store8 offset=1063716
          end
          local.get 1
          i32.const 32
          i32.add
          global.set $__stack_pointer
          return
        end
        i32.const 1050336
        i32.const 43
        i32.const 1052380
        call $_ZN4core9panicking5panic17h5f3201ae514c7bcbE
        unreachable
      end
      i32.const 1
      i32.const 1024
      call $_ZN5alloc5alloc18handle_alloc_error17h722a53e8070ff93aE
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
    i32.const 1053740
    i32.store offset=8
    local.get 1
    i32.const 1050196
    i32.store offset=16
    local.get 1
    i32.const 8
    i32.add
    i32.const 1052364
    call $_ZN4core9panicking9panic_fmt17h4c9c94dcfde83250E
    unreachable
  )
  (func $_ZN3std10sys_common9backtrace5print17hc57878cd021af9b8E (;111;) (type 7) (param i32 i32 i32 i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    i32.const 0
    i32.load8_u offset=1063724
    local.set 5
    i32.const 1
    local.set 6
    i32.const 0
    i32.const 1
    i32.store8 offset=1063724
    local.get 4
    local.get 5
    i32.store8 offset=36
    block ;; label = @1
      local.get 5
      br_if 0 (;@1;)
      block ;; label = @2
        i32.const 0
        i32.load offset=1063748
        i32.const 2147483647
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        call $_ZN3std9panicking11panic_count17is_zero_slow_path17hcdb989fd2253dff7E
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
      i32.const 1050932
      i32.store offset=12
      local.get 4
      i32.const 16
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
        i32.load offset=1063748
        i32.const 2147483647
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        call $_ZN3std9panicking11panic_count17is_zero_slow_path17hcdb989fd2253dff7E
        br_if 0 (;@2;)
        i32.const 0
        i32.const 1
        i32.store8 offset=1063725
      end
      i32.const 0
      i32.const 0
      i32.store8 offset=1063724
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
    i32.const 1050196
    i32.store offset=20
    local.get 4
    i32.const 1
    i32.store offset=16
    local.get 4
    i32.const 1052256
    i32.store offset=12
    local.get 4
    i32.const 36
    i32.add
    local.get 4
    i32.const 12
    i32.add
    call $_ZN4core9panicking13assert_failed17hd26cd55f6f6f629dE
    unreachable
  )
  (func $_ZN91_$LT$std..sys_common..backtrace.._print..DisplayBacktrace$u20$as$u20$core..fmt..Display$GT$3fmt17h18070e4ad91fc378E (;112;) (type 2) (param i32 i32) (result i32)
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
    call $_ZN3std3env11current_dir17hf4d9dfe705a916e4E
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
    i32.const 1052572
    i32.store offset=8
    local.get 2
    i32.const 1050196
    i32.store offset=16
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 1
          local.get 2
          i32.const 8
          i32.add
          call $_ZN4core3fmt9Formatter9write_fmt17h83b5a1707d5b6e2cE
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
            i32.const 1052668
            i32.store offset=8
            local.get 2
            i32.const 1050196
            i32.store offset=16
            local.get 1
            local.get 2
            i32.const 8
            i32.add
            call $_ZN4core3fmt9Formatter9write_fmt17h83b5a1707d5b6e2cE
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
  (func $_ZN3std10sys_common9backtrace26__rust_end_short_backtrace17h4708749c4a5e7755E (;113;) (type 0) (param i32)
    local.get 0
    call $_ZN3std9panicking19begin_panic_handler28_$u7b$$u7b$closure$u7d$$u7d$17hdedc9ee43b558024E
    unreachable
  )
  (func $_ZN3std9panicking19begin_panic_handler28_$u7b$$u7b$closure$u7d$$u7d$17hdedc9ee43b558024E (;114;) (type 0) (param i32)
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
          i32.const 1050196
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
      i32.const 1053124
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
      call $_ZN3std9panicking20rust_panic_with_hook17h6a7157d4523a883fE
      unreachable
    end
    local.get 1
    local.get 2
    i32.store offset=12
    local.get 1
    i32.const -2147483648
    i32.store
    local.get 1
    i32.const 1053144
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
    call $_ZN3std9panicking20rust_panic_with_hook17h6a7157d4523a883fE
    unreachable
  )
  (func $_ZN3std5alloc24default_alloc_error_hook17he5c27c2a00a1f22bE (;115;) (type 1) (param i32 i32)
    (local i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 64
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block ;; label = @1
      i32.const 0
      i32.load8_u offset=1063680
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
      i32.const 1052712
      i32.store offset=12
      local.get 2
      i32.const 17
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
      i32.const 1050608
      local.get 2
      i32.const 12
      i32.add
      call $_ZN4core3fmt5write17h890955524eea605cE
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
    i32.const 1052744
    i32.store offset=12
    local.get 2
    i32.const 17
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
    i32.const 1052784
    call $_ZN4core9panicking9panic_fmt17h4c9c94dcfde83250E
    unreachable
  )
  (func $__rdl_alloc (;116;) (type 2) (param i32 i32) (result i32)
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
  (func $__rdl_dealloc (;117;) (type 3) (param i32 i32 i32)
    local.get 0
    call $free
  )
  (func $__rdl_realloc (;118;) (type 5) (param i32 i32 i32 i32) (result i32)
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
  (func $_ZN3std9panicking12default_hook28_$u7b$$u7b$closure$u7d$$u7d$17hc36b36b295329250E (;119;) (type 3) (param i32 i32 i32)
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
    i32.const 14
    i32.store
    local.get 3
    i32.const 40
    i32.add
    i32.const 12
    i32.add
    i32.const 18
    i32.store
    local.get 3
    i32.const 1052956
    i32.store offset=16
    local.get 3
    i32.const 14
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
          call $_ZN3std10sys_common9backtrace5print17hc57878cd021af9b8E
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
        call $_ZN3std10sys_common9backtrace5print17hc57878cd021af9b8E
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
      i32.load8_u offset=1063588
      local.set 0
      i32.const 0
      i32.const 0
      i32.store8 offset=1063588
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
      i32.const 1053068
      i32.store offset=40
      local.get 3
      i32.const 1050196
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
  (func $rust_begin_unwind (;120;) (type 0) (param i32)
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
      i32.const 1050336
      i32.const 43
      i32.const 1053076
      call $_ZN4core9panicking5panic17h5f3201ae514c7bcbE
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
    call $_ZN3std10sys_common9backtrace26__rust_end_short_backtrace17h4708749c4a5e7755E
    unreachable
  )
  (func $_ZN102_$LT$std..panicking..begin_panic_handler..FormatStringPayload$u20$as$u20$core..panic..PanicPayload$GT$8take_box17h82b487330533288eE (;121;) (type 1) (param i32 i32)
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
      i32.const 1050632
      local.get 3
      call $_ZN4core3fmt5write17h890955524eea605cE
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
    i32.load8_u offset=1063681
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
      call $_ZN5alloc5alloc18handle_alloc_error17h722a53e8070ff93aE
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
    i32.const 1053092
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store
    local.get 2
    i32.const 48
    i32.add
    global.set $__stack_pointer
  )
  (func $_ZN102_$LT$std..panicking..begin_panic_handler..FormatStringPayload$u20$as$u20$core..panic..PanicPayload$GT$3get17h6d9db02763a39a57E (;122;) (type 1) (param i32 i32)
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
      i32.const 1050632
      local.get 3
      call $_ZN4core3fmt5write17h890955524eea605cE
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
    i32.const 1053092
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
  )
  (func $_ZN99_$LT$std..panicking..begin_panic_handler..StaticStrPayload$u20$as$u20$core..panic..PanicPayload$GT$8take_box17h630c1ce94eac9b08E (;123;) (type 1) (param i32 i32)
    (local i32 i32)
    i32.const 0
    i32.load8_u offset=1063681
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
      call $_ZN5alloc5alloc18handle_alloc_error17h722a53e8070ff93aE
      unreachable
    end
    local.get 1
    local.get 2
    i32.store offset=4
    local.get 1
    local.get 3
    i32.store
    local.get 0
    i32.const 1053108
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store
  )
  (func $_ZN99_$LT$std..panicking..begin_panic_handler..StaticStrPayload$u20$as$u20$core..panic..PanicPayload$GT$3get17h9d53a07a6ca87788E (;124;) (type 1) (param i32 i32)
    local.get 0
    i32.const 1053108
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store
  )
  (func $_ZN3std9panicking20rust_panic_with_hook17h6a7157d4523a883fE (;125;) (type 16) (param i32 i32 i32 i32 i32 i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 80
    i32.sub
    local.tee 6
    global.set $__stack_pointer
    i32.const 0
    i32.const 0
    i32.load offset=1063748
    local.tee 7
    i32.const 1
    i32.add
    i32.store offset=1063748
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
                  i32.load8_u offset=1063764
                  br_if 1 (;@6;)
                  i32.const 0
                  i32.const 1
                  i32.store8 offset=1063764
                  i32.const 0
                  i32.const 0
                  i32.load offset=1063760
                  i32.const 1
                  i32.add
                  i32.store offset=1063760
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
                  i32.const 1053164
                  i32.store offset=20
                  local.get 6
                  i32.const 1050196
                  i32.store offset=16
                  i32.const 0
                  i32.load offset=1063732
                  local.tee 7
                  i32.const -1
                  i32.le_s
                  br_if 5 (;@2;)
                  i32.const 0
                  local.get 7
                  i32.const 1
                  i32.add
                  i32.store offset=1063732
                  i32.const 0
                  i32.load offset=1063740
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
                  i32.load offset=1063740
                  local.get 6
                  i32.const 16
                  i32.add
                  i32.const 0
                  i32.load offset=1063744
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
                i32.const 1053164
                i32.store offset=20
                local.get 6
                i32.const 1050196
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
                i32.const 1053232
                i32.store offset=40
                local.get 6
                i32.const 19
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
                i32.const 1050608
                local.get 6
                i32.const 40
                i32.add
                call $_ZN4core3fmt5write17h890955524eea605cE
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
                  call $_ZN3std3sys4wasi14abort_internal17h97c20099b339f774E
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
                call $_ZN3std3sys4wasi14abort_internal17h97c20099b339f774E
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
              i32.const 1053300
              i32.store offset=40
              local.get 6
              i32.const 1050196
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
              i32.const 1050608
              local.get 6
              i32.const 40
              i32.add
              call $_ZN4core3fmt5write17h890955524eea605cE
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
                call $_ZN3std3sys4wasi14abort_internal17h97c20099b339f774E
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
            call $_ZN3std3sys4wasi14abort_internal17h97c20099b339f774E
            unreachable
          end
          local.get 6
          i32.const 16
          i32.add
          call $_ZN3std9panicking12default_hook17h8b36ebea1c16cf0dE
        end
        i32.const 0
        i32.const 0
        i32.load offset=1063732
        i32.const -1
        i32.add
        i32.store offset=1063732
        i32.const 0
        i32.const 0
        i32.store8 offset=1063764
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
      i32.const 1053676
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
      call $_ZN3std2io5Write9write_fmt17h93c1ecc6a742a6fcE
      local.get 6
      i32.load8_u offset=64
      local.get 6
      i32.load offset=68
      call $_ZN4core3ptr81drop_in_place$LT$core..result..Result$LT$$LP$$RP$$C$std..io..error..Error$GT$$GT$17he1d07345407e5497E
      call $_ZN3std3sys4wasi14abort_internal17h97c20099b339f774E
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
    i32.const 1053356
    i32.store offset=40
    local.get 6
    i32.const 1050196
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
    call $_ZN3std2io5Write9write_fmt17h93c1ecc6a742a6fcE
    local.get 6
    i32.load8_u offset=64
    local.get 6
    i32.load offset=68
    call $_ZN4core3ptr81drop_in_place$LT$core..result..Result$LT$$LP$$RP$$C$std..io..error..Error$GT$$GT$17he1d07345407e5497E
    call $_ZN3std3sys4wasi14abort_internal17h97c20099b339f774E
    unreachable
  )
  (func $rust_panic (;126;) (type 1) (param i32 i32)
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
    i32.const 1053420
    i32.store offset=12
    local.get 2
    i32.const 17
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
    call $_ZN3std2io5Write9write_fmt17h93c1ecc6a742a6fcE
    local.get 2
    i32.load8_u offset=4
    local.get 2
    i32.load offset=8
    call $_ZN4core3ptr81drop_in_place$LT$core..result..Result$LT$$LP$$RP$$C$std..io..error..Error$GT$$GT$17he1d07345407e5497E
    call $_ZN3std3sys4wasi14abort_internal17h97c20099b339f774E
    unreachable
  )
  (func $_ZN64_$LT$std..sys..wasi..stdio..Stderr$u20$as$u20$std..io..Write$GT$5write17h0c145d49840ec5beE (;127;) (type 7) (param i32 i32 i32 i32)
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
    call $_ZN4wasi13lib_generated8fd_write17hf49b30128c2714d3E
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
  (func $_ZN64_$LT$std..sys..wasi..stdio..Stderr$u20$as$u20$std..io..Write$GT$14write_vectored17h3ffa40051f9caeddE (;128;) (type 7) (param i32 i32 i32 i32)
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
    call $_ZN4wasi13lib_generated8fd_write17hf49b30128c2714d3E
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
  (func $_ZN64_$LT$std..sys..wasi..stdio..Stderr$u20$as$u20$std..io..Write$GT$17is_write_vectored17h922cd97db2995730E (;129;) (type 12) (param i32) (result i32)
    i32.const 1
  )
  (func $_ZN64_$LT$std..sys..wasi..stdio..Stderr$u20$as$u20$std..io..Write$GT$5flush17h00c625cb4891deebE (;130;) (type 1) (param i32 i32)
    local.get 0
    i32.const 4
    i32.store8
  )
  (func $_ZN3std3sys4wasi19hashmap_random_keys17hbdc0b948ce1cb89eE (;131;) (type 0) (param i32)
    (local i32 i64)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    local.get 1
    i32.const 16
    i32.add
    i64.const 0
    i64.store
    local.get 1
    i64.const 0
    i64.store offset=8
    local.get 1
    local.get 1
    i32.const 8
    i32.add
    i32.const 16
    call $_ZN4wasi13lib_generated10random_get17h7012308658086b8bE
    block ;; label = @1
      local.get 1
      i32.load16_u
      br_if 0 (;@1;)
      local.get 1
      i64.load offset=8
      local.set 2
      local.get 0
      local.get 1
      i64.load offset=16
      i64.store offset=8
      local.get 0
      local.get 2
      i64.store
      local.get 1
      i32.const 32
      i32.add
      global.set $__stack_pointer
      return
    end
    local.get 1
    local.get 1
    i32.load16_u offset=2
    i32.store16 offset=30
    i32.const 1053560
    i32.const 18
    local.get 1
    i32.const 30
    i32.add
    i32.const 1053544
    i32.const 1053612
    call $_ZN4core6result13unwrap_failed17h7812484c33dfa842E
    unreachable
  )
  (func $_ZN3std5alloc8rust_oom17h4199af748a8525a6E (;132;) (type 1) (param i32 i32)
    (local i32)
    local.get 0
    local.get 1
    i32.const 0
    i32.load offset=1063728
    local.tee 2
    i32.const 20
    local.get 2
    select
    call_indirect (type 1)
    call $_ZN3std7process5abort17h003913cedc7f2c6eE
    unreachable
  )
  (func $__rg_oom (;133;) (type 1) (param i32 i32)
    local.get 1
    local.get 0
    call $_ZN3std5alloc8rust_oom17h4199af748a8525a6E
    unreachable
  )
  (func $__rust_start_panic (;134;) (type 2) (param i32 i32) (result i32)
    unreachable
    unreachable
  )
  (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h0817299a726df9f5E (;135;) (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 0
    i32.load offset=4
    local.get 1
    call $_ZN40_$LT$str$u20$as$u20$core..fmt..Debug$GT$3fmt17h839fd847cc958bcdE
  )
  (func $_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u16$GT$3fmt17hcd7c0c3518380c0dE (;136;) (type 2) (param i32 i32) (result i32)
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
        call $_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u16$GT$3fmt17hc68eef1bf3b24f50E
        return
      end
      local.get 0
      local.get 1
      call $_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i16$GT$3fmt17h67676b927244aafbE
      return
    end
    local.get 0
    local.get 1
    call $_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i16$GT$3fmt17hed8dc0c873c39c89E
  )
  (func $_ZN4core3ptr24drop_in_place$LT$u16$GT$17h2be5d1308325373fE (;137;) (type 0) (param i32))
  (func $_ZN4core3ptr24drop_in_place$LT$u32$GT$17h9daefed861bb557fE (;138;) (type 0) (param i32))
  (func $_ZN4wasi13lib_generated5Errno4name17h86918d69ba867cc2E (;139;) (type 1) (param i32 i32)
    local.get 0
    local.get 1
    i32.load16_s
    i32.const 2
    i32.shl
    local.tee 1
    i32.const 1056360
    i32.add
    i32.load
    i32.store offset=4
    local.get 0
    local.get 1
    i32.const 1056668
    i32.add
    i32.load
    i32.store
  )
  (func $_ZN4wasi13lib_generated5Errno7message17hbce3dea1a67aedc1E (;140;) (type 1) (param i32 i32)
    local.get 0
    local.get 1
    i32.load16_s
    i32.const 2
    i32.shl
    local.tee 1
    i32.const 1056976
    i32.add
    i32.load
    i32.store offset=4
    local.get 0
    local.get 1
    i32.const 1057284
    i32.add
    i32.load
    i32.store
  )
  (func $_ZN63_$LT$wasi..lib_generated..Errno$u20$as$u20$core..fmt..Debug$GT$3fmt17h5d615f2dd44a9ecaE (;141;) (type 2) (param i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    i32.const 24
    i32.add
    local.get 1
    i32.const 1056337
    i32.const 5
    call $_ZN4core3fmt9Formatter12debug_struct17h8dcb4ffc7ee470c3E
    local.get 2
    i32.const 24
    i32.add
    i32.const 1054076
    i32.const 4
    local.get 0
    i32.const 1056344
    call $_ZN4core3fmt8builders11DebugStruct5field17h262f149dc4e3bf7dE
    local.set 1
    local.get 2
    i32.const 16
    i32.add
    local.get 0
    call $_ZN4wasi13lib_generated5Errno4name17h86918d69ba867cc2E
    local.get 2
    local.get 2
    i64.load offset=16
    i64.store offset=32 align=4
    local.get 1
    i32.const 1054080
    i32.const 4
    local.get 2
    i32.const 32
    i32.add
    i32.const 1054084
    call $_ZN4core3fmt8builders11DebugStruct5field17h262f149dc4e3bf7dE
    local.set 1
    local.get 2
    i32.const 8
    i32.add
    local.get 0
    call $_ZN4wasi13lib_generated5Errno7message17hbce3dea1a67aedc1E
    local.get 2
    local.get 2
    i64.load offset=8
    i64.store offset=40 align=4
    local.get 1
    i32.const 1054100
    i32.const 7
    local.get 2
    i32.const 40
    i32.add
    i32.const 1054084
    call $_ZN4core3fmt8builders11DebugStruct5field17h262f149dc4e3bf7dE
    call $_ZN4core3fmt8builders11DebugStruct6finish17haaefede11847f5d5E
    local.set 0
    local.get 2
    i32.const 48
    i32.add
    global.set $__stack_pointer
    local.get 0
  )
  (func $_ZN4wasi13lib_generated8fd_write17hf49b30128c2714d3E (;142;) (type 7) (param i32 i32 i32 i32)
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
        call $_ZN4wasi13lib_generated22wasi_snapshot_preview18fd_write17hc1e6e60059cf136aE
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
  (func $_ZN4wasi13lib_generated10random_get17h7012308658086b8bE (;143;) (type 3) (param i32 i32 i32)
    local.get 0
    local.get 1
    local.get 2
    call $_ZN4wasi13lib_generated22wasi_snapshot_preview110random_get17he04df62476182fdcE
    local.tee 2
    i32.store16 offset=2
    local.get 0
    local.get 2
    i32.const 0
    i32.ne
    i32.store16
  )
  (func $malloc (;144;) (type 12) (param i32) (result i32)
    local.get 0
    call $dlmalloc
  )
  (func $dlmalloc (;145;) (type 12) (param i32) (result i32)
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
                            i32.load offset=1063832
                            local.tee 2
                            br_if 0 (;@12;)
                            block ;; label = @13
                              i32.const 0
                              i32.load offset=1064280
                              local.tee 3
                              br_if 0 (;@13;)
                              i32.const 0
                              i64.const -1
                              i64.store offset=1064292 align=4
                              i32.const 0
                              i64.const 281474976776192
                              i64.store offset=1064284 align=4
                              i32.const 0
                              local.get 1
                              i32.const 8
                              i32.add
                              i32.const -16
                              i32.and
                              i32.const 1431655768
                              i32.xor
                              local.tee 3
                              i32.store offset=1064280
                              i32.const 0
                              i32.const 0
                              i32.store offset=1064300
                              i32.const 0
                              i32.const 0
                              i32.store offset=1064252
                            end
                            i32.const 1114112
                            i32.const 1064352
                            i32.lt_u
                            br_if 1 (;@11;)
                            i32.const 0
                            local.set 2
                            i32.const 1114112
                            i32.const 1064352
                            i32.sub
                            i32.const 89
                            i32.lt_u
                            br_if 0 (;@12;)
                            i32.const 0
                            local.set 4
                            i32.const 0
                            i32.const 1064352
                            i32.store offset=1064256
                            i32.const 0
                            i32.const 1064352
                            i32.store offset=1063824
                            i32.const 0
                            local.get 3
                            i32.store offset=1063844
                            i32.const 0
                            i32.const -1
                            i32.store offset=1063840
                            i32.const 0
                            i32.const 1114112
                            i32.const 1064352
                            i32.sub
                            i32.store offset=1064260
                            loop ;; label = @13
                              local.get 4
                              i32.const 1063868
                              i32.add
                              local.get 4
                              i32.const 1063856
                              i32.add
                              local.tee 3
                              i32.store
                              local.get 3
                              local.get 4
                              i32.const 1063848
                              i32.add
                              local.tee 5
                              i32.store
                              local.get 4
                              i32.const 1063860
                              i32.add
                              local.get 5
                              i32.store
                              local.get 4
                              i32.const 1063876
                              i32.add
                              local.get 4
                              i32.const 1063864
                              i32.add
                              local.tee 5
                              i32.store
                              local.get 5
                              local.get 3
                              i32.store
                              local.get 4
                              i32.const 1063884
                              i32.add
                              local.get 4
                              i32.const 1063872
                              i32.add
                              local.tee 3
                              i32.store
                              local.get 3
                              local.get 5
                              i32.store
                              local.get 4
                              i32.const 1063880
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
                            i32.const 1064352
                            i32.const -8
                            i32.const 1064352
                            i32.sub
                            i32.const 15
                            i32.and
                            i32.const 0
                            i32.const 1064352
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
                            i32.const 1064352
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
                            i32.load offset=1064296
                            i32.store offset=1063836
                            i32.const 0
                            local.get 4
                            i32.store offset=1063820
                            i32.const 0
                            local.get 2
                            i32.store offset=1063832
                            local.get 3
                            i32.const 1064352
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
                                i32.load offset=1063808
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
                                    i32.const 1063848
                                    i32.add
                                    local.tee 4
                                    local.get 3
                                    i32.const 1063856
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
                                    i32.store offset=1063808
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
                              i32.load offset=1063816
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
                                    i32.const 1063848
                                    i32.add
                                    local.tee 5
                                    local.get 4
                                    i32.const 1063856
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
                                    i32.store offset=1063808
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
                                  i32.const 1063848
                                  i32.add
                                  local.set 7
                                  i32.const 0
                                  i32.load offset=1063828
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
                                      i32.store offset=1063808
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
                                i32.store offset=1063828
                                i32.const 0
                                local.get 5
                                i32.store offset=1063816
                                br 13 (;@1;)
                              end
                              i32.const 0
                              i32.load offset=1063812
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
                              i32.const 1064112
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
                                i32.load offset=1063824
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
                            i32.load offset=1063812
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
                                    i32.const 1064112
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
                                  i32.const 1064112
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
                            i32.load offset=1063816
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
                              i32.load offset=1063824
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
                            i32.load offset=1063816
                            local.tee 4
                            local.get 7
                            i32.lt_u
                            br_if 0 (;@12;)
                            i32.const 0
                            i32.load offset=1063828
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
                            i32.store offset=1063816
                            i32.const 0
                            local.get 0
                            i32.store offset=1063828
                            local.get 3
                            i32.const 8
                            i32.add
                            local.set 4
                            br 11 (;@1;)
                          end
                          block ;; label = @12
                            i32.const 0
                            i32.load offset=1063820
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
                            i32.store offset=1063832
                            i32.const 0
                            local.get 3
                            i32.store offset=1063820
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
                              i32.load offset=1064280
                              i32.eqz
                              br_if 0 (;@13;)
                              i32.const 0
                              i32.load offset=1064288
                              local.set 3
                              br 1 (;@12;)
                            end
                            i32.const 0
                            i64.const -1
                            i64.store offset=1064292 align=4
                            i32.const 0
                            i64.const 281474976776192
                            i64.store offset=1064284 align=4
                            i32.const 0
                            local.get 1
                            i32.const 12
                            i32.add
                            i32.const -16
                            i32.and
                            i32.const 1431655768
                            i32.xor
                            i32.store offset=1064280
                            i32.const 0
                            i32.const 0
                            i32.store offset=1064300
                            i32.const 0
                            i32.const 0
                            i32.store offset=1064252
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
                            i32.store offset=1064304
                            br 11 (;@1;)
                          end
                          block ;; label = @12
                            i32.const 0
                            i32.load offset=1064248
                            local.tee 4
                            i32.eqz
                            br_if 0 (;@12;)
                            block ;; label = @13
                              i32.const 0
                              i32.load offset=1064240
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
                            i32.store offset=1064304
                            br 11 (;@1;)
                          end
                          i32.const 0
                          i32.load8_u offset=1064252
                          i32.const 4
                          i32.and
                          br_if 5 (;@6;)
                          block ;; label = @12
                            block ;; label = @13
                              block ;; label = @14
                                local.get 2
                                i32.eqz
                                br_if 0 (;@14;)
                                i32.const 1064256
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
                                i32.load offset=1064284
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
                                i32.load offset=1064248
                                local.tee 4
                                i32.eqz
                                br_if 0 (;@14;)
                                i32.const 0
                                i32.load offset=1064240
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
                              i32.load offset=1064288
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
                i32.load offset=1064252
                i32.const 4
                i32.or
                i32.store offset=1064252
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
            i32.load offset=1064240
            local.get 6
            i32.add
            local.tee 4
            i32.store offset=1064240
            block ;; label = @5
              local.get 4
              i32.const 0
              i32.load offset=1064244
              i32.le_u
              br_if 0 (;@5;)
              i32.const 0
              local.get 4
              i32.store offset=1064244
            end
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    i32.const 0
                    i32.load offset=1063832
                    local.tee 3
                    i32.eqz
                    br_if 0 (;@8;)
                    i32.const 1064256
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
                      i32.load offset=1063824
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
                    i32.store offset=1063824
                  end
                  i32.const 0
                  local.set 4
                  i32.const 0
                  local.get 6
                  i32.store offset=1064260
                  i32.const 0
                  local.get 0
                  i32.store offset=1064256
                  i32.const 0
                  i32.const -1
                  i32.store offset=1063840
                  i32.const 0
                  i32.const 0
                  i32.load offset=1064280
                  i32.store offset=1063844
                  i32.const 0
                  i32.const 0
                  i32.store offset=1064268
                  loop ;; label = @8
                    local.get 4
                    i32.const 1063868
                    i32.add
                    local.get 4
                    i32.const 1063856
                    i32.add
                    local.tee 3
                    i32.store
                    local.get 3
                    local.get 4
                    i32.const 1063848
                    i32.add
                    local.tee 5
                    i32.store
                    local.get 4
                    i32.const 1063860
                    i32.add
                    local.get 5
                    i32.store
                    local.get 4
                    i32.const 1063876
                    i32.add
                    local.get 4
                    i32.const 1063864
                    i32.add
                    local.tee 5
                    i32.store
                    local.get 5
                    local.get 3
                    i32.store
                    local.get 4
                    i32.const 1063884
                    i32.add
                    local.get 4
                    i32.const 1063872
                    i32.add
                    local.tee 3
                    i32.store
                    local.get 3
                    local.get 5
                    i32.store
                    local.get 4
                    i32.const 1063880
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
                  i32.load offset=1064296
                  i32.store offset=1063836
                  i32.const 0
                  local.get 4
                  i32.store offset=1063820
                  i32.const 0
                  local.get 3
                  i32.store offset=1063832
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
                i32.load offset=1063820
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
                i32.load offset=1064296
                i32.store offset=1063836
                i32.const 0
                local.get 5
                i32.store offset=1063820
                i32.const 0
                local.get 0
                i32.store offset=1063832
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
                i32.load offset=1063824
                local.tee 9
                i32.ge_u
                br_if 0 (;@6;)
                i32.const 0
                local.get 0
                i32.store offset=1063824
                local.get 0
                local.set 9
              end
              local.get 0
              local.get 6
              i32.add
              local.set 5
              i32.const 1064256
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
                        i32.const 1064256
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
                        i32.store offset=1063832
                        i32.const 0
                        i32.const 0
                        i32.load offset=1063820
                        local.get 4
                        i32.add
                        local.tee 4
                        i32.store offset=1063820
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
                        i32.load offset=1063828
                        i32.ne
                        br_if 0 (;@10;)
                        i32.const 0
                        local.get 7
                        i32.store offset=1063828
                        i32.const 0
                        i32.const 0
                        i32.load offset=1063816
                        local.get 4
                        i32.add
                        local.tee 4
                        i32.store offset=1063816
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
                            i32.const 1063848
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
                              i32.load offset=1063808
                              i32.const -2
                              local.get 9
                              i32.rotl
                              i32.and
                              i32.store offset=1063808
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
                              i32.const 1064112
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
                              i32.load offset=1063812
                              i32.const -2
                              local.get 5
                              i32.rotl
                              i32.and
                              i32.store offset=1063812
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
                        i32.const 1063848
                        i32.add
                        local.set 3
                        block ;; label = @11
                          block ;; label = @12
                            i32.const 0
                            i32.load offset=1063808
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
                            i32.store offset=1063808
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
                      i32.const 1064112
                      i32.add
                      local.set 5
                      block ;; label = @10
                        i32.const 0
                        i32.load offset=1063812
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
                        i32.store offset=1063812
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
                    i32.load offset=1064296
                    i32.store offset=1063836
                    i32.const 0
                    local.get 4
                    i32.store offset=1063820
                    i32.const 0
                    local.get 2
                    i32.store offset=1063832
                    local.get 9
                    i32.const 16
                    i32.add
                    i32.const 0
                    i64.load offset=1064264 align=4
                    i64.store align=4
                    local.get 9
                    i32.const 0
                    i64.load offset=1064256 align=4
                    i64.store offset=8 align=4
                    i32.const 0
                    local.get 9
                    i32.const 8
                    i32.add
                    i32.store offset=1064264
                    i32.const 0
                    local.get 6
                    i32.store offset=1064260
                    i32.const 0
                    local.get 0
                    i32.store offset=1064256
                    i32.const 0
                    i32.const 0
                    i32.store offset=1064268
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
                      i32.const 1063848
                      i32.add
                      local.set 4
                      block ;; label = @10
                        block ;; label = @11
                          i32.const 0
                          i32.load offset=1063808
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
                          i32.store offset=1063808
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
                    i32.const 1064112
                    i32.add
                    local.set 5
                    block ;; label = @9
                      i32.const 0
                      i32.load offset=1063812
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
                      i32.store offset=1063812
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
            i32.load offset=1063820
            local.tee 4
            local.get 7
            i32.le_u
            br_if 0 (;@4;)
            i32.const 0
            i32.load offset=1063832
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
            i32.store offset=1063820
            i32.const 0
            local.get 5
            i32.store offset=1063832
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
          i32.store offset=1064304
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
              i32.const 1064112
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
              i32.store offset=1063812
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
            i32.const 1063848
            i32.add
            local.set 4
            block ;; label = @5
              block ;; label = @6
                i32.const 0
                i32.load offset=1063808
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
                i32.store offset=1063808
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
          i32.const 1064112
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
            i32.store offset=1063812
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
            i32.const 1064112
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
            i32.store offset=1063812
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
          i32.const 1063848
          i32.add
          local.set 7
          i32.const 0
          i32.load offset=1063828
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
              i32.store offset=1063808
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
        i32.store offset=1063828
        i32.const 0
        local.get 3
        i32.store offset=1063816
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
  (func $free (;146;) (type 0) (param i32)
    local.get 0
    call $dlfree
  )
  (func $dlfree (;147;) (type 0) (param i32)
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
        i32.load offset=1063824
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
          i32.load offset=1063828
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
            i32.const 1063848
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
              i32.load offset=1063808
              i32.const -2
              local.get 5
              i32.rotl
              i32.and
              i32.store offset=1063808
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
              i32.const 1064112
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
              i32.load offset=1063812
              i32.const -2
              local.get 4
              i32.rotl
              i32.and
              i32.store offset=1063812
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
        i32.store offset=1063816
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
            i32.load offset=1063832
            i32.ne
            br_if 0 (;@4;)
            i32.const 0
            local.get 1
            i32.store offset=1063832
            i32.const 0
            i32.const 0
            i32.load offset=1063820
            local.get 0
            i32.add
            local.tee 0
            i32.store offset=1063820
            local.get 1
            local.get 0
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 1
            i32.const 0
            i32.load offset=1063828
            i32.ne
            br_if 3 (;@1;)
            i32.const 0
            i32.const 0
            i32.store offset=1063816
            i32.const 0
            i32.const 0
            i32.store offset=1063828
            return
          end
          block ;; label = @4
            local.get 3
            i32.const 0
            i32.load offset=1063828
            i32.ne
            br_if 0 (;@4;)
            i32.const 0
            local.get 1
            i32.store offset=1063828
            i32.const 0
            i32.const 0
            i32.load offset=1063816
            local.get 0
            i32.add
            local.tee 0
            i32.store offset=1063816
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
              i32.const 1063848
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
                i32.load offset=1063808
                i32.const -2
                local.get 5
                i32.rotl
                i32.and
                i32.store offset=1063808
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
                i32.load offset=1063824
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
                i32.const 1064112
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
                i32.load offset=1063812
                i32.const -2
                local.get 4
                i32.rotl
                i32.and
                i32.store offset=1063812
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
          i32.load offset=1063828
          i32.ne
          br_if 1 (;@2;)
          i32.const 0
          local.get 0
          i32.store offset=1063816
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
        i32.const 1063848
        i32.add
        local.set 2
        block ;; label = @3
          block ;; label = @4
            i32.const 0
            i32.load offset=1063808
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
            i32.store offset=1063808
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
      i32.const 1064112
      i32.add
      local.set 4
      block ;; label = @2
        block ;; label = @3
          i32.const 0
          i32.load offset=1063812
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
          i32.store offset=1063812
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
      i32.load offset=1063840
      i32.const -1
      i32.add
      local.tee 1
      i32.const -1
      local.get 1
      select
      i32.store offset=1063840
    end
  )
  (func $calloc (;148;) (type 2) (param i32 i32) (result i32)
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
  (func $realloc (;149;) (type 2) (param i32 i32) (result i32)
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
      i32.store offset=1064304
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
          i32.load offset=1064288
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
          i32.load offset=1063832
          i32.ne
          br_if 0 (;@3;)
          i32.const 0
          i32.load offset=1063820
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
          i32.store offset=1063832
          i32.const 0
          local.get 5
          local.get 2
          i32.sub
          local.tee 2
          i32.store offset=1063820
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
          i32.load offset=1063828
          i32.ne
          br_if 0 (;@3;)
          i32.const 0
          i32.load offset=1063816
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
          i32.store offset=1063828
          i32.const 0
          local.get 1
          i32.store offset=1063816
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
            i32.const 1063848
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
              i32.load offset=1063808
              i32.const -2
              local.get 11
              i32.rotl
              i32.and
              i32.store offset=1063808
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
              i32.load offset=1063824
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
              i32.const 1064112
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
              i32.load offset=1063812
              i32.const -2
              local.get 5
              i32.rotl
              i32.and
              i32.store offset=1063812
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
  (func $dispose_chunk (;150;) (type 1) (param i32 i32)
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
            i32.load offset=1063828
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
              i32.const 1063848
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
              i32.load offset=1063808
              i32.const -2
              local.get 5
              i32.rotl
              i32.and
              i32.store offset=1063808
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
                i32.load offset=1063824
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
                i32.const 1064112
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
                i32.load offset=1063812
                i32.const -2
                local.get 4
                i32.rotl
                i32.and
                i32.store offset=1063812
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
          i32.store offset=1063816
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
            i32.load offset=1063832
            i32.ne
            br_if 0 (;@4;)
            i32.const 0
            local.get 0
            i32.store offset=1063832
            i32.const 0
            i32.const 0
            i32.load offset=1063820
            local.get 1
            i32.add
            local.tee 1
            i32.store offset=1063820
            local.get 0
            local.get 1
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 0
            i32.const 0
            i32.load offset=1063828
            i32.ne
            br_if 3 (;@1;)
            i32.const 0
            i32.const 0
            i32.store offset=1063816
            i32.const 0
            i32.const 0
            i32.store offset=1063828
            return
          end
          block ;; label = @4
            local.get 2
            i32.const 0
            i32.load offset=1063828
            i32.ne
            br_if 0 (;@4;)
            i32.const 0
            local.get 0
            i32.store offset=1063828
            i32.const 0
            i32.const 0
            i32.load offset=1063816
            local.get 1
            i32.add
            local.tee 1
            i32.store offset=1063816
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
              i32.const 1063848
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
                i32.load offset=1063808
                i32.const -2
                local.get 5
                i32.rotl
                i32.and
                i32.store offset=1063808
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
                i32.load offset=1063824
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
                i32.const 1064112
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
                i32.load offset=1063812
                i32.const -2
                local.get 4
                i32.rotl
                i32.and
                i32.store offset=1063812
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
          i32.load offset=1063828
          i32.ne
          br_if 1 (;@2;)
          i32.const 0
          local.get 1
          i32.store offset=1063816
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
        i32.const 1063848
        i32.add
        local.set 3
        block ;; label = @3
          block ;; label = @4
            i32.const 0
            i32.load offset=1063808
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
            i32.store offset=1063808
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
      i32.const 1064112
      i32.add
      local.set 4
      block ;; label = @2
        i32.const 0
        i32.load offset=1063812
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
        i32.store offset=1063812
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
  (func $internal_memalign (;151;) (type 2) (param i32 i32) (result i32)
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
      i32.store offset=1064304
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
  (func $aligned_alloc (;152;) (type 2) (param i32 i32) (result i32)
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
  (func $_Exit (;153;) (type 0) (param i32)
    local.get 0
    call $__wasi_proc_exit
    unreachable
  )
  (func $__wasilibc_ensure_environ (;154;) (type 13)
    block ;; label = @1
      i32.const 0
      i32.load offset=1063592
      i32.const -1
      i32.ne
      br_if 0 (;@1;)
      call $__wasilibc_initialize_environ
    end
  )
  (func $__wasilibc_initialize_environ (;155;) (type 13)
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
          i32.const 1064308
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
    i32.store offset=1063592
    local.get 0
    i32.const 16
    i32.add
    global.set $__stack_pointer
  )
  (func $__wasi_environ_get (;156;) (type 2) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call $__imported_wasi_snapshot_preview1_environ_get
    i32.const 65535
    i32.and
  )
  (func $__wasi_environ_sizes_get (;157;) (type 2) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call $__imported_wasi_snapshot_preview1_environ_sizes_get
    i32.const 65535
    i32.and
  )
  (func $__wasi_proc_exit (;158;) (type 0) (param i32)
    local.get 0
    call $__imported_wasi_snapshot_preview1_proc_exit
    unreachable
  )
  (func $abort (;159;) (type 13)
    unreachable
    unreachable
  )
  (func $getcwd (;160;) (type 2) (param i32 i32) (result i32)
    (local i32)
    i32.const 0
    i32.load offset=1063596
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
        i32.store offset=1064304
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
      i32.store offset=1064304
    end
    local.get 0
  )
  (func $sbrk (;161;) (type 12) (param i32) (result i32)
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
        i32.store offset=1064304
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
  (func $getenv (;162;) (type 12) (param i32) (result i32)
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
      i32.load offset=1063592
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
  (func $dummy (;163;) (type 13))
  (func $__wasm_call_dtors (;164;) (type 13)
    call $dummy
    call $dummy
  )
  (func $memcmp (;165;) (type 4) (param i32 i32 i32) (result i32)
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
  (func $memcpy (;166;) (type 4) (param i32 i32 i32) (result i32)
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
  (func $memmove (;167;) (type 4) (param i32 i32 i32) (result i32)
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
  (func $memset (;168;) (type 4) (param i32 i32 i32) (result i32)
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
  (func $__strchrnul (;169;) (type 2) (param i32 i32) (result i32)
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
  (func $__stpcpy (;170;) (type 2) (param i32 i32) (result i32)
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
  (func $strcpy (;171;) (type 2) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call $__stpcpy
    drop
    local.get 0
  )
  (func $strdup (;172;) (type 12) (param i32) (result i32)
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
  (func $#func173<dummy> (@name "dummy") (;173;) (type 2) (param i32 i32) (result i32)
    local.get 0
  )
  (func $__lctrans (;174;) (type 2) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call $#func173<dummy>
  )
  (func $strerror (;175;) (type 12) (param i32) (result i32)
    (local i32)
    block ;; label = @1
      i32.const 0
      i32.load offset=1064336
      local.tee 1
      br_if 0 (;@1;)
      i32.const 1064312
      local.set 1
      i32.const 0
      i32.const 1064312
      i32.store offset=1064336
    end
    i32.const 0
    local.get 0
    local.get 0
    i32.const 76
    i32.gt_u
    select
    i32.const 1
    i32.shl
    i32.const 1059152
    i32.add
    i32.load16_u
    i32.const 1057594
    i32.add
    local.get 1
    i32.load offset=20
    call $__lctrans
  )
  (func $strerror_r (;176;) (type 4) (param i32 i32 i32) (result i32)
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
  (func $strlen (;177;) (type 12) (param i32) (result i32)
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
  (func $strncmp (;178;) (type 4) (param i32 i32 i32) (result i32)
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
  (func $_ZN4core3ptr37drop_in_place$LT$core..fmt..Error$GT$17h5d671d2c1d102d73E (;179;) (type 0) (param i32))
  (func $_ZN69_$LT$core..alloc..layout..LayoutError$u20$as$u20$core..fmt..Debug$GT$3fmt17h992770ef3553fb60E (;180;) (type 2) (param i32 i32) (result i32)
    local.get 1
    i32.const 1059308
    i32.const 11
    call $_ZN4core3fmt9Formatter9write_str17h4bf1acaddf72a444E
  )
  (func $_ZN5alloc7raw_vec11finish_grow17h49caba1939568f3bE (;181;) (type 7) (param i32 i32 i32 i32)
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
                  i32.load8_u offset=1063681
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
              i32.load8_u offset=1063681
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
  (func $_ZN5alloc5alloc18handle_alloc_error17h722a53e8070ff93aE (;182;) (type 1) (param i32 i32)
    local.get 1
    local.get 0
    call $__rust_alloc_error_handler
    unreachable
  )
  (func $_ZN5alloc7raw_vec17capacity_overflow17h5006d534427a0a2bE (;183;) (type 13)
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
    i32.const 1059364
    i32.store offset=8
    local.get 0
    i32.const 1059308
    i32.store offset=16
    local.get 0
    i32.const 8
    i32.add
    i32.const 1059372
    call $_ZN4core9panicking9panic_fmt17h4c9c94dcfde83250E
    unreachable
  )
  (func $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17h44ea41f7f81505fdE (;184;) (type 1) (param i32 i32)
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
        call $_ZN5alloc7raw_vec11finish_grow17h49caba1939568f3bE
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
        call $_ZN5alloc5alloc18handle_alloc_error17h722a53e8070ff93aE
        unreachable
      end
      call $_ZN5alloc7raw_vec17capacity_overflow17h5006d534427a0a2bE
      unreachable
    end
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
  )
  (func $_ZN72_$LT$$RF$str$u20$as$u20$alloc..ffi..c_str..CString..new..SpecNewImpl$GT$13spec_new_impl17h357b044010dd6f22E (;185;) (type 3) (param i32 i32 i32)
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
              i32.load8_u offset=1063681
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
                call $_ZN4core5slice6memchr14memchr_aligned17h4eaec85e20476149E
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
            i32.const 1059388
            i32.const 43
            i32.const 1059464
            call $_ZN4core9panicking5panic17h5f3201ae514c7bcbE
            unreachable
          end
          call $_ZN5alloc7raw_vec17capacity_overflow17h5006d534427a0a2bE
          unreachable
        end
        i32.const 1
        local.get 4
        call $_ZN5alloc5alloc18handle_alloc_error17h722a53e8070ff93aE
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
        call $_ZN5alloc3ffi5c_str7CString19_from_vec_unchecked17h710b5d212bf8e54aE
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
  (func $_ZN5alloc3ffi5c_str7CString19_from_vec_unchecked17h710b5d212bf8e54aE (;186;) (type 1) (param i32 i32)
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
        call $_ZN5alloc7raw_vec11finish_grow17h49caba1939568f3bE
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
        call $_ZN5alloc5alloc18handle_alloc_error17h722a53e8070ff93aE
        unreachable
      end
      call $_ZN5alloc7raw_vec17capacity_overflow17h5006d534427a0a2bE
      unreachable
    end
    block ;; label = @1
      local.get 4
      local.get 3
      i32.ne
      br_if 0 (;@1;)
      local.get 1
      local.get 4
      call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17h44ea41f7f81505fdE
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
    call $_ZN5alloc5alloc18handle_alloc_error17h722a53e8070ff93aE
    unreachable
  )
  (func $_ZN5alloc4sync32arcinner_layout_for_value_layout17h1b6efaa745813bbfE (;187;) (type 3) (param i32 i32 i32)
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
    i32.const 1059480
    i32.const 43
    local.get 3
    i32.const 15
    i32.add
    i32.const 1059524
    i32.const 1059568
    call $_ZN4core6result13unwrap_failed17h7812484c33dfa842E
    unreachable
  )
  (func $_ZN4core3ops8function6FnOnce9call_once17hd548f09e85633547E (;188;) (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    drop
    loop (result i32) ;; label = @1
      br 0 (;@1;)
    end
  )
  (func $_ZN4core3ptr25drop_in_place$LT$char$GT$17h6ee13944e4abd559E (;189;) (type 0) (param i32))
  (func $_ZN4core3ptr37drop_in_place$LT$core..fmt..Error$GT$17h141d50a833f61f7aE (;190;) (type 0) (param i32))
  (func $_ZN4core9panicking9panic_fmt17h4c9c94dcfde83250E (;191;) (type 1) (param i32 i32)
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
    i32.const 1059776
    i32.store offset=16
    local.get 2
    i32.const 1059584
    i32.store offset=12
    local.get 2
    i32.const 12
    i32.add
    call $rust_begin_unwind
    unreachable
  )
  (func $_ZN4core5slice5index26slice_start_index_len_fail17h43cf12d46dc03083E (;192;) (type 3) (param i32 i32 i32)
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
    i32.const 17
    i32.store
    local.get 3
    i32.const 2
    i32.store offset=12
    local.get 3
    i32.const 1060492
    i32.store offset=8
    local.get 3
    i32.const 17
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
    call $_ZN4core9panicking9panic_fmt17h4c9c94dcfde83250E
    unreachable
  )
  (func $_ZN4core9panicking18panic_bounds_check17he2ca869b88362af2E (;193;) (type 3) (param i32 i32 i32)
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
    i64.const 2
    i64.store align=4
    local.get 3
    i32.const 32
    i32.add
    i32.const 12
    i32.add
    i32.const 17
    i32.store
    local.get 3
    i32.const 2
    i32.store offset=12
    local.get 3
    i32.const 1059844
    i32.store offset=8
    local.get 3
    i32.const 17
    i32.store offset=36
    local.get 3
    local.get 3
    i32.const 32
    i32.add
    i32.store offset=16
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
    call $_ZN4core9panicking9panic_fmt17h4c9c94dcfde83250E
    unreachable
  )
  (func $_ZN4core5slice5index24slice_end_index_len_fail17h206e334eab3e7498E (;194;) (type 3) (param i32 i32 i32)
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
    i32.const 17
    i32.store
    local.get 3
    i32.const 2
    i32.store offset=12
    local.get 3
    i32.const 1060524
    i32.store offset=8
    local.get 3
    i32.const 17
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
    call $_ZN4core9panicking9panic_fmt17h4c9c94dcfde83250E
    unreachable
  )
  (func $_ZN4core3fmt9Formatter3pad17h58189de40b979c18E (;195;) (type 4) (param i32 i32 i32) (result i32)
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
          call $_ZN4core3str5count14do_count_chars17h2c35506888f0ac80E
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
  (func $_ZN4core9panicking5panic17h5f3201ae514c7bcbE (;196;) (type 3) (param i32 i32 i32)
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
    i32.const 1059584
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
    call $_ZN4core9panicking9panic_fmt17h4c9c94dcfde83250E
    unreachable
  )
  (func $_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17h66c9e2e594bd720fE (;197;) (type 2) (param i32 i32) (result i32)
    local.get 0
    i64.load32_u
    i32.const 1
    local.get 1
    call $_ZN4core3fmt3num3imp7fmt_u6417ha7c93bd38c403be2E
  )
  (func $_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u32$GT$3fmt17h9963dd7304ee75d9E (;198;) (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 128
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              local.get 1
              i32.load offset=28
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
              call $_ZN4core3fmt3num3imp7fmt_u6417ha7c93bd38c403be2E
              local.set 0
              br 2 (;@3;)
            end
            local.get 0
            i32.load
            local.set 0
            i32.const 0
            local.set 3
            loop ;; label = @5
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
              br_if 0 (;@5;)
            end
            local.get 3
            i32.const 128
            i32.add
            local.tee 0
            i32.const 128
            i32.gt_u
            br_if 2 (;@2;)
            local.get 1
            i32.const 1
            i32.const 1060112
            i32.const 2
            local.get 2
            local.get 3
            i32.add
            i32.const 128
            i32.add
            i32.const 0
            local.get 3
            i32.sub
            call $_ZN4core3fmt9Formatter12pad_integral17ha8545ce28396770eE
            local.set 0
            br 1 (;@3;)
          end
          local.get 0
          i32.load
          local.set 0
          i32.const 0
          local.set 3
          loop ;; label = @4
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
            br_if 0 (;@4;)
          end
          local.get 3
          i32.const 128
          i32.add
          local.tee 0
          i32.const 128
          i32.gt_u
          br_if 2 (;@1;)
          local.get 1
          i32.const 1
          i32.const 1060112
          i32.const 2
          local.get 2
          local.get 3
          i32.add
          i32.const 128
          i32.add
          i32.const 0
          local.get 3
          i32.sub
          call $_ZN4core3fmt9Formatter12pad_integral17ha8545ce28396770eE
          local.set 0
        end
        local.get 2
        i32.const 128
        i32.add
        global.set $__stack_pointer
        local.get 0
        return
      end
      local.get 0
      i32.const 128
      i32.const 1060116
      call $_ZN4core5slice5index26slice_start_index_len_fail17h43cf12d46dc03083E
      unreachable
    end
    local.get 0
    i32.const 128
    i32.const 1060116
    call $_ZN4core5slice5index26slice_start_index_len_fail17h43cf12d46dc03083E
    unreachable
  )
  (func $_ZN4core3fmt5write17h890955524eea605cE (;199;) (type 4) (param i32 i32 i32) (result i32)
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
                  i32.const 73
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
                  i32.const 73
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
  (func $_ZN71_$LT$core..ops..range..Range$LT$Idx$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17h650ab90e3c444b89E (;200;) (type 2) (param i32 i32) (result i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    i32.const 1
    local.set 3
    block ;; label = @1
      local.get 0
      local.get 1
      call $_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u32$GT$3fmt17h9963dd7304ee75d9E
      br_if 0 (;@1;)
      local.get 2
      i32.const 20
      i32.add
      i64.const 0
      i64.store align=4
      i32.const 1
      local.set 3
      local.get 2
      i32.const 1
      i32.store offset=12
      local.get 2
      i32.const 1059660
      i32.store offset=8
      local.get 2
      i32.const 1059584
      i32.store offset=16
      local.get 1
      i32.load offset=20
      local.get 1
      i32.const 24
      i32.add
      i32.load
      local.get 2
      i32.const 8
      i32.add
      call $_ZN4core3fmt5write17h890955524eea605cE
      br_if 0 (;@1;)
      local.get 0
      i32.const 4
      i32.add
      local.get 1
      call $_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u32$GT$3fmt17h9963dd7304ee75d9E
      local.set 3
    end
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
    local.get 3
  )
  (func $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h1b016e6ca1df6044E (;201;) (type 1) (param i32 i32)
    local.get 0
    i64.const 4507122837358743131
    i64.store offset=8
    local.get 0
    i64.const -2401257079958803507
    i64.store
  )
  (func $_ZN4core5slice5index22slice_index_order_fail17h889710de9520e473E (;202;) (type 3) (param i32 i32 i32)
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
    i32.const 17
    i32.store
    local.get 3
    i32.const 2
    i32.store offset=12
    local.get 3
    i32.const 1060576
    i32.store offset=8
    local.get 3
    i32.const 17
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
    call $_ZN4core9panicking9panic_fmt17h4c9c94dcfde83250E
    unreachable
  )
  (func $_ZN63_$LT$core..cell..BorrowMutError$u20$as$u20$core..fmt..Debug$GT$3fmt17h34898b32f446e8ffE (;203;) (type 2) (param i32 i32) (result i32)
    local.get 1
    i32.load offset=20
    i32.const 1059684
    i32.const 14
    local.get 1
    i32.const 24
    i32.add
    i32.load
    i32.load offset=12
    call_indirect (type 4)
  )
  (func $_ZN4core4cell22panic_already_borrowed17h813146898ec049ddE (;204;) (type 0) (param i32)
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
    i32.const 1059716
    i32.store offset=12
    local.get 1
    i32.const 74
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
    call $_ZN4core9panicking9panic_fmt17h4c9c94dcfde83250E
    unreachable
  )
  (func $_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i16$GT$3fmt17hed8dc0c873c39c89E (;205;) (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 128
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 0
    i32.load16_u
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
      i32.const 65535
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
      i32.const 1060116
      call $_ZN4core5slice5index26slice_start_index_len_fail17h43cf12d46dc03083E
      unreachable
    end
    local.get 1
    i32.const 1
    i32.const 1060112
    i32.const 2
    local.get 2
    local.get 0
    i32.add
    i32.const 128
    i32.add
    i32.const 0
    local.get 0
    i32.sub
    call $_ZN4core3fmt9Formatter12pad_integral17ha8545ce28396770eE
    local.set 0
    local.get 2
    i32.const 128
    i32.add
    global.set $__stack_pointer
    local.get 0
  )
  (func $_ZN4core4char7methods22_$LT$impl$u20$char$GT$16escape_debug_ext17h60cdf599b8c6d176E (;206;) (type 3) (param i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 3
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
                        local.get 1
                        br_table 5 (;@5;) 8 (;@2;) 8 (;@2;) 8 (;@2;) 8 (;@2;) 8 (;@2;) 8 (;@2;) 8 (;@2;) 8 (;@2;) 1 (;@9;) 3 (;@7;) 8 (;@2;) 8 (;@2;) 2 (;@8;) 8 (;@2;) 8 (;@2;) 8 (;@2;) 8 (;@2;) 8 (;@2;) 8 (;@2;) 8 (;@2;) 8 (;@2;) 8 (;@2;) 8 (;@2;) 8 (;@2;) 8 (;@2;) 8 (;@2;) 8 (;@2;) 8 (;@2;) 8 (;@2;) 8 (;@2;) 8 (;@2;) 8 (;@2;) 8 (;@2;) 6 (;@4;) 8 (;@2;) 8 (;@2;) 8 (;@2;) 8 (;@2;) 7 (;@3;) 0 (;@10;)
                      end
                      local.get 1
                      i32.const 92
                      i32.eq
                      br_if 3 (;@6;)
                      br 7 (;@2;)
                    end
                    local.get 0
                    i32.const 512
                    i32.store16 offset=10
                    local.get 0
                    i64.const 0
                    i64.store offset=2 align=2
                    local.get 0
                    i32.const 29788
                    i32.store16
                    br 7 (;@1;)
                  end
                  local.get 0
                  i32.const 512
                  i32.store16 offset=10
                  local.get 0
                  i64.const 0
                  i64.store offset=2 align=2
                  local.get 0
                  i32.const 29276
                  i32.store16
                  br 6 (;@1;)
                end
                local.get 0
                i32.const 512
                i32.store16 offset=10
                local.get 0
                i64.const 0
                i64.store offset=2 align=2
                local.get 0
                i32.const 28252
                i32.store16
                br 5 (;@1;)
              end
              local.get 0
              i32.const 512
              i32.store16 offset=10
              local.get 0
              i64.const 0
              i64.store offset=2 align=2
              local.get 0
              i32.const 23644
              i32.store16
              br 4 (;@1;)
            end
            local.get 0
            i32.const 512
            i32.store16 offset=10
            local.get 0
            i64.const 0
            i64.store offset=2 align=2
            local.get 0
            i32.const 12380
            i32.store16
            br 3 (;@1;)
          end
          local.get 2
          i32.const 65536
          i32.and
          i32.eqz
          br_if 1 (;@2;)
          local.get 0
          i32.const 512
          i32.store16 offset=10
          local.get 0
          i64.const 0
          i64.store offset=2 align=2
          local.get 0
          i32.const 8796
          i32.store16
          br 2 (;@1;)
        end
        local.get 2
        i32.const 256
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        i32.const 512
        i32.store16 offset=10
        local.get 0
        i64.const 0
        i64.store offset=2 align=2
        local.get 0
        i32.const 10076
        i32.store16
        br 1 (;@1;)
      end
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                local.get 2
                i32.const 1
                i32.and
                i32.eqz
                br_if 0 (;@6;)
                local.get 1
                call $_ZN4core7unicode12unicode_data15grapheme_extend6lookup17h4b266a61fda9d716E
                br_if 1 (;@5;)
              end
              local.get 1
              call $_ZN4core7unicode9printable12is_printable17h8c88fda38b5ef3b1E
              i32.eqz
              br_if 1 (;@4;)
              local.get 0
              local.get 1
              i32.store offset=4
              local.get 0
              i32.const 128
              i32.store8
              br 4 (;@1;)
            end
            local.get 3
            i32.const 6
            i32.add
            i32.const 2
            i32.add
            i32.const 0
            i32.store8
            local.get 3
            i32.const 0
            i32.store16 offset=6
            local.get 3
            i32.const 125
            i32.store8 offset=15
            local.get 3
            local.get 1
            i32.const 15
            i32.and
            i32.const 1059668
            i32.add
            i32.load8_u
            i32.store8 offset=14
            local.get 3
            local.get 1
            i32.const 4
            i32.shr_u
            i32.const 15
            i32.and
            i32.const 1059668
            i32.add
            i32.load8_u
            i32.store8 offset=13
            local.get 3
            local.get 1
            i32.const 8
            i32.shr_u
            i32.const 15
            i32.and
            i32.const 1059668
            i32.add
            i32.load8_u
            i32.store8 offset=12
            local.get 3
            local.get 1
            i32.const 12
            i32.shr_u
            i32.const 15
            i32.and
            i32.const 1059668
            i32.add
            i32.load8_u
            i32.store8 offset=11
            local.get 3
            local.get 1
            i32.const 16
            i32.shr_u
            i32.const 15
            i32.and
            i32.const 1059668
            i32.add
            i32.load8_u
            i32.store8 offset=10
            local.get 3
            local.get 1
            i32.const 20
            i32.shr_u
            i32.const 15
            i32.and
            i32.const 1059668
            i32.add
            i32.load8_u
            i32.store8 offset=9
            local.get 1
            i32.const 1
            i32.or
            i32.clz
            i32.const 2
            i32.shr_u
            i32.const -2
            i32.add
            local.tee 1
            i32.const 11
            i32.ge_u
            br_if 1 (;@3;)
            local.get 3
            i32.const 6
            i32.add
            local.get 1
            i32.add
            local.tee 2
            i32.const 0
            i32.load16_u offset=1062708 align=1
            i32.store16 align=1
            local.get 2
            i32.const 2
            i32.add
            i32.const 0
            i32.load8_u offset=1062710
            i32.store8
            local.get 0
            local.get 3
            i64.load offset=6 align=2
            i64.store align=1
            local.get 0
            i32.const 8
            i32.add
            local.get 3
            i32.const 6
            i32.add
            i32.const 8
            i32.add
            i32.load16_u
            i32.store16 align=1
            local.get 0
            i32.const 10
            i32.store8 offset=11
            local.get 0
            local.get 1
            i32.store8 offset=10
            br 3 (;@1;)
          end
          local.get 3
          i32.const 6
          i32.add
          i32.const 2
          i32.add
          i32.const 0
          i32.store8
          local.get 3
          i32.const 0
          i32.store16 offset=6
          local.get 3
          i32.const 125
          i32.store8 offset=15
          local.get 3
          local.get 1
          i32.const 15
          i32.and
          i32.const 1059668
          i32.add
          i32.load8_u
          i32.store8 offset=14
          local.get 3
          local.get 1
          i32.const 4
          i32.shr_u
          i32.const 15
          i32.and
          i32.const 1059668
          i32.add
          i32.load8_u
          i32.store8 offset=13
          local.get 3
          local.get 1
          i32.const 8
          i32.shr_u
          i32.const 15
          i32.and
          i32.const 1059668
          i32.add
          i32.load8_u
          i32.store8 offset=12
          local.get 3
          local.get 1
          i32.const 12
          i32.shr_u
          i32.const 15
          i32.and
          i32.const 1059668
          i32.add
          i32.load8_u
          i32.store8 offset=11
          local.get 3
          local.get 1
          i32.const 16
          i32.shr_u
          i32.const 15
          i32.and
          i32.const 1059668
          i32.add
          i32.load8_u
          i32.store8 offset=10
          local.get 3
          local.get 1
          i32.const 20
          i32.shr_u
          i32.const 15
          i32.and
          i32.const 1059668
          i32.add
          i32.load8_u
          i32.store8 offset=9
          local.get 1
          i32.const 1
          i32.or
          i32.clz
          i32.const 2
          i32.shr_u
          i32.const -2
          i32.add
          local.tee 1
          i32.const 11
          i32.ge_u
          br_if 1 (;@2;)
          local.get 3
          i32.const 6
          i32.add
          local.get 1
          i32.add
          local.tee 2
          i32.const 0
          i32.load16_u offset=1062708 align=1
          i32.store16 align=1
          local.get 2
          i32.const 2
          i32.add
          i32.const 0
          i32.load8_u offset=1062710
          i32.store8
          local.get 0
          local.get 3
          i64.load offset=6 align=2
          i64.store align=1
          local.get 0
          i32.const 8
          i32.add
          local.get 3
          i32.const 6
          i32.add
          i32.const 8
          i32.add
          i32.load16_u
          i32.store16 align=1
          local.get 0
          i32.const 10
          i32.store8 offset=11
          local.get 0
          local.get 1
          i32.store8 offset=10
          br 2 (;@1;)
        end
        local.get 1
        i32.const 10
        i32.const 1062692
        call $_ZN4core5slice5index26slice_start_index_len_fail17h43cf12d46dc03083E
        unreachable
      end
      local.get 1
      i32.const 10
      i32.const 1062692
      call $_ZN4core5slice5index26slice_start_index_len_fail17h43cf12d46dc03083E
      unreachable
    end
    local.get 3
    i32.const 16
    i32.add
    global.set $__stack_pointer
  )
  (func $_ZN4core7unicode12unicode_data15grapheme_extend6lookup17h4b266a61fda9d716E (;207;) (type 12) (param i32) (result i32)
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
    block ;; label = @1
      block ;; label = @2
        loop ;; label = @3
          local.get 3
          i32.const 1
          i32.shr_u
          local.get 2
          i32.add
          local.tee 3
          i32.const 2
          i32.shl
          i32.const 1062728
          i32.add
          i32.load
          i32.const 11
          i32.shl
          local.tee 5
          local.get 1
          i32.eq
          br_if 1 (;@2;)
          local.get 3
          local.get 4
          local.get 5
          local.get 1
          i32.gt_u
          select
          local.tee 4
          local.get 3
          i32.const 1
          i32.add
          local.get 2
          local.get 5
          local.get 1
          i32.lt_u
          select
          local.tee 2
          i32.sub
          local.set 3
          local.get 4
          local.get 2
          i32.gt_u
          br_if 0 (;@3;)
          br 2 (;@1;)
        end
      end
      local.get 3
      i32.const 1
      i32.add
      local.set 2
    end
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            local.get 2
            i32.const 32
            i32.gt_u
            br_if 0 (;@4;)
            local.get 2
            i32.const 2
            i32.shl
            local.tee 3
            i32.const 1062728
            i32.add
            i32.load
            i32.const 21
            i32.shr_u
            local.set 1
            local.get 2
            i32.const 32
            i32.ne
            br_if 1 (;@3;)
            i32.const 31
            local.set 2
            i32.const 727
            local.set 5
            br 2 (;@2;)
          end
          local.get 2
          i32.const 33
          i32.const 1062632
          call $_ZN4core9panicking18panic_bounds_check17he2ca869b88362af2E
          unreachable
        end
        local.get 3
        i32.const 1062732
        i32.add
        i32.load
        i32.const 21
        i32.shr_u
        local.set 5
        block ;; label = @3
          local.get 2
          br_if 0 (;@3;)
          i32.const 0
          local.set 2
          br 2 (;@1;)
        end
        local.get 2
        i32.const -1
        i32.add
        local.set 2
      end
      local.get 2
      i32.const 2
      i32.shl
      i32.const 1062728
      i32.add
      i32.load
      i32.const 2097151
      i32.and
      local.set 2
    end
    block ;; label = @1
      block ;; label = @2
        local.get 5
        local.get 1
        i32.const -1
        i32.xor
        i32.add
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        local.get 2
        i32.sub
        local.set 4
        local.get 1
        i32.const 727
        local.get 1
        i32.const 727
        i32.gt_u
        select
        local.set 3
        local.get 5
        i32.const -1
        i32.add
        local.set 5
        i32.const 0
        local.set 2
        loop ;; label = @3
          local.get 3
          local.get 1
          i32.eq
          br_if 2 (;@1;)
          local.get 2
          local.get 1
          i32.const 1062860
          i32.add
          i32.load8_u
          i32.add
          local.tee 2
          local.get 4
          i32.gt_u
          br_if 1 (;@2;)
          local.get 5
          local.get 1
          i32.const 1
          i32.add
          local.tee 1
          i32.ne
          br_if 0 (;@3;)
        end
        local.get 5
        local.set 1
      end
      local.get 1
      i32.const 1
      i32.and
      return
    end
    local.get 3
    i32.const 727
    i32.const 1062648
    call $_ZN4core9panicking18panic_bounds_check17he2ca869b88362af2E
    unreachable
  )
  (func $_ZN4core7unicode9printable12is_printable17h8c88fda38b5ef3b1E (;208;) (type 12) (param i32) (result i32)
    (local i32)
    block ;; label = @1
      local.get 0
      i32.const 32
      i32.ge_u
      br_if 0 (;@1;)
      i32.const 0
      return
    end
    i32.const 1
    local.set 1
    block ;; label = @1
      block ;; label = @2
        local.get 0
        i32.const 127
        i32.lt_u
        br_if 0 (;@2;)
        local.get 0
        i32.const 65536
        i32.lt_u
        br_if 1 (;@1;)
        block ;; label = @3
          block ;; label = @4
            local.get 0
            i32.const 131072
            i32.lt_u
            br_if 0 (;@4;)
            block ;; label = @5
              local.get 0
              i32.const -205744
              i32.add
              i32.const 712016
              i32.ge_u
              br_if 0 (;@5;)
              i32.const 0
              return
            end
            block ;; label = @5
              local.get 0
              i32.const -201547
              i32.add
              i32.const 5
              i32.ge_u
              br_if 0 (;@5;)
              i32.const 0
              return
            end
            block ;; label = @5
              local.get 0
              i32.const -195102
              i32.add
              i32.const 1506
              i32.ge_u
              br_if 0 (;@5;)
              i32.const 0
              return
            end
            block ;; label = @5
              local.get 0
              i32.const -191457
              i32.add
              i32.const 3103
              i32.ge_u
              br_if 0 (;@5;)
              i32.const 0
              return
            end
            block ;; label = @5
              local.get 0
              i32.const -183970
              i32.add
              i32.const 14
              i32.ge_u
              br_if 0 (;@5;)
              i32.const 0
              return
            end
            block ;; label = @5
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
          i32.const 1061188
          i32.const 44
          i32.const 1061276
          i32.const 196
          i32.const 1061472
          i32.const 450
          call $_ZN4core7unicode9printable5check17h6f3ea5e955a4153fE
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
    i32.const 1061922
    i32.const 40
    i32.const 1062002
    i32.const 287
    i32.const 1062289
    i32.const 303
    call $_ZN4core7unicode9printable5check17h6f3ea5e955a4153fE
  )
  (func $_ZN4core3ffi5c_str4CStr19from_bytes_with_nul17h5393327c07cb4c27E (;209;) (type 3) (param i32 i32 i32)
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
  (func $_ZN4core3str8converts9from_utf817h3661e58cc325c35bE (;210;) (type 3) (param i32 i32 i32)
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
                                      i32.const 1060592
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
  (func $_ZN4core3fmt8builders11DebugStruct5field17h262f149dc4e3bf7dE (;211;) (type 17) (param i32 i32 i32 i32 i32) (result i32)
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
        i32.const 1060055
        i32.const 1060052
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
        i32.const 1060004
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
        i32.const 1060057
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
      i32.const 1060024
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
      call $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h1bc5e3fd4b32d905E
      br_if 0 (;@1;)
      local.get 5
      i32.const 12
      i32.add
      i32.const 1060004
      i32.const 2
      call $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h1bc5e3fd4b32d905E
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
      i32.const 1060060
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
  (func $_ZN4core3fmt3num3imp51_$LT$impl$u20$core..fmt..Display$u20$for$u20$u8$GT$3fmt17h429bcbf4930d92a2E (;212;) (type 2) (param i32 i32) (result i32)
    local.get 0
    i64.load8_u
    i32.const 1
    local.get 1
    call $_ZN4core3fmt3num3imp7fmt_u6417ha7c93bd38c403be2E
  )
  (func $_ZN4core6result13unwrap_failed17h7812484c33dfa842E (;213;) (type 8) (param i32 i32 i32 i32 i32)
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
    i32.const 75
    i32.store
    local.get 5
    i32.const 2
    i32.store offset=28
    local.get 5
    i32.const 1060008
    i32.store offset=24
    local.get 5
    i32.const 76
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
    call $_ZN4core9panicking9panic_fmt17h4c9c94dcfde83250E
    unreachable
  )
  (func $_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u16$GT$3fmt17hc68eef1bf3b24f50E (;214;) (type 2) (param i32 i32) (result i32)
    local.get 0
    i64.load16_u
    i32.const 1
    local.get 1
    call $_ZN4core3fmt3num3imp7fmt_u6417ha7c93bd38c403be2E
  )
  (func $_ZN4core6option13expect_failed17h7690befb2bc651caE (;215;) (type 3) (param i32 i32 i32)
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
    i32.const 1059724
    i32.store offset=16
    local.get 3
    i32.const 76
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
    call $_ZN4core9panicking9panic_fmt17h4c9c94dcfde83250E
    unreachable
  )
  (func $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hbcfedd02189591b0E (;216;) (type 2) (param i32 i32) (result i32)
    local.get 1
    local.get 0
    i32.load
    local.get 0
    i32.load offset=4
    call $_ZN4core3fmt9Formatter3pad17h58189de40b979c18E
  )
  (func $_ZN70_$LT$core..panic..location..Location$u20$as$u20$core..fmt..Display$GT$3fmt17h5a1f046e61dd743eE (;217;) (type 2) (param i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    i32.const 44
    i32.add
    i32.const 17
    i32.store
    local.get 2
    i32.const 24
    i32.add
    i32.const 12
    i32.add
    i32.const 17
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
    i32.const 1059736
    i32.store
    local.get 2
    i32.const 76
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
    call $_ZN4core3fmt5write17h890955524eea605cE
    local.set 0
    local.get 2
    i32.const 48
    i32.add
    global.set $__stack_pointer
    local.get 0
  )
  (func $_ZN73_$LT$core..panic..panic_info..PanicInfo$u20$as$u20$core..fmt..Display$GT$3fmt17hf057b4be12e8885dE (;218;) (type 2) (param i32 i32) (result i32)
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
      i32.const 1059760
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
      i32.const 17
      i32.store
      local.get 2
      i32.const 40
      i32.add
      i32.const 12
      i32.add
      i32.const 17
      i32.store
      local.get 2
      i32.const 3
      i32.store offset=20
      local.get 2
      i32.const 1059736
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
      i32.const 76
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
      call $_ZN4core3fmt5write17h890955524eea605cE
      br_if 0 (;@1;)
      block ;; label = @2
        block ;; label = @3
          local.get 0
          i32.load offset=8
          local.tee 1
          i32.eqz
          br_if 0 (;@3;)
          local.get 4
          i32.const 1059772
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
          call $_ZN4core3fmt5write17h890955524eea605cE
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
        i32.const 1059772
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
  (func $_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i32$GT$3fmt17hff8d8204aa9c9138E (;219;) (type 2) (param i32 i32) (result i32)
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
      i32.const 1060116
      call $_ZN4core5slice5index26slice_start_index_len_fail17h43cf12d46dc03083E
      unreachable
    end
    local.get 1
    i32.const 1
    i32.const 1060112
    i32.const 2
    local.get 2
    local.get 3
    i32.add
    i32.const 128
    i32.add
    i32.const 0
    local.get 3
    i32.sub
    call $_ZN4core3fmt9Formatter12pad_integral17ha8545ce28396770eE
    local.set 0
    local.get 2
    i32.const 128
    i32.add
    global.set $__stack_pointer
    local.get 0
  )
  (func $_ZN4core9panicking19assert_failed_inner17h028fb57387c98e3fE (;220;) (type 18) (param i32 i32 i32 i32 i32 i32 i32)
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
          i32.const 1059860
          i32.store offset=24
          i32.const 2
          local.set 2
          br 2 (;@1;)
        end
        local.get 7
        i32.const 1059862
        i32.store offset=24
        i32.const 2
        local.set 2
        br 1 (;@1;)
      end
      local.get 7
      i32.const 1059864
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
      i32.const 75
      i32.store
      local.get 7
      i32.const 56
      i32.add
      i32.const 12
      i32.add
      i32.const 75
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
      i32.const 1059920
      i32.store offset=88
      local.get 7
      i32.const 76
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
      call $_ZN4core9panicking9panic_fmt17h4c9c94dcfde83250E
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
    i32.const 75
    i32.store
    local.get 7
    i32.const 76
    i32.add
    i32.const 75
    i32.store
    local.get 7
    i32.const 56
    i32.add
    i32.const 12
    i32.add
    i32.const 77
    i32.store
    local.get 7
    i32.const 4
    i32.store offset=92
    local.get 7
    i32.const 1059972
    i32.store offset=88
    local.get 7
    i32.const 76
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
    call $_ZN4core9panicking9panic_fmt17h4c9c94dcfde83250E
    unreachable
  )
  (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h08b181ec1a3a2818E (;221;) (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 1
    local.get 0
    i32.load offset=4
    i32.load offset=12
    call_indirect (type 2)
  )
  (func $_ZN59_$LT$core..fmt..Arguments$u20$as$u20$core..fmt..Display$GT$3fmt17hf432543ead310655E (;222;) (type 2) (param i32 i32) (result i32)
    local.get 1
    i32.load offset=20
    local.get 1
    i32.const 24
    i32.add
    i32.load
    local.get 0
    call $_ZN4core3fmt5write17h890955524eea605cE
  )
  (func $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h1bc5e3fd4b32d905E (;223;) (type 4) (param i32 i32 i32) (result i32)
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
            i32.const 1060048
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
  (func $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$10write_char17ha3700472f18f7184E (;224;) (type 2) (param i32 i32) (result i32)
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
      i32.const 1060048
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
  (func $_ZN4core3fmt8builders11DebugStruct21finish_non_exhaustive17h99ed4ae75088b9afE (;225;) (type 12) (param i32) (result i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    i32.const 1
    local.set 2
    block ;; label = @1
      local.get 0
      i32.load8_u offset=4
      br_if 0 (;@1;)
      local.get 0
      i32.load
      local.set 3
      block ;; label = @2
        local.get 0
        i32.load8_u offset=5
        br_if 0 (;@2;)
        local.get 3
        i32.load offset=20
        i32.const 1060062
        i32.const 7
        local.get 3
        i32.const 24
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 4)
        local.set 2
        br 1 (;@1;)
      end
      block ;; label = @2
        local.get 3
        i32.load8_u offset=28
        i32.const 4
        i32.and
        br_if 0 (;@2;)
        local.get 3
        i32.load offset=20
        i32.const 1060069
        i32.const 6
        local.get 3
        i32.const 24
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 4)
        local.set 2
        br 1 (;@1;)
      end
      i32.const 1
      local.set 2
      local.get 1
      i32.const 1
      i32.store8 offset=15
      local.get 1
      local.get 3
      i64.load offset=20 align=4
      i64.store align=4
      local.get 1
      local.get 1
      i32.const 15
      i32.add
      i32.store offset=8
      local.get 1
      i32.const 1060075
      i32.const 3
      call $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h1bc5e3fd4b32d905E
      br_if 0 (;@1;)
      local.get 3
      i32.load offset=20
      i32.const 1060078
      i32.const 1
      local.get 3
      i32.load offset=24
      i32.load offset=12
      call_indirect (type 4)
      local.set 2
    end
    local.get 0
    local.get 2
    i32.store8 offset=4
    local.get 1
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 2
  )
  (func $_ZN4core3fmt8builders11DebugStruct6finish17haaefede11847f5d5E (;226;) (type 12) (param i32) (result i32)
    (local i32 i32)
    local.get 0
    i32.load8_u offset=4
    local.set 1
    block ;; label = @1
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
    block ;; label = @1
      local.get 1
      i32.const 255
      i32.and
      br_if 0 (;@1;)
      block ;; label = @2
        local.get 0
        i32.load
        local.tee 1
        i32.load8_u offset=28
        i32.const 4
        i32.and
        br_if 0 (;@2;)
        local.get 0
        local.get 1
        i32.load offset=20
        i32.const 1060079
        i32.const 2
        local.get 1
        i32.load offset=24
        i32.load offset=12
        call_indirect (type 4)
        local.tee 1
        i32.store8 offset=4
        local.get 1
        return
      end
      local.get 1
      i32.load offset=20
      i32.const 1060078
      i32.const 1
      local.get 1
      i32.load offset=24
      i32.load offset=12
      call_indirect (type 4)
      local.set 2
    end
    local.get 0
    local.get 2
    i32.store8 offset=4
    local.get 2
  )
  (func $_ZN4core3fmt8builders10DebugTuple5field17haa05b3fae70e9280E (;227;) (type 4) (param i32 i32 i32) (result i32)
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
        i32.const 1060055
        i32.const 1060081
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
        i32.const 1060082
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
      i32.const 1060024
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
      i32.const 1060060
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
  (func $_ZN4core3fmt9Formatter12pad_integral17ha8545ce28396770eE (;228;) (type 19) (param i32 i32 i32 i32 i32 i32) (result i32)
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
          call $_ZN4core3str5count14do_count_chars17h2c35506888f0ac80E
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
        call $_ZN4core3fmt9Formatter12pad_integral12write_prefix17h9f4827fb315f246bE
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
        call $_ZN4core3fmt9Formatter12pad_integral12write_prefix17h9f4827fb315f246bE
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
        call $_ZN4core3fmt9Formatter12pad_integral12write_prefix17h9f4827fb315f246bE
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
      call $_ZN4core3fmt9Formatter12pad_integral12write_prefix17h9f4827fb315f246bE
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
  (func $_ZN4core3fmt5Write9write_fmt17h33566f065e9aa63aE (;229;) (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.const 1060024
    local.get 1
    call $_ZN4core3fmt5write17h890955524eea605cE
  )
  (func $_ZN4core3str5count14do_count_chars17h2c35506888f0ac80E (;230;) (type 2) (param i32 i32) (result i32)
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
  (func $_ZN4core3fmt9Formatter12pad_integral12write_prefix17h9f4827fb315f246bE (;231;) (type 17) (param i32 i32 i32 i32 i32) (result i32)
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
  (func $_ZN4core3fmt9Formatter9write_str17h4bf1acaddf72a444E (;232;) (type 4) (param i32 i32 i32) (result i32)
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
  (func $_ZN4core3fmt9Formatter9write_fmt17h83b5a1707d5b6e2cE (;233;) (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load offset=20
    local.get 0
    i32.const 24
    i32.add
    i32.load
    local.get 1
    call $_ZN4core3fmt5write17h890955524eea605cE
  )
  (func $_ZN4core3fmt9Formatter12debug_struct17h8dcb4ffc7ee470c3E (;234;) (type 7) (param i32 i32 i32 i32)
    local.get 1
    i32.load offset=20
    local.get 2
    local.get 3
    local.get 1
    i32.const 24
    i32.add
    i32.load
    i32.load offset=12
    call_indirect (type 4)
    local.set 3
    local.get 0
    i32.const 0
    i32.store8 offset=5
    local.get 0
    local.get 3
    i32.store8 offset=4
    local.get 0
    local.get 1
    i32.store
  )
  (func $_ZN4core3fmt9Formatter26debug_struct_field2_finish17h44f11ac8f3608eacE (;235;) (type 20) (param i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32) (result i32)
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
    call $_ZN4core3fmt8builders11DebugStruct5field17h262f149dc4e3bf7dE
    local.get 7
    local.get 8
    local.get 9
    local.get 10
    call $_ZN4core3fmt8builders11DebugStruct5field17h262f149dc4e3bf7dE
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
        i32.const 1060079
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
      i32.const 1060078
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
  (func $_ZN4core3fmt9Formatter25debug_tuple_field1_finish17h523899074bd3629cE (;236;) (type 17) (param i32 i32 i32 i32 i32) (result i32)
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
    call $_ZN4core3fmt8builders10DebugTuple5field17haa05b3fae70e9280E
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
        i32.const 1060084
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
      i32.const 1059654
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
  (func $_ZN43_$LT$bool$u20$as$u20$core..fmt..Display$GT$3fmt17hd1b099dc6a4f6226E (;237;) (type 2) (param i32 i32) (result i32)
    block ;; label = @1
      local.get 0
      i32.load8_u
      br_if 0 (;@1;)
      local.get 1
      i32.const 1060332
      i32.const 5
      call $_ZN4core3fmt9Formatter3pad17h58189de40b979c18E
      return
    end
    local.get 1
    i32.const 1060337
    i32.const 4
    call $_ZN4core3fmt9Formatter3pad17h58189de40b979c18E
  )
  (func $_ZN40_$LT$str$u20$as$u20$core..fmt..Debug$GT$3fmt17h839fd847cc958bcdE (;238;) (type 4) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i64 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    i32.const 1
    local.set 4
    block ;; label = @1
      block ;; label = @2
        local.get 2
        i32.load offset=20
        local.tee 5
        i32.const 34
        local.get 2
        i32.const 24
        i32.add
        i32.load
        local.tee 6
        i32.load offset=16
        local.tee 7
        call_indirect (type 2)
        br_if 0 (;@2;)
        block ;; label = @3
          block ;; label = @4
            local.get 1
            br_if 0 (;@4;)
            i32.const 0
            local.set 2
            i32.const 0
            local.set 1
            br 1 (;@3;)
          end
          local.get 0
          local.get 1
          i32.add
          local.set 8
          i32.const 0
          local.set 2
          local.get 0
          local.set 9
          i32.const 0
          local.set 10
          block ;; label = @4
            block ;; label = @5
              loop ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    local.get 9
                    local.tee 11
                    i32.load8_s
                    local.tee 12
                    i32.const -1
                    i32.le_s
                    br_if 0 (;@8;)
                    local.get 11
                    i32.const 1
                    i32.add
                    local.set 9
                    local.get 12
                    i32.const 255
                    i32.and
                    local.set 13
                    br 1 (;@7;)
                  end
                  local.get 11
                  i32.load8_u offset=1
                  i32.const 63
                  i32.and
                  local.set 14
                  local.get 12
                  i32.const 31
                  i32.and
                  local.set 15
                  block ;; label = @8
                    local.get 12
                    i32.const -33
                    i32.gt_u
                    br_if 0 (;@8;)
                    local.get 15
                    i32.const 6
                    i32.shl
                    local.get 14
                    i32.or
                    local.set 13
                    local.get 11
                    i32.const 2
                    i32.add
                    local.set 9
                    br 1 (;@7;)
                  end
                  local.get 14
                  i32.const 6
                  i32.shl
                  local.get 11
                  i32.load8_u offset=2
                  i32.const 63
                  i32.and
                  i32.or
                  local.set 14
                  local.get 11
                  i32.const 3
                  i32.add
                  local.set 9
                  block ;; label = @8
                    local.get 12
                    i32.const -16
                    i32.ge_u
                    br_if 0 (;@8;)
                    local.get 14
                    local.get 15
                    i32.const 12
                    i32.shl
                    i32.or
                    local.set 13
                    br 1 (;@7;)
                  end
                  local.get 14
                  i32.const 6
                  i32.shl
                  local.get 9
                  i32.load8_u
                  i32.const 63
                  i32.and
                  i32.or
                  local.get 15
                  i32.const 18
                  i32.shl
                  i32.const 1835008
                  i32.and
                  i32.or
                  local.tee 13
                  i32.const 1114112
                  i32.eq
                  br_if 3 (;@4;)
                  local.get 11
                  i32.const 4
                  i32.add
                  local.set 9
                end
                local.get 3
                i32.const 4
                i32.add
                local.get 13
                i32.const 65537
                call $_ZN4core4char7methods22_$LT$impl$u20$char$GT$16escape_debug_ext17h60cdf599b8c6d176E
                block ;; label = @7
                  block ;; label = @8
                    local.get 3
                    i32.load8_u offset=4
                    i32.const 128
                    i32.eq
                    br_if 0 (;@8;)
                    local.get 3
                    i32.load8_u offset=15
                    local.get 3
                    i32.load8_u offset=14
                    i32.sub
                    i32.const 255
                    i32.and
                    i32.const 1
                    i32.eq
                    br_if 0 (;@8;)
                    local.get 10
                    local.get 2
                    i32.lt_u
                    br_if 3 (;@5;)
                    block ;; label = @9
                      local.get 2
                      i32.eqz
                      br_if 0 (;@9;)
                      block ;; label = @10
                        local.get 2
                        local.get 1
                        i32.lt_u
                        br_if 0 (;@10;)
                        local.get 2
                        local.get 1
                        i32.eq
                        br_if 1 (;@9;)
                        br 5 (;@5;)
                      end
                      local.get 0
                      local.get 2
                      i32.add
                      i32.load8_s
                      i32.const -64
                      i32.lt_s
                      br_if 4 (;@5;)
                    end
                    block ;; label = @9
                      local.get 10
                      i32.eqz
                      br_if 0 (;@9;)
                      block ;; label = @10
                        local.get 10
                        local.get 1
                        i32.lt_u
                        br_if 0 (;@10;)
                        local.get 10
                        local.get 1
                        i32.eq
                        br_if 1 (;@9;)
                        br 5 (;@5;)
                      end
                      local.get 0
                      local.get 10
                      i32.add
                      i32.load8_s
                      i32.const -65
                      i32.le_s
                      br_if 4 (;@5;)
                    end
                    block ;; label = @9
                      block ;; label = @10
                        local.get 5
                        local.get 0
                        local.get 2
                        i32.add
                        local.get 10
                        local.get 2
                        i32.sub
                        local.get 6
                        i32.load offset=12
                        call_indirect (type 4)
                        br_if 0 (;@10;)
                        local.get 3
                        i32.const 16
                        i32.add
                        i32.const 8
                        i32.add
                        local.tee 15
                        local.get 3
                        i32.const 4
                        i32.add
                        i32.const 8
                        i32.add
                        i32.load
                        i32.store
                        local.get 3
                        local.get 3
                        i64.load offset=4 align=4
                        local.tee 16
                        i64.store offset=16
                        block ;; label = @11
                          local.get 16
                          i32.wrap_i64
                          i32.const 255
                          i32.and
                          i32.const 128
                          i32.ne
                          br_if 0 (;@11;)
                          i32.const 128
                          local.set 14
                          loop ;; label = @12
                            block ;; label = @13
                              block ;; label = @14
                                local.get 14
                                i32.const 255
                                i32.and
                                i32.const 128
                                i32.eq
                                br_if 0 (;@14;)
                                local.get 3
                                i32.load8_u offset=26
                                local.tee 12
                                local.get 3
                                i32.load8_u offset=27
                                i32.ge_u
                                br_if 5 (;@9;)
                                local.get 3
                                local.get 12
                                i32.const 1
                                i32.add
                                i32.store8 offset=26
                                local.get 12
                                i32.const 10
                                i32.ge_u
                                br_if 7 (;@7;)
                                local.get 3
                                i32.const 16
                                i32.add
                                local.get 12
                                i32.add
                                i32.load8_u
                                local.set 2
                                br 1 (;@13;)
                              end
                              i32.const 0
                              local.set 14
                              local.get 15
                              i32.const 0
                              i32.store
                              local.get 3
                              i32.load offset=20
                              local.set 2
                              local.get 3
                              i64.const 0
                              i64.store offset=16
                            end
                            local.get 5
                            local.get 2
                            local.get 7
                            call_indirect (type 2)
                            i32.eqz
                            br_if 0 (;@12;)
                            br 2 (;@10;)
                          end
                        end
                        local.get 3
                        i32.load8_u offset=26
                        local.tee 2
                        i32.const 10
                        local.get 2
                        i32.const 10
                        i32.gt_u
                        select
                        local.set 12
                        local.get 3
                        i32.load8_u offset=27
                        local.tee 14
                        local.get 2
                        local.get 14
                        local.get 2
                        i32.gt_u
                        select
                        local.set 17
                        loop ;; label = @11
                          local.get 17
                          local.get 2
                          i32.eq
                          br_if 2 (;@9;)
                          local.get 3
                          local.get 2
                          i32.const 1
                          i32.add
                          local.tee 14
                          i32.store8 offset=26
                          local.get 12
                          local.get 2
                          i32.eq
                          br_if 4 (;@7;)
                          local.get 3
                          i32.const 16
                          i32.add
                          local.get 2
                          i32.add
                          local.set 15
                          local.get 14
                          local.set 2
                          local.get 5
                          local.get 15
                          i32.load8_u
                          local.get 7
                          call_indirect (type 2)
                          i32.eqz
                          br_if 0 (;@11;)
                        end
                      end
                      i32.const 1
                      local.set 4
                      br 7 (;@2;)
                    end
                    i32.const 1
                    local.set 2
                    block ;; label = @9
                      local.get 13
                      i32.const 128
                      i32.lt_u
                      br_if 0 (;@9;)
                      i32.const 2
                      local.set 2
                      local.get 13
                      i32.const 2048
                      i32.lt_u
                      br_if 0 (;@9;)
                      i32.const 3
                      i32.const 4
                      local.get 13
                      i32.const 65536
                      i32.lt_u
                      select
                      local.set 2
                    end
                    local.get 2
                    local.get 10
                    i32.add
                    local.set 2
                  end
                  local.get 10
                  local.get 11
                  i32.sub
                  local.get 9
                  i32.add
                  local.set 10
                  local.get 9
                  local.get 8
                  i32.ne
                  br_if 1 (;@6;)
                  br 3 (;@4;)
                end
              end
              local.get 12
              i32.const 10
              i32.const 1062712
              call $_ZN4core9panicking18panic_bounds_check17he2ca869b88362af2E
              unreachable
            end
            local.get 0
            local.get 1
            local.get 2
            local.get 10
            i32.const 1060360
            call $_ZN4core3str16slice_error_fail17hd99937096c80f263E
            unreachable
          end
          block ;; label = @4
            local.get 2
            br_if 0 (;@4;)
            i32.const 0
            local.set 2
            br 1 (;@3;)
          end
          block ;; label = @4
            block ;; label = @5
              local.get 1
              local.get 2
              i32.gt_u
              br_if 0 (;@5;)
              local.get 1
              local.get 2
              i32.eq
              br_if 1 (;@4;)
              br 4 (;@1;)
            end
            local.get 0
            local.get 2
            i32.add
            i32.load8_s
            i32.const -65
            i32.le_s
            br_if 3 (;@1;)
          end
          local.get 1
          local.get 2
          i32.sub
          local.set 1
        end
        local.get 5
        local.get 0
        local.get 2
        i32.add
        local.get 1
        local.get 6
        i32.load offset=12
        call_indirect (type 4)
        br_if 0 (;@2;)
        local.get 5
        i32.const 34
        local.get 7
        call_indirect (type 2)
        local.set 4
      end
      local.get 3
      i32.const 32
      i32.add
      global.set $__stack_pointer
      local.get 4
      return
    end
    local.get 0
    local.get 1
    local.get 2
    local.get 1
    i32.const 1060344
    call $_ZN4core3str16slice_error_fail17hd99937096c80f263E
    unreachable
  )
  (func $_ZN4core3str16slice_error_fail17hd99937096c80f263E (;239;) (type 8) (param i32 i32 i32 i32 i32)
    local.get 0
    local.get 1
    local.get 2
    local.get 3
    local.get 4
    call $_ZN4core3str19slice_error_fail_rt17heaeca9d790972d55E
    unreachable
  )
  (func $_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17h33dd62bf170497e3E (;240;) (type 4) (param i32 i32 i32) (result i32)
    local.get 2
    local.get 0
    local.get 1
    call $_ZN4core3fmt9Formatter3pad17h58189de40b979c18E
  )
  (func $_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17hd09ca3de828ddddaE (;241;) (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    i32.const 1
    local.set 3
    block ;; label = @1
      block ;; label = @2
        local.get 1
        i32.load offset=20
        local.tee 4
        i32.const 39
        local.get 1
        i32.const 24
        i32.add
        i32.load
        i32.load offset=16
        local.tee 5
        call_indirect (type 2)
        br_if 0 (;@2;)
        local.get 2
        local.get 0
        i32.load
        i32.const 257
        call $_ZN4core4char7methods22_$LT$impl$u20$char$GT$16escape_debug_ext17h60cdf599b8c6d176E
        block ;; label = @3
          block ;; label = @4
            local.get 2
            i32.load8_u
            i32.const 128
            i32.ne
            br_if 0 (;@4;)
            local.get 2
            i32.const 8
            i32.add
            local.set 6
            i32.const 128
            local.set 7
            loop ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  local.get 7
                  i32.const 255
                  i32.and
                  i32.const 128
                  i32.eq
                  br_if 0 (;@7;)
                  local.get 2
                  i32.load8_u offset=10
                  local.tee 0
                  local.get 2
                  i32.load8_u offset=11
                  i32.ge_u
                  br_if 4 (;@3;)
                  local.get 2
                  local.get 0
                  i32.const 1
                  i32.add
                  i32.store8 offset=10
                  local.get 0
                  i32.const 10
                  i32.ge_u
                  br_if 6 (;@1;)
                  local.get 2
                  local.get 0
                  i32.add
                  i32.load8_u
                  local.set 1
                  br 1 (;@6;)
                end
                i32.const 0
                local.set 7
                local.get 6
                i32.const 0
                i32.store
                local.get 2
                i32.load offset=4
                local.set 1
                local.get 2
                i64.const 0
                i64.store
              end
              local.get 4
              local.get 1
              local.get 5
              call_indirect (type 2)
              i32.eqz
              br_if 0 (;@5;)
              br 3 (;@2;)
            end
          end
          local.get 2
          i32.load8_u offset=10
          local.tee 1
          i32.const 10
          local.get 1
          i32.const 10
          i32.gt_u
          select
          local.set 0
          local.get 2
          i32.load8_u offset=11
          local.tee 7
          local.get 1
          local.get 7
          local.get 1
          i32.gt_u
          select
          local.set 8
          loop ;; label = @4
            local.get 8
            local.get 1
            i32.eq
            br_if 1 (;@3;)
            local.get 2
            local.get 1
            i32.const 1
            i32.add
            local.tee 7
            i32.store8 offset=10
            local.get 0
            local.get 1
            i32.eq
            br_if 3 (;@1;)
            local.get 2
            local.get 1
            i32.add
            local.set 6
            local.get 7
            local.set 1
            local.get 4
            local.get 6
            i32.load8_u
            local.get 5
            call_indirect (type 2)
            i32.eqz
            br_if 0 (;@4;)
            br 2 (;@2;)
          end
        end
        local.get 4
        i32.const 39
        local.get 5
        call_indirect (type 2)
        local.set 3
      end
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 3
      return
    end
    local.get 0
    i32.const 10
    i32.const 1062712
    call $_ZN4core9panicking18panic_bounds_check17he2ca869b88362af2E
    unreachable
  )
  (func $_ZN4core5slice6memchr14memchr_aligned17h4eaec85e20476149E (;242;) (type 7) (param i32 i32 i32 i32)
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
  (func $_ZN4core5slice6memchr7memrchr17hba53988255f8572cE (;243;) (type 7) (param i32 i32 i32 i32)
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
          i32.const 1060424
          call $_ZN4core5slice5index26slice_start_index_len_fail17h43cf12d46dc03083E
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
        i32.const 1060408
        call $_ZN4core5slice5index24slice_end_index_len_fail17h206e334eab3e7498E
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
  (func $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i8$GT$3fmt17he59c9ce34d177f0eE (;244;) (type 2) (param i32 i32) (result i32)
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
      i32.const 1060116
      call $_ZN4core5slice5index26slice_start_index_len_fail17h43cf12d46dc03083E
      unreachable
    end
    local.get 1
    i32.const 1
    i32.const 1060112
    i32.const 2
    local.get 2
    local.get 0
    i32.add
    i32.const 128
    i32.add
    i32.const 0
    local.get 0
    i32.sub
    call $_ZN4core3fmt9Formatter12pad_integral17ha8545ce28396770eE
    local.set 0
    local.get 2
    i32.const 128
    i32.add
    global.set $__stack_pointer
    local.get 0
  )
  (func $_ZN4core3str19slice_error_fail_rt17heaeca9d790972d55E (;245;) (type 8) (param i32 i32 i32 i32 i32)
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
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 1
          i32.const 257
          i32.lt_u
          br_if 0 (;@3;)
          i32.const 256
          local.set 6
          block ;; label = @4
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
          block ;; label = @4
            block ;; label = @5
              local.get 6
              local.get 1
              i32.lt_u
              local.tee 7
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
          local.get 5
          local.get 0
          i32.store offset=16
          local.get 5
          local.get 6
          i32.store offset=20
          i32.const 5
          i32.const 0
          local.get 7
          select
          local.set 6
          i32.const 1060848
          i32.const 1059584
          local.get 7
          select
          local.set 7
          br 1 (;@2;)
        end
        local.get 5
        local.get 1
        i32.store offset=20
        local.get 5
        local.get 0
        i32.store offset=16
        i32.const 0
        local.set 6
        i32.const 1059584
        local.set 7
      end
      local.get 5
      local.get 6
      i32.store offset=28
      local.get 5
      local.get 7
      i32.store offset=24
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                local.get 2
                local.get 1
                i32.gt_u
                local.tee 6
                br_if 0 (;@6;)
                local.get 3
                local.get 1
                i32.gt_u
                br_if 0 (;@6;)
                local.get 2
                local.get 3
                i32.gt_u
                br_if 1 (;@5;)
                block ;; label = @7
                  block ;; label = @8
                    local.get 2
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 2
                    local.get 1
                    i32.ge_u
                    br_if 0 (;@8;)
                    local.get 0
                    local.get 2
                    i32.add
                    i32.load8_s
                    i32.const -64
                    i32.lt_s
                    br_if 1 (;@7;)
                  end
                  local.get 3
                  local.set 2
                end
                local.get 5
                local.get 2
                i32.store offset=32
                local.get 1
                local.set 3
                block ;; label = @7
                  local.get 2
                  local.get 1
                  i32.ge_u
                  br_if 0 (;@7;)
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
                  local.get 2
                  i32.const 1
                  i32.add
                  local.tee 6
                  i32.gt_u
                  br_if 3 (;@4;)
                  block ;; label = @8
                    local.get 3
                    local.get 6
                    i32.eq
                    br_if 0 (;@8;)
                    local.get 0
                    local.get 6
                    i32.add
                    local.get 0
                    local.get 3
                    i32.add
                    local.tee 8
                    i32.sub
                    local.set 6
                    block ;; label = @9
                      local.get 0
                      local.get 2
                      i32.add
                      local.tee 9
                      i32.load8_s
                      i32.const -65
                      i32.le_s
                      br_if 0 (;@9;)
                      local.get 6
                      i32.const -1
                      i32.add
                      local.set 7
                      br 1 (;@8;)
                    end
                    local.get 3
                    local.get 2
                    i32.eq
                    br_if 0 (;@8;)
                    block ;; label = @9
                      local.get 9
                      i32.const -1
                      i32.add
                      local.tee 2
                      i32.load8_s
                      i32.const -65
                      i32.le_s
                      br_if 0 (;@9;)
                      local.get 6
                      i32.const -2
                      i32.add
                      local.set 7
                      br 1 (;@8;)
                    end
                    local.get 8
                    local.get 2
                    i32.eq
                    br_if 0 (;@8;)
                    block ;; label = @9
                      local.get 9
                      i32.const -2
                      i32.add
                      local.tee 2
                      i32.load8_s
                      i32.const -65
                      i32.le_s
                      br_if 0 (;@9;)
                      local.get 6
                      i32.const -3
                      i32.add
                      local.set 7
                      br 1 (;@8;)
                    end
                    local.get 8
                    local.get 2
                    i32.eq
                    br_if 0 (;@8;)
                    block ;; label = @9
                      local.get 9
                      i32.const -3
                      i32.add
                      local.tee 2
                      i32.load8_s
                      i32.const -65
                      i32.le_s
                      br_if 0 (;@9;)
                      local.get 6
                      i32.const -4
                      i32.add
                      local.set 7
                      br 1 (;@8;)
                    end
                    local.get 8
                    local.get 2
                    i32.eq
                    br_if 0 (;@8;)
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
                block ;; label = @7
                  local.get 3
                  i32.eqz
                  br_if 0 (;@7;)
                  block ;; label = @8
                    block ;; label = @9
                      local.get 1
                      local.get 3
                      i32.gt_u
                      br_if 0 (;@9;)
                      local.get 1
                      local.get 3
                      i32.eq
                      br_if 1 (;@8;)
                      br 7 (;@2;)
                    end
                    local.get 0
                    local.get 3
                    i32.add
                    i32.load8_s
                    i32.const -65
                    i32.le_s
                    br_if 6 (;@2;)
                  end
                  local.get 1
                  local.get 3
                  i32.sub
                  local.set 1
                end
                local.get 1
                i32.eqz
                br_if 3 (;@3;)
                block ;; label = @7
                  block ;; label = @8
                    block ;; label = @9
                      block ;; label = @10
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
                        br 2 (;@8;)
                      end
                      local.get 5
                      local.get 1
                      i32.const 255
                      i32.and
                      i32.store offset=36
                      i32.const 1
                      local.set 1
                      br 2 (;@7;)
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
                    block ;; label = @9
                      local.get 1
                      i32.const -16
                      i32.ge_u
                      br_if 0 (;@9;)
                      local.get 0
                      local.get 6
                      i32.const 12
                      i32.shl
                      i32.or
                      local.set 2
                      br 1 (;@8;)
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
                    br_if 5 (;@3;)
                  end
                  local.get 5
                  local.get 2
                  i32.store offset=36
                  i32.const 1
                  local.set 1
                  local.get 2
                  i32.const 128
                  i32.lt_u
                  br_if 0 (;@7;)
                  i32.const 2
                  local.set 1
                  local.get 2
                  i32.const 2048
                  i32.lt_u
                  br_if 0 (;@7;)
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
                i64.const 5
                i64.store align=4
                local.get 5
                i32.const 108
                i32.add
                i32.const 76
                i32.store
                local.get 5
                i32.const 100
                i32.add
                i32.const 76
                i32.store
                local.get 5
                i32.const 92
                i32.add
                i32.const 78
                i32.store
                local.get 5
                i32.const 72
                i32.add
                i32.const 12
                i32.add
                i32.const 79
                i32.store
                local.get 5
                i32.const 5
                i32.store offset=52
                local.get 5
                i32.const 1060984
                i32.store offset=48
                local.get 5
                i32.const 17
                i32.store offset=76
                local.get 5
                local.get 5
                i32.const 72
                i32.add
                i32.store offset=56
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
                call $_ZN4core9panicking9panic_fmt17h4c9c94dcfde83250E
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
              i64.const 3
              i64.store align=4
              local.get 5
              i32.const 92
              i32.add
              i32.const 76
              i32.store
              local.get 5
              i32.const 72
              i32.add
              i32.const 12
              i32.add
              i32.const 76
              i32.store
              local.get 5
              i32.const 3
              i32.store offset=52
              local.get 5
              i32.const 1061048
              i32.store offset=48
              local.get 5
              i32.const 17
              i32.store offset=76
              local.get 5
              local.get 5
              i32.const 72
              i32.add
              i32.store offset=56
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
              call $_ZN4core9panicking9panic_fmt17h4c9c94dcfde83250E
              unreachable
            end
            local.get 5
            i32.const 100
            i32.add
            i32.const 76
            i32.store
            local.get 5
            i32.const 92
            i32.add
            i32.const 76
            i32.store
            local.get 5
            i32.const 72
            i32.add
            i32.const 12
            i32.add
            i32.const 17
            i32.store
            local.get 5
            i32.const 48
            i32.add
            i32.const 12
            i32.add
            i64.const 4
            i64.store align=4
            local.get 5
            i32.const 4
            i32.store offset=52
            local.get 5
            i32.const 1060888
            i32.store offset=48
            local.get 5
            i32.const 17
            i32.store offset=76
            local.get 5
            local.get 5
            i32.const 72
            i32.add
            i32.store offset=56
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
            call $_ZN4core9panicking9panic_fmt17h4c9c94dcfde83250E
            unreachable
          end
          local.get 3
          local.get 6
          i32.const 1061100
          call $_ZN4core5slice5index22slice_index_order_fail17h889710de9520e473E
          unreachable
        end
        i32.const 1059611
        i32.const 43
        local.get 4
        call $_ZN4core9panicking5panic17h5f3201ae514c7bcbE
        unreachable
      end
      local.get 0
      local.get 1
      local.get 3
      local.get 1
      local.get 4
      call $_ZN4core3str16slice_error_fail17hd99937096c80f263E
      unreachable
    end
    local.get 0
    local.get 1
    i32.const 0
    local.get 6
    local.get 4
    call $_ZN4core3str16slice_error_fail17hd99937096c80f263E
    unreachable
  )
  (func $_ZN4core7unicode9printable5check17h6f3ea5e955a4153fE (;246;) (type 21) (param i32 i32 i32 i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    i32.const 1
    local.set 7
    block ;; label = @1
      block ;; label = @2
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
        loop ;; label = @3
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
          block ;; label = @4
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
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                local.get 10
                local.get 13
                i32.gt_u
                br_if 0 (;@6;)
                local.get 13
                local.get 4
                i32.gt_u
                br_if 1 (;@5;)
                local.get 3
                local.get 10
                i32.add
                local.set 1
                loop ;; label = @7
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
              i32.const 1061172
              call $_ZN4core5slice5index22slice_index_order_fail17h889710de9520e473E
              unreachable
            end
            local.get 13
            local.get 4
            i32.const 1061172
            call $_ZN4core5slice5index24slice_end_index_len_fail17h206e334eab3e7498E
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
      loop ;; label = @2
        local.get 5
        i32.const 1
        i32.add
        local.set 10
        block ;; label = @3
          block ;; label = @4
            local.get 5
            i32.load8_u
            local.tee 2
            i32.extend8_s
            local.tee 13
            i32.const 0
            i32.lt_s
            br_if 0 (;@4;)
            local.get 10
            local.set 5
            br 1 (;@3;)
          end
          block ;; label = @4
            local.get 10
            local.get 11
            i32.eq
            br_if 0 (;@4;)
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
            br 1 (;@3;)
          end
          i32.const 1059611
          i32.const 43
          i32.const 1061156
          call $_ZN4core9panicking5panic17h5f3201ae514c7bcbE
          unreachable
        end
        local.get 1
        local.get 2
        i32.sub
        local.tee 1
        i32.const 0
        i32.lt_s
        br_if 1 (;@1;)
        local.get 7
        i32.const 1
        i32.xor
        local.set 7
        local.get 5
        local.get 11
        i32.ne
        br_if 0 (;@2;)
      end
    end
    local.get 7
    i32.const 1
    i32.and
  )
  (func $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i8$GT$3fmt17he2282454763b40d3E (;247;) (type 2) (param i32 i32) (result i32)
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
      i32.const 1060116
      call $_ZN4core5slice5index26slice_start_index_len_fail17h43cf12d46dc03083E
      unreachable
    end
    local.get 1
    i32.const 1
    i32.const 1060112
    i32.const 2
    local.get 2
    local.get 0
    i32.add
    i32.const 128
    i32.add
    i32.const 0
    local.get 0
    i32.sub
    call $_ZN4core3fmt9Formatter12pad_integral17ha8545ce28396770eE
    local.set 0
    local.get 2
    i32.const 128
    i32.add
    global.set $__stack_pointer
    local.get 0
  )
  (func $_ZN4core3fmt3num3imp7fmt_u6417ha7c93bd38c403be2E (;248;) (type 22) (param i64 i32 i32) (result i32)
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
        i32.const 1060132
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
        i32.const 1060132
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
      i32.const 1060132
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
        i32.const 1060132
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
    i32.const 1059584
    i32.const 0
    local.get 3
    i32.const 9
    i32.add
    local.get 4
    i32.add
    i32.const 39
    local.get 4
    i32.sub
    call $_ZN4core3fmt9Formatter12pad_integral17ha8545ce28396770eE
    local.set 4
    local.get 3
    i32.const 48
    i32.add
    global.set $__stack_pointer
    local.get 4
  )
  (func $_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i16$GT$3fmt17h67676b927244aafbE (;249;) (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 128
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 0
    i32.load16_u
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
      i32.const 65535
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
      i32.const 1060116
      call $_ZN4core5slice5index26slice_start_index_len_fail17h43cf12d46dc03083E
      unreachable
    end
    local.get 1
    i32.const 1
    i32.const 1060112
    i32.const 2
    local.get 2
    local.get 0
    i32.add
    i32.const 128
    i32.add
    i32.const 0
    local.get 0
    i32.sub
    call $_ZN4core3fmt9Formatter12pad_integral17ha8545ce28396770eE
    local.set 0
    local.get 2
    i32.const 128
    i32.add
    global.set $__stack_pointer
    local.get 0
  )
  (func $_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i32$GT$3fmt17h6acce4b91e41041fE (;250;) (type 2) (param i32 i32) (result i32)
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
      i32.const 1060116
      call $_ZN4core5slice5index26slice_start_index_len_fail17h43cf12d46dc03083E
      unreachable
    end
    local.get 1
    i32.const 1
    i32.const 1060112
    i32.const 2
    local.get 2
    local.get 3
    i32.add
    i32.const 128
    i32.add
    i32.const 0
    local.get 3
    i32.sub
    call $_ZN4core3fmt9Formatter12pad_integral17ha8545ce28396770eE
    local.set 0
    local.get 2
    i32.const 128
    i32.add
    global.set $__stack_pointer
    local.get 0
  )
  (func $_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17he6e3ccd6b1fca402E (;251;) (type 2) (param i32 i32) (result i32)
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
    call $_ZN4core3fmt3num3imp7fmt_u6417ha7c93bd38c403be2E
  )
  (func $_init_spec_checker.command_export (;252;) (type 9) (param i64)
    local.get 0
    call $_init_spec_checker
    call $__wasm_call_dtors
  )
  (func $_tick_yew.command_export (;253;) (type 10) (param i64 f32)
    local.get 0
    local.get 1
    call $_tick_yew
    call $__wasm_call_dtors
  )
  (func $_check_spec.command_export (;254;) (type 11) (param i64) (result f32)
    local.get 0
    call $_check_spec
    call $__wasm_call_dtors
  )
  (func $load.command_export (;255;) (type 13)
    call $load
    call $__wasm_call_dtors
  )
  (func $add.command_export (;256;) (type 2) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call $add
    call $__wasm_call_dtors
  )
  (func $print_run_as_root.command_export (;257;) (type 13)
    call $print_run_as_root
    call $__wasm_call_dtors
  )
  (table (;0;) 86 86 funcref)
  (memory (;0;) 17)
  (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
  (export "memory" (memory 0))
  (export "_init_spec_checker" (func $_init_spec_checker.command_export))
  (export "_tick_yew" (func $_tick_yew.command_export))
  (export "_check_spec" (func $_check_spec.command_export))
  (export "load" (func $load.command_export))
  (export "add" (func $add.command_export))
  (export "print_run_as_root" (func $print_run_as_root.command_export))
  (elem (;0;) (i32.const 1) func $_ZN4core3ptr30drop_in_place$LT$$RF$isize$GT$17hbf34798627917637E $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hdc74b241136d2562E $_ZN4core3ptr92drop_in_place$LT$std..io..Write..write_fmt..Adapter$LT$std..sys..wasi..stdio..Stderr$GT$$GT$17h7b29167fe273dc74E $_ZN80_$LT$std..io..Write..write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17h7a631dd30797a730E $_ZN4core3fmt5Write10write_char17hf85733f50d686e5cE $_ZN4core3fmt5Write9write_fmt17h28e824752ff5ab22E $_ZN4core3ptr162drop_in_place$LT$std..sync..poison..PoisonError$LT$std..sync..rwlock..RwLockWriteGuard$LT$core..option..Option$LT$skyapex..spec_check..SpecChecker$GT$$GT$$GT$$GT$17ha38c2f9a20d04a09E $_ZN76_$LT$std..sync..poison..PoisonError$LT$T$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17h1b50841a791d287fE $_ZN4core3ptr161drop_in_place$LT$std..sync..poison..PoisonError$LT$std..sync..rwlock..RwLockReadGuard$LT$core..option..Option$LT$skyapex..spec_check..SpecChecker$GT$$GT$$GT$$GT$17hc6e2ea90168da319E $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hb954ea6cfc180865E $_ZN4core3ptr92drop_in_place$LT$std..io..Write..write_fmt..Adapter$LT$std..sys..wasi..stdio..Stderr$GT$$GT$17h7b29167fe273dc74E.llvm.13741533063225024593 $_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17he6e3ccd6b1fca402E $_ZN60_$LT$alloc..string..String$u20$as$u20$core..fmt..Display$GT$3fmt17h4f88562cb0fdb0b2E $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h4935ef6be9273719E $_ZN60_$LT$std..io..error..Error$u20$as$u20$core..fmt..Display$GT$3fmt17h69a01cca8cf82440E $_ZN91_$LT$std..sys_common..backtrace.._print..DisplayBacktrace$u20$as$u20$core..fmt..Display$GT$3fmt17h18070e4ad91fc378E $_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17h66c9e2e594bd720fE $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h28f18327876a1111E $_ZN73_$LT$core..panic..panic_info..PanicInfo$u20$as$u20$core..fmt..Display$GT$3fmt17hf057b4be12e8885dE $_ZN3std5alloc24default_alloc_error_hook17he5c27c2a00a1f22bE $_ZN4core3ptr122drop_in_place$LT$$RF$alloc..boxed..Box$LT$dyn$u20$core..error..Error$u2b$core..marker..Sync$u2b$core..marker..Send$GT$$GT$17h97a573c8385cd3f3E $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17he2f815353add60f3E $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h97c11851c7cd90d9E $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17he5cf1af9da646324E $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h3f68d0fccb7931f2E $_ZN4core3ptr88drop_in_place$LT$std..io..Write..write_fmt..Adapter$LT$alloc..vec..Vec$LT$u8$GT$$GT$$GT$17h95ab4550ec4744f1E $_ZN80_$LT$std..io..Write..write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17hb70b43f3d4c03d02E $_ZN4core3fmt5Write10write_char17h906853a4beb697c3E $_ZN4core3fmt5Write9write_fmt17h60aacf7e3630642cE $_ZN80_$LT$std..io..Write..write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17h3645be837ec99777E $_ZN4core3fmt5Write10write_char17hf8c41fbbae2a3a44E $_ZN4core3fmt5Write9write_fmt17hfa56d2716ef32465E $_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h0584c21fe4121947E $_ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Write$GT$9write_str17h4bc6421b4252011aE $_ZN58_$LT$alloc..string..String$u20$as$u20$core..fmt..Write$GT$10write_char17hc8f5ed4cd4cff5c5E $_ZN4core3fmt5Write9write_fmt17h770044ce5f576af8E $_ZN80_$LT$std..io..Write..write_fmt..Adapter$LT$T$GT$$u20$as$u20$core..fmt..Write$GT$9write_str17hdb9e855babefd96dE $_ZN4core3fmt5Write10write_char17hc13b02cd8db1d5f4E $_ZN4core3fmt5Write9write_fmt17h914d32a557f83bb7E $_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17h3468cfc708821600E $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$5write17ha287c1051446cd88E $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$14write_vectored17hd8f38df32697019bE $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$17is_write_vectored17h19984a8e54de7033E $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$5flush17hb26d9c6c4dfd9475E $_ZN3std2io5impls74_$LT$impl$u20$std..io..Write$u20$for$u20$alloc..vec..Vec$LT$u8$C$A$GT$$GT$9write_all17hee48d19b9cfa6b30E $_ZN3std2io5Write18write_all_vectored17h19c40315f984a3a6E $_ZN3std2io5Write9write_fmt17h6324f22ae5513448E $_ZN4core3ptr29drop_in_place$LT$$LP$$RP$$GT$17h2cc8ce0a008ae539E $_ZN64_$LT$std..sys..wasi..stdio..Stderr$u20$as$u20$std..io..Write$GT$5write17h0c145d49840ec5beE $_ZN64_$LT$std..sys..wasi..stdio..Stderr$u20$as$u20$std..io..Write$GT$14write_vectored17h3ffa40051f9caeddE $_ZN64_$LT$std..sys..wasi..stdio..Stderr$u20$as$u20$std..io..Write$GT$17is_write_vectored17h922cd97db2995730E $_ZN64_$LT$std..sys..wasi..stdio..Stderr$u20$as$u20$std..io..Write$GT$5flush17h00c625cb4891deebE $_ZN3std2io5Write9write_all17hafc4d82fa322d20fE $_ZN3std2io5Write18write_all_vectored17h6c11d8c926e24abaE $_ZN3std2io5Write9write_fmt17h93c1ecc6a742a6fcE $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17he5809b74c70e51b3E $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17hca611893b35124d4E $_ZN99_$LT$std..panicking..begin_panic_handler..StaticStrPayload$u20$as$u20$core..panic..PanicPayload$GT$8take_box17h630c1ce94eac9b08E $_ZN99_$LT$std..panicking..begin_panic_handler..StaticStrPayload$u20$as$u20$core..panic..PanicPayload$GT$3get17h9d53a07a6ca87788E $_ZN4core3ptr77drop_in_place$LT$std..panicking..begin_panic_handler..FormatStringPayload$GT$17h1c91699c70947622E $_ZN102_$LT$std..panicking..begin_panic_handler..FormatStringPayload$u20$as$u20$core..panic..PanicPayload$GT$8take_box17h82b487330533288eE $_ZN102_$LT$std..panicking..begin_panic_handler..FormatStringPayload$u20$as$u20$core..panic..PanicPayload$GT$3get17h6d9db02763a39a57E $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h6c8635a3a4fc6b26E $_ZN64_$LT$core..str..error..Utf8Error$u20$as$u20$core..fmt..Debug$GT$3fmt17hc7b0f2dc31690f4dE $_ZN4core3ptr47drop_in_place$LT$wasi..lib_generated..Errno$GT$17hbb8995281c09dab1E $_ZN63_$LT$wasi..lib_generated..Errno$u20$as$u20$core..fmt..Debug$GT$3fmt17h5d615f2dd44a9ecaE $_ZN4core3ptr24drop_in_place$LT$u32$GT$17h9daefed861bb557fE $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h0817299a726df9f5E $_ZN4core3ptr24drop_in_place$LT$u16$GT$17h2be5d1308325373fE $_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u16$GT$3fmt17hcd7c0c3518380c0dE $_ZN4core3ptr37drop_in_place$LT$core..fmt..Error$GT$17h5d671d2c1d102d73E $_ZN69_$LT$core..alloc..layout..LayoutError$u20$as$u20$core..fmt..Debug$GT$3fmt17h992770ef3553fb60E $_ZN4core3ops8function6FnOnce9call_once17hd548f09e85633547E $_ZN63_$LT$core..cell..BorrowMutError$u20$as$u20$core..fmt..Debug$GT$3fmt17h34898b32f446e8ffE $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h08b181ec1a3a2818E $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hbcfedd02189591b0E $_ZN59_$LT$core..fmt..Arguments$u20$as$u20$core..fmt..Display$GT$3fmt17hf432543ead310655E $_ZN71_$LT$core..ops..range..Range$LT$Idx$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17h650ab90e3c444b89E $_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17hd09ca3de828ddddaE $_ZN4core3ptr37drop_in_place$LT$core..fmt..Error$GT$17h141d50a833f61f7aE $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h1b016e6ca1df6044E $_ZN4core3ptr25drop_in_place$LT$char$GT$17h6ee13944e4abd559E $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h1bc5e3fd4b32d905E $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$10write_char17ha3700472f18f7184E $_ZN4core3fmt5Write9write_fmt17h33566f065e9aa63aE)
  (data $.rodata (;0;) (i32.const 1048576) "/home/deck/.cargo/registry/src/github.com-1ecc6299db9ec823/indexmap-2.1.0/src/map/core.rs\00\00\00\00\00\10\00Y\00\00\00!\00\00\00\0f\00\00\00Once instance has previously been poisoned\00\00l\00\10\00*\00\00\00one-time initialization may not be performed recursively\a0\00\10\008\00\00\00called `Option::unwrap()` on a `None` value/rustc/bf8716f1cd6416266807706bcae0ecb2e51c9d4a/library/std/src/sync/once.rs\00\0b\01\10\00L\00\00\00\95\00\00\002\00\00\00\01\00\00\00\04\00\00\00\04\00\00\00\02\00\00\00\03\00\00\00\0c\00\00\00\04\00\00\00\04\00\00\00\05\00\00\00\06\00\00\00PoisonError/home/deck/.cargo/registry/src/github.com-1ecc6299db9ec823/indexmap-2.1.0/src/map/core/raw.rs\9b\01\10\00]\00\00\00\a4\00\00\00\1e\00\00\00fatal runtime error: rwlock locked for writing\0a\00\08\02\10\00/\00\00\00fatal runtime error: rwlock locked for reading\0a\00@\02\10\00/\00\00\00\ff\ff\ff\ff\ff\ff\ff\ffx\02\10\00\00\00\00\00\00\00\00\00\00\00\00\00called `Result::unwrap()` on an `Err` value\00\07\00\00\00\08\00\00\00\04\00\00\00\08\00\00\00src/spec_check.rs\00\00\00\cc\02\10\00\11\00\00\00a\00\00\00\1b\00\00\00called `Option::unwrap()` on a `None` value\00\cc\02\10\00\11\00\00\00n\00\00\00\0a\00\00\00\cc\02\10\00\11\00\00\00l\00\00\00\0a\00\00\00\09\00\00\00\08\00\00\00\04\00\00\00\08\00\00\00\cc\02\10\00\11\00\00\00g\00\00\00\1c\00\00\00\cc\02\10\00\11\00\00\00w\00\00\00\0a\00\00\00/home/deck/.cargo/registry/src/github.com-1ecc6299db9ec823/lazy_static-1.4.0/src/inline_lazy.rs\00l\03\10\00_\00\00\00\1e\00\00\00\10\00\00\00/home/deck/.cargo/registry/src/github.com-1ecc6299db9ec823/indexmap-2.1.0/src/map/core.rs\00\00\00\dc\03\10\00Y\00\00\00)\00\00\00#\00\00\00\dc\03\10\00Y\00\00\00\e9\02\00\00\19\00\00\00\0a\00\00\00\dc\03\10\00\00\00\00\00X\04\10\00\01\00\00\00\01\c3\0e\12Bk\90\a4\bf\9a\9eX\0a\0c\adl\1e\e1~failed to write whole buffer\00\7f\04\10\00\1c\00\00\00\17\00\00\00/rustc/bf8716f1cd6416266807706bcae0ecb2e51c9d4a/library/std/src/io/mod.rs\00\00\00\a8\04\10\00I\00\00\00\8d\06\00\00$\00\00\00\0b\00\00\00\0c\00\00\00\04\00\00\00\04\00\00\00\05\00\00\00\06\00\00\00formatter error\00\1c\05\10\00\0f\00\00\00(\00\00\00\ff\ff\ff\ff/rustc/bf8716f1cd6416266807706bcae0ecb2e51c9d4a/library/std/src/sys/wasi/../unsupported/locks/rwlock.rs\00<\05\10\00g\00\00\00?\00\00\00\09\00\00\00Hash table capacity overflow\b4\05\10\00\1c\00\00\00/home/deck/.cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.14.3/src/raw/mod.rs\00\00\d8\05\10\00Z\00\00\00V\00\00\00(\00\00\00\15\00\00\00\04\00\00\00\04\00\00\00\16\00\00\00reentrant init\00\00T\06\10\00\0e\00\00\00/rustc/bf8716f1cd6416266807706bcae0ecb2e51c9d4a/library/core/src/cell/once.rs\00\00\00l\06\10\00M\00\00\00\d9\00\00\00B\00\00\00\00\00\00\00\15\00\00\00\04\00\00\00\04\00\00\00\17\00\00\00called `Option::unwrap()` on a `None` valueinternal error: entered unreachable code/rustc/bf8716f1cd6416266807706bcae0ecb2e51c9d4a/library/alloc/src/vec/mod.rs\003\07\10\00L\00\00\00$\08\00\00$\00\00\00\15\00\00\00\04\00\00\00\04\00\00\00\18\00\00\00Utf8Errorvalid_up_toerror_len\00\00\00\15\00\00\00\04\00\00\00\04\00\00\00\19\00\00\00NoneSome\1a\00\00\00\0c\00\00\00\04\00\00\00\1b\00\00\00\1c\00\00\00\1d\00\00\00\1a\00\00\00\0c\00\00\00\04\00\00\00\1e\00\00\00\1f\00\00\00 \00\00\00!\00\00\00\0c\00\00\00\04\00\00\00\22\00\00\00#\00\00\00$\00\00\00\1a\00\00\00\0c\00\00\00\04\00\00\00%\00\00\00&\00\00\00'\00\00\00\0alibrary/std/src/thread/mod.rsfailed to generate unique thread ID: bitspace exhausted\00\00\00V\08\10\007\00\00\009\08\10\00\1d\00\00\00\98\04\00\00\0d\00\00\00RUST_BACKTRACEcalled `Result::unwrap()` on an `Err` valueassertion failed: mid <= self.len()failed to write the buffered data\00\00\00\04\09\10\00!\00\00\00\17\00\00\00T\06\10\00\00\00\00\00library/std/src/io/buffered/linewritershim.rs\00\00\00<\09\10\00-\00\00\00\01\01\00\00)\00\00\00entity not foundpermission deniedconnection refusedconnection resethost unreachablenetwork unreachableconnection abortednot connectedaddress in useaddress not availablenetwork downbroken pipeentity already existsoperation would blocknot a directoryis a directorydirectory not emptyread-only filesystem or storage mediumfilesystem loop or indirection limit (e.g. symlink loop)stale network file handleinvalid input parameterinvalid datatimed outwrite zerono storage spaceseek on unseekable filefilesystem quota exceededfile too largeresource busyexecutable file busydeadlockcross-device link or renametoo many linksinvalid filenameargument list too longoperation interruptedunsupportedunexpected end of fileout of memoryother erroruncategorized error (os error )\00\00\00T\06\10\00\00\00\00\00i\0c\10\00\0b\00\00\00t\0c\10\00\01\00\00\00library/std/src/io/stdio.rs\00\90\0c\10\00\1b\00\00\00\eb\02\00\00\14\00\00\00failed printing to : \00\00\00\bc\0c\10\00\13\00\00\00\cf\0c\10\00\02\00\00\00\90\0c\10\00\1b\00\00\00\fd\03\00\00\09\00\00\00stdoutlibrary/std/src/io/mod.rs\00\fa\0c\10\00\19\00\00\00\a3\05\00\00 \00\00\00advancing io slices beyond their length\00$\0d\10\00'\00\00\00\fa\0c\10\00\19\00\00\00\a5\05\00\00\0d\00\00\00advancing IoSlice beyond its length\00d\0d\10\00#\00\00\00library/std/src/sys/wasi/io.rs\00\00\90\0d\10\00\1e\00\00\00\17\00\00\00\0d\00\00\00failed to write whole buffer\c0\0d\10\00\1c\00\00\00\17\00\00\00\fa\0c\10\00\19\00\00\00\8d\06\00\00$\00\00\00formatter error\00\f8\0d\10\00\0f\00\00\00(\00\00\00library/std/src/panic.rs\14\0e\10\00\18\00\00\00\f5\00\00\00\12\00\00\00fullcannot recursively acquire mutex@\0e\10\00 \00\00\00library/std/src/sys/wasi/../unsupported/locks/mutex.rs\00\00h\0e\10\006\00\00\00\14\00\00\00\09\00\00\00library/std/src/sync/once.rs\b0\0e\10\00\1c\00\00\00\d0\00\00\00\14\00\00\00\b0\0e\10\00\1c\00\00\00\d0\00\00\001\00\00\00lock count overflow in reentrant mutexlibrary/std/src/sync/remutex.rs\00\00\00\12\0f\10\00\1f\00\00\00\91\00\00\00\0e\00\00\00file name contained an unexpected NUL byte\00\00D\0f\10\00*\00\00\00\14\00\00\00\00\00\00\00\02\00\00\00p\0f\10\00stack backtrace:\0a\00\00\00\88\0f\10\00\11\00\00\00note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.\0a\a4\0f\10\00X\00\00\00memory allocation of  bytes failed\0a\00\04\10\10\00\15\00\00\00\19\10\10\00\0e\00\00\00 bytes failed\00\00\00\04\10\10\00\15\00\00\008\10\10\00\0d\00\00\00library/std/src/alloc.rsX\10\10\00\18\00\00\00b\01\00\00\09\00\00\00library/std/src/panicking.rsBox<dyn Any><unnamed>\00\00\00(\00\00\00\0c\00\00\00\04\00\00\00)\00\00\00*\00\00\00+\00\00\00,\00\00\00-\00\00\00.\00\00\00/\00\00\000\00\00\00\00\00\00\00\01\00\00\001\00\00\002\00\00\003\00\00\004\00\00\005\00\00\006\00\00\007\00\00\00thread '' panicked at :\0a\04\11\10\00\08\00\00\00\0c\11\10\00\0e\00\00\00\1a\11\10\00\02\00\00\008\08\10\00\01\00\00\00note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace\0a\00\00<\11\10\00N\00\00\00\80\10\10\00\1c\00\00\00\84\02\00\00\1e\00\00\00!\00\00\00\0c\00\00\00\04\00\00\008\00\00\00\15\00\00\00\08\00\00\00\04\00\00\009\00\00\00\15\00\00\00\08\00\00\00\04\00\00\00:\00\00\00;\00\00\00<\00\00\00\10\00\00\00\04\00\00\00=\00\00\00>\00\00\000\00\00\00\00\00\00\00\01\00\00\00?\00\00\00\0apanicked after panic::always_abort(), aborting.\0a\00\00\00T\06\10\00\00\00\00\00\fc\11\10\001\00\00\00thread panicked while processing panic. aborting.\0a\00\00@\12\10\002\00\00\00thread caused non-unwinding panic. aborting.\0a\00\00\00|\12\10\00-\00\00\00fatal runtime error: failed to initiate panic, error \00\00\00\b4\12\10\005\00\00\008\08\10\00\01\00\00\00\15\00\00\00\08\00\00\00\04\00\00\00@\00\00\00library/std/src/sys/wasi/os.rs\00\00\0c\13\10\00\1e\00\00\00C\00\00\006\00\00\00strerror_r failure\00\00<\13\10\00\12\00\00\00\0c\13\10\00\1e\00\00\00A\00\00\00\0d\00\00\00A\00\00\00\02\00\00\00\02\00\00\00B\00\00\00random_get failurelibrary/std/src/sys/wasi/mod.rs\00\00\00\8a\13\10\00\1f\00\00\00\be\00\00\00%\00\00\00fatal runtime error: rwlock locked for writing\0a\00\bc\13\10\00/\00\00\00one-time initialization may not be performed recursively\f4\13\10\008\00\00\00\10\00\00\00\11\00\00\00\12\00\00\00\10\00\00\00\10\00\00\00\13\00\00\00\12\00\00\00\0d\00\00\00\0e\00\00\00\15\00\00\00\0c\00\00\00\0b\00\00\00\15\00\00\00\15\00\00\00\0f\00\00\00\0e\00\00\00\13\00\00\00&\00\00\008\00\00\00\19\00\00\00\17\00\00\00\0c\00\00\00\09\00\00\00\0a\00\00\00\10\00\00\00\17\00\00\00\19\00\00\00\0e\00\00\00\0d\00\00\00\14\00\00\00\08\00\00\00\1b\00\00\00\0e\00\00\00\10\00\00\00\16\00\00\00\15\00\00\00\0b\00\00\00\16\00\00\00\0d\00\00\00\0b\00\00\00\13\00\00\00|\09\10\00\8c\09\10\00\9d\09\10\00\af\09\10\00\bf\09\10\00\cf\09\10\00\e2\09\10\00\f4\09\10\00\01\0a\10\00\0f\0a\10\00$\0a\10\000\0a\10\00;\0a\10\00P\0a\10\00e\0a\10\00t\0a\10\00\82\0a\10\00\95\0a\10\00\bb\0a\10\00\f3\0a\10\00\0c\0b\10\00#\0b\10\00/\0b\10\008\0b\10\00B\0b\10\00R\0b\10\00i\0b\10\00\82\0b\10\00\90\0b\10\00\9d\0b\10\00\b1\0b\10\00\b9\0b\10\00\d4\0b\10\00\e2\0b\10\00\f2\0b\10\00\08\0c\10\00\1d\0c\10\00(\0c\10\00>\0c\10\00K\0c\10\00V\0c\10\00codenameC\00\00\00\08\00\00\00\04\00\00\00D\00\00\00messageSUCCESS2BIGACCESADDRINUSEADDRNOTAVAILAFNOSUPPORTAGAINALREADYBADFBADMSGBUSYCANCELEDCHILDCONNABORTEDCONNREFUSEDCONNRESETDEADLKDESTADDRREQDOMDQUOTEXISTFAULTFBIGHOSTUNREACHIDRMILSEQINPROGRESSINTRINVALIOISCONNISDIRLOOPMFILEMLINKMSGSIZEMULTIHOPNAMETOOLONGNETDOWNNETRESETNETUNREACHNFILENOBUFSNODEVNOENTNOEXECNOLCKNOLINKNOMEMNOMSGNOPROTOOPTNOSPCNOSYSNOTCONNNOTDIRNOTEMPTYNOTRECOVERABLENOTSOCKNOTSUPNOTTYNXIOOVERFLOWOWNERDEADPERMPIPEPROTOPROTONOSUPPORTPROTOTYPERANGEROFSSPIPESRCHSTALETIMEDOUTTXTBSYXDEVNOTCAPABLENo error occurred. System call completed successfully.Argument list too long.Permission denied.Address in use.Address not available.Address family not supported.Resource unavailable, or operation would block.Connection already in progress.Bad file descriptor.Bad message.Device or resource busy.Operation canceled.No child processes.Connection aborted.Connection refused.Connection reset.Resource deadlock would occur.Destination address required.Mathematics argument out of domain of function.Reserved.File exists.Bad address.File too large.Host is unreachable.Identifier removed.Illegal byte sequence.Operation in progress.Interrupted function.Invalid argument.I/O error.Socket is connected.Is a directory.Too many levels of symbolic links.File descriptor value too large.Too many links.Message too large.Filename too long.Network is down.Connection aborted by network.Network unreachable.Too many files open in system.No buffer space available.No such device.No such file or directory.Executable file format error.No locks available.Not enough space.No message of the desired type.Protocol not available.No space left on device.Function not supported.The socket is not connected.Not a directory or a symbolic link to a directory.Directory not empty.State not recoverable.Not a socket.Not supported, or operation not supported on socket.Inappropriate I/O control operation.No such device or address.Value too large to be stored in data type.Previous owner died.Operation not permitted.Broken pipe.Protocol error.Protocol not supported.Protocol wrong type for socket.Result too large.Read-only file system.Invalid seek.No such process.Connection timed out.Text file busy.Cross-device link.Extension: Capabilities insufficient.Errno\00\00E\00\00\00\02\00\00\00\02\00\00\00F\00\00\00\07\00\00\00\04\00\00\00\05\00\00\00\09\00\00\00\0c\00\00\00\0b\00\00\00\05\00\00\00\07\00\00\00\04\00\00\00\06\00\00\00\04\00\00\00\08\00\00\00\05\00\00\00\0b\00\00\00\0b\00\00\00\09\00\00\00\06\00\00\00\0b\00\00\00\03\00\00\00\05\00\00\00\05\00\00\00\05\00\00\00\04\00\00\00\0b\00\00\00\04\00\00\00\05\00\00\00\0a\00\00\00\04\00\00\00\05\00\00\00\02\00\00\00\06\00\00\00\05\00\00\00\04\00\00\00\05\00\00\00\05\00\00\00\07\00\00\00\08\00\00\00\0b\00\00\00\07\00\00\00\08\00\00\00\0a\00\00\00\05\00\00\00\06\00\00\00\05\00\00\00\05\00\00\00\06\00\00\00\05\00\00\00\06\00\00\00\05\00\00\00\05\00\00\00\0a\00\00\00\05\00\00\00\05\00\00\00\07\00\00\00\06\00\00\00\08\00\00\00\0e\00\00\00\07\00\00\00\06\00\00\00\05\00\00\00\04\00\00\00\08\00\00\00\09\00\00\00\04\00\00\00\04\00\00\00\05\00\00\00\0e\00\00\00\09\00\00\00\05\00\00\00\04\00\00\00\05\00\00\00\04\00\00\00\05\00\00\00\08\00\00\00\06\00\00\00\04\00\00\00\0a\00\00\00\9b\15\10\00\a2\15\10\00\a6\15\10\00\ab\15\10\00\b4\15\10\00\c0\15\10\00\cb\15\10\00\d0\15\10\00\d7\15\10\00\db\15\10\00\e1\15\10\00\e5\15\10\00\ed\15\10\00\f2\15\10\00\fd\15\10\00\08\16\10\00\11\16\10\00\17\16\10\00\22\16\10\00%\16\10\00*\16\10\00/\16\10\004\16\10\008\16\10\00C\16\10\00G\16\10\00L\16\10\00V\16\10\00Z\16\10\00_\16\10\00a\16\10\00g\16\10\00l\16\10\00p\16\10\00u\16\10\00z\16\10\00\81\16\10\00\89\16\10\00\94\16\10\00\9b\16\10\00\a3\16\10\00\ad\16\10\00\b2\16\10\00\b8\16\10\00\bd\16\10\00\c2\16\10\00\c8\16\10\00\cd\16\10\00\d3\16\10\00\d8\16\10\00\dd\16\10\00\e7\16\10\00\ec\16\10\00\f1\16\10\00\f8\16\10\00\fe\16\10\00\06\17\10\00\14\17\10\00\1b\17\10\00!\17\10\00&\17\10\00*\17\10\002\17\10\00;\17\10\00?\17\10\00C\17\10\00H\17\10\00V\17\10\00_\17\10\00d\17\10\00h\17\10\00m\17\10\00q\17\10\00v\17\10\00~\17\10\00\84\17\10\00\88\17\10\006\00\00\00\17\00\00\00\12\00\00\00\0f\00\00\00\16\00\00\00\1d\00\00\00/\00\00\00\1f\00\00\00\14\00\00\00\0c\00\00\00\18\00\00\00\13\00\00\00\13\00\00\00\13\00\00\00\13\00\00\00\11\00\00\00\1e\00\00\00\1d\00\00\00/\00\00\00\09\00\00\00\0c\00\00\00\0c\00\00\00\0f\00\00\00\14\00\00\00\13\00\00\00\16\00\00\00\16\00\00\00\15\00\00\00\11\00\00\00\0a\00\00\00\14\00\00\00\0f\00\00\00\22\00\00\00 \00\00\00\0f\00\00\00\12\00\00\00\09\00\00\00\12\00\00\00\10\00\00\00\1e\00\00\00\14\00\00\00\1e\00\00\00\1a\00\00\00\0f\00\00\00\1a\00\00\00\1d\00\00\00\13\00\00\00\09\00\00\00\11\00\00\00\1f\00\00\00\17\00\00\00\18\00\00\00\17\00\00\00\1c\00\00\002\00\00\00\14\00\00\00\16\00\00\00\0d\00\00\004\00\00\00$\00\00\00\1a\00\00\00*\00\00\00\14\00\00\00\18\00\00\00\0c\00\00\00\0f\00\00\00\17\00\00\00\1f\00\00\00\11\00\00\00\16\00\00\00\0d\00\00\00\10\00\00\00\09\00\00\00\15\00\00\00\0f\00\00\00\12\00\00\00%\00\00\00\92\17\10\00\c8\17\10\00\df\17\10\00\f1\17\10\00\00\18\10\00\16\18\10\003\18\10\00b\18\10\00\81\18\10\00\95\18\10\00\a1\18\10\00\b9\18\10\00\cc\18\10\00\df\18\10\00\f2\18\10\00\05\19\10\00\16\19\10\004\19\10\00Q\19\10\00\80\19\10\00\89\19\10\00\95\19\10\00\a1\19\10\00\b0\19\10\00\c4\19\10\00\d7\19\10\00\ed\19\10\00\03\1a\10\00\18\1a\10\00)\1a\10\003\1a\10\00G\1a\10\00V\1a\10\00x\1a\10\00\98\1a\10\00\a7\1a\10\00\80\19\10\00\b9\1a\10\00\cb\1a\10\00\db\1a\10\00\f9\1a\10\00\0d\1b\10\00+\1b\10\00E\1b\10\00T\1b\10\00n\1b\10\00\8b\1b\10\00\80\19\10\00\9e\1b\10\00\af\1b\10\00\ce\1b\10\00\e5\1b\10\00\fd\1b\10\00\14\1c\10\000\1c\10\00b\1c\10\00v\1c\10\00\8c\1c\10\00\99\1c\10\00\cd\1c\10\00\f1\1c\10\00\0b\1d\10\005\1d\10\00I\1d\10\00a\1d\10\00m\1d\10\00|\1d\10\00\93\1d\10\00\b2\1d\10\00\c3\1d\10\00\d9\1d\10\00\e6\1d\10\00\80\19\10\00\f6\1d\10\00\0b\1e\10\00\1a\1e\10\00,\1e\10\00/\00Success\00Illegal byte sequence\00Domain error\00Result not representable\00Not a tty\00Permission denied\00Operation not permitted\00No such file or directory\00No such process\00File exists\00Value too large for data type\00No space left on device\00Out of memory\00Resource busy\00Interrupted system call\00Resource temporarily unavailable\00Invalid seek\00Cross-device link\00Read-only file system\00Directory not empty\00Connection reset by peer\00Operation timed out\00Connection refused\00Host is unreachable\00Address in use\00Broken pipe\00I/O error\00No such device or address\00No such device\00Not a directory\00Is a directory\00Text file busy\00Exec format error\00Invalid argument\00Argument list too long\00Symbolic link loop\00Filename too long\00Too many open files in system\00No file descriptors available\00Bad file descriptor\00No child process\00Bad address\00File too large\00Too many links\00No locks available\00Resource deadlock would occur\00State not recoverable\00Previous owner died\00Operation canceled\00Function not implemented\00No message of desired type\00Identifier removed\00Link has been severed\00Protocol error\00Bad message\00Not a socket\00Destination address required\00Message too large\00Protocol wrong type for socket\00Protocol not available\00Protocol not supported\00Not supported\00Address family not supported by protocol\00Address not available\00Network is down\00Network unreachable\00Connection reset by network\00Connection aborted\00No buffer space available\00Socket is connected\00Socket not connected\00Operation already in progress\00Operation in progress\00Stale file handle\00Quota exceeded\00Multihop attempted\00Capabilities insufficient\00\00\00\00\00\00\00\00\00u\02N\00\d6\01\e2\04\b9\04\18\01\8e\05\ed\02\16\04\f2\00\97\03\01\038\05\af\01\82\01O\03/\04\1e\00\d4\05\a2\00\12\03\1e\03\c2\01\de\03\08\00\ac\05\00\01d\02\f1\01e\054\02\8c\02\cf\02-\03L\04\e3\05\9f\02\f8\04\1c\05\08\05\b1\02K\05\15\02x\00R\02<\03\f1\03\e4\00\c3\03}\04\cc\00\aa\03y\05$\02n\01m\03\22\04\ab\04D\00\fb\01\ae\00\83\03`\00\e5\01\07\04\94\04^\04+\00X\019\01\92\00\c2\05\9b\01C\02F\01\f6\05\00\00LayoutErrorlibrary/alloc/src/raw_vec.rscapacity overflow\13*\10\00\11\00\00\00\f7)\10\00\1c\00\00\00;\02\00\00\05\00\00\00called `Option::unwrap()` on a `None` valuelibrary/alloc/src/ffi/c_str.rs\00\00\00g*\10\00\1e\00\00\00\1b\01\00\007\00\00\00called `Result::unwrap()` on an `Err` value\00G\00\00\00\00\00\00\00\01\00\00\00H\00\00\00library/alloc/src/sync.rs\00\00\00\d4*\10\00\19\00\00\00o\01\00\002\00\00\00library/core/src/fmt/mod.rscalled `Option::unwrap()` on a `None` value)..\00\00\00G+\10\00\02\00\00\000123456789abcdefBorrowMutErroralready borrowed: r+\10\00\12\00\00\00\00+\10\00\00\00\00\00:\00\00\00\00+\10\00\00\00\00\00\94+\10\00\01\00\00\00\94+\10\00\01\00\00\00panicked at :\0a\00\00P\00\00\00\00\00\00\00\01\00\00\00Q\00\00\00index out of bounds: the len is  but the index is \00\00\d0+\10\00 \00\00\00\f0+\10\00\12\00\00\00==!=matchesassertion `left  right` failed\0a  left: \0a right: \00\1f,\10\00\10\00\00\00/,\10\00\17\00\00\00F,\10\00\09\00\00\00 right` failed: \0a  left: \00\00\00\1f,\10\00\10\00\00\00h,\10\00\10\00\00\00x,\10\00\09\00\00\00F,\10\00\09\00\00\00: \00\00\00+\10\00\00\00\00\00\a4,\10\00\02\00\00\00R\00\00\00\0c\00\00\00\04\00\00\00S\00\00\00T\00\00\00U\00\00\00     { ,  {\0a,\0a { .. }, .. }..\0a} }((\0a,library/core/src/fmt/num.rs0x\00\00\f5,\10\00\1b\00\00\00i\00\00\00\17\00\00\0000010203040506070809101112131415161718192021222324252627282930313233343536373839404142434445464748495051525354555657585960616263646566676869707172737475767778798081828384858687888990919293949596979899falsetrue\00\00\00\00+\10\00\1b\00\00\005\09\00\00\1a\00\00\00\00+\10\00\1b\00\00\00.\09\00\00\22\00\00\00library/core/src/slice/memchr.rs\18.\10\00 \00\00\00\9f\00\00\00\09\00\00\00\18.\10\00 \00\00\00\83\00\00\00\1e\00\00\00range start index  out of range for slice of length X.\10\00\12\00\00\00j.\10\00\22\00\00\00range end index \9c.\10\00\10\00\00\00j.\10\00\22\00\00\00slice index starts at  but ends at \00\bc.\10\00\16\00\00\00\d2.\10\00\0d\00\00\00\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\03\03\03\03\03\03\03\03\03\03\03\03\03\03\03\03\04\04\04\04\04\00\00\00\00\00\00\00\00\00\00\00[...]begin <= end ( <= ) when slicing ``\f5/\10\00\0e\00\00\00\030\10\00\04\00\00\00\070\10\00\10\00\00\00\170\10\00\01\00\00\00byte index  is not a char boundary; it is inside  (bytes ) of `\0080\10\00\0b\00\00\00C0\10\00&\00\00\00i0\10\00\08\00\00\00q0\10\00\06\00\00\00\170\10\00\01\00\00\00 is out of bounds of `\00\0080\10\00\0b\00\00\00\a00\10\00\16\00\00\00\170\10\00\01\00\00\00library/core/src/str/mod.rs\00\d00\10\00\1b\00\00\00\09\01\00\00,\00\00\00library/core/src/unicode/printable.rs\00\00\00\fc0\10\00%\00\00\00\1a\00\00\006\00\00\00\fc0\10\00%\00\00\00\0a\00\00\00+\00\00\00\00\06\01\01\03\01\04\02\05\07\07\02\08\08\09\02\0a\05\0b\02\0e\04\10\01\11\02\12\05\13\11\14\01\15\02\17\02\19\0d\1c\05\1d\08\1f\01$\01j\04k\02\af\03\b1\02\bc\02\cf\02\d1\02\d4\0c\d5\09\d6\02\d7\02\da\01\e0\05\e1\02\e7\04\e8\02\ee \f0\04\f8\02\fa\03\fb\01\0c';>NO\8f\9e\9e\9f{\8b\93\96\a2\b2\ba\86\b1\06\07\096=>V\f3\d0\d1\04\14\1867VW\7f\aa\ae\af\bd5\e0\12\87\89\8e\9e\04\0d\0e\11\12)14:EFIJNOde\5c\b6\b7\1b\1c\07\08\0a\0b\14\1769:\a8\a9\d8\d9\097\90\91\a8\07\0a;>fi\8f\92\11o_\bf\ee\efZb\f4\fc\ffST\9a\9b./'(U\9d\a0\a1\a3\a4\a7\a8\ad\ba\bc\c4\06\0b\0c\15\1d:?EQ\a6\a7\cc\cd\a0\07\19\1a\22%>?\e7\ec\ef\ff\c5\c6\04 #%&(38:HJLPSUVXZ\5c^`cefksx}\7f\8a\a4\aa\af\b0\c0\d0\ae\afno\be\93^\22{\05\03\04-\03f\03\01/.\80\82\1d\031\0f\1c\04$\09\1e\05+\05D\04\0e*\80\aa\06$\04$\04(\084\0bNC\817\09\16\0a\08\18;E9\03c\08\090\16\05!\03\1b\05\01@8\04K\05/\04\0a\07\09\07@ '\04\0c\096\03:\05\1a\07\04\0c\07PI73\0d3\07.\08\0a\81&RK+\08*\16\1a&\1c\14\17\09N\04$\09D\0d\19\07\0a\06H\08'\09u\0bB>*\06;\05\0a\06Q\06\01\05\10\03\05\80\8bb\1eH\08\0a\80\a6^\22E\0b\0a\06\0d\13:\06\0a6,\04\17\80\b9<dS\0cH\09\0aFE\1bH\08S\0dI\07\0a\80\f6F\0a\1d\03GI7\03\0e\08\0a\069\07\0a\816\19\07;\03\1cV\01\0f2\0d\83\9bfu\0b\80\c4\8aLc\0d\840\10\16\8f\aa\82G\a1\b9\829\07*\04\5c\06&\0aF\0a(\05\13\82\b0[eK\049\07\11@\05\0b\02\0e\97\f8\08\84\d6*\09\a2\e7\813\0f\01\1d\06\0e\04\08\81\8c\89\04k\05\0d\03\09\07\10\92`G\09t<\80\f6\0as\08p\15Fz\14\0c\14\0cW\09\19\80\87\81G\03\85B\0f\15\84P\1f\06\06\80\d5+\05>!\01p-\03\1a\04\02\81@\1f\11:\05\01\81\d0*\82\e6\80\f7)L\04\0a\04\02\83\11DL=\80\c2<\06\01\04U\05\1b4\02\81\0e,\04d\0cV\0a\80\ae8\1d\0d,\04\09\07\02\0e\06\80\9a\83\d8\04\11\03\0d\03w\04_\06\0c\04\01\0f\0c\048\08\0a\06(\08\22N\81T\0c\1d\03\09\076\08\0e\04\09\07\09\07\80\cb%\0a\84\06\00\01\03\05\05\06\06\02\07\06\08\07\09\11\0a\1c\0b\19\0c\1a\0d\10\0e\0c\0f\04\10\03\12\12\13\09\16\01\17\04\18\01\19\03\1a\07\1b\01\1c\02\1f\16 \03+\03-\0b.\010\031\022\01\a7\02\a9\02\aa\04\ab\08\fa\02\fb\05\fd\02\fe\03\ff\09\adxy\8b\8d\a20WX\8b\8c\90\1c\dd\0e\0fKL\fb\fc./?\5c]_\e2\84\8d\8e\91\92\a9\b1\ba\bb\c5\c6\c9\ca\de\e4\e5\ff\00\04\11\12)147:;=IJ]\84\8e\92\a9\b1\b4\ba\bb\c6\ca\ce\cf\e4\e5\00\04\0d\0e\11\12)14:;EFIJ^de\84\91\9b\9d\c9\ce\cf\0d\11):;EIW[\5c^_de\8d\91\a9\b4\ba\bb\c5\c9\df\e4\e5\f0\0d\11EIde\80\84\b2\bc\be\bf\d5\d7\f0\f1\83\85\8b\a4\a6\be\bf\c5\c7\cf\da\dbH\98\bd\cd\c6\ce\cfINOWY^_\89\8e\8f\b1\b6\b7\bf\c1\c6\c7\d7\11\16\17[\5c\f6\f7\fe\ff\80mq\de\df\0e\1fno\1c\1d_}~\ae\af\7f\bb\bc\16\17\1e\1fFGNOXZ\5c^~\7f\b5\c5\d4\d5\dc\f0\f1\f5rs\8ftu\96&./\a7\af\b7\bf\c7\cf\d7\df\9a@\97\980\8f\1f\d2\d4\ce\ffNOZ[\07\08\0f\10'/\ee\efno7=?BE\90\91Sgu\c8\c9\d0\d1\d8\d9\e7\fe\ff\00 _\22\82\df\04\82D\08\1b\04\06\11\81\ac\0e\80\ab\05\1f\09\81\1b\03\19\08\01\04/\044\04\07\03\01\07\06\07\11\0aP\0f\12\07U\07\03\04\1c\0a\09\03\08\03\07\03\02\03\03\03\0c\04\05\03\0b\06\01\0e\15\05N\07\1b\07W\07\02\06\17\0cP\04C\03-\03\01\04\11\06\0f\0c:\04\1d%_ m\04j%\80\c8\05\82\b0\03\1a\06\82\fd\03Y\07\16\09\18\09\14\0c\14\0cj\06\0a\06\1a\06Y\07+\05F\0a,\04\0c\04\01\031\0b,\04\1a\06\0b\03\80\ac\06\0a\06/1M\03\80\a4\08<\03\0f\03<\078\08+\05\82\ff\11\18\08/\11-\03!\0f!\0f\80\8c\04\82\97\19\0b\15\88\94\05/\05;\07\02\0e\18\09\80\be\22t\0c\80\d6\1a\0c\05\80\ff\05\80\df\0c\f2\9d\037\09\81\5c\14\80\b8\08\80\cb\05\0a\18;\03\0a\068\08F\08\0c\06t\0b\1e\03Z\04Y\09\80\83\18\1c\0a\16\09L\04\80\8a\06\ab\a4\0c\17\041\a1\04\81\da&\07\0c\05\05\80\a6\10\81\f5\07\01 *\06L\04\80\8d\04\80\be\03\1b\03\0f\0dlibrary/core/src/unicode/unicode_data.rs\c06\10\00(\00\00\00P\00\00\00(\00\00\00\c06\10\00(\00\00\00\5c\00\00\00\16\00\00\00library/core/src/escape.rs\00\00\087\10\00\1a\00\00\008\00\00\00\0b\00\00\00\5cu{\00\087\10\00\1a\00\00\00f\00\00\00#\00\00\00\00\03\00\00\83\04 \00\91\05`\00]\13\a0\00\12\17 \1f\0c `\1f\ef,\a0+*0 ,o\a6\e0,\02\a8`-\1e\fb`.\00\fe 6\9e\ff`6\fd\01\e16\01\0a!7$\0d\e17\ab\0ea9/\18\a190\1caH\f3\1e\a1L@4aP\f0j\a1QOo!R\9d\bc\a1R\00\cfaSe\d1\a1S\00\da!T\00\e0\e1U\ae\e2aW\ec\e4!Y\d0\e8\a1Y \00\eeY\f0\01\7fZ\00p\00\07\00-\01\01\01\02\01\02\01\01H\0b0\15\10\01e\07\02\06\02\02\01\04#\01\1e\1b[\0b:\09\09\01\18\04\01\09\01\03\01\05+\03<\08*\18\01 7\01\01\01\04\08\04\01\03\07\0a\02\1d\01:\01\01\01\02\04\08\01\09\01\0a\02\1a\01\02\029\01\04\02\04\02\02\03\03\01\1e\02\03\01\0b\029\01\04\05\01\02\04\01\14\02\16\06\01\01:\01\01\02\01\04\08\01\07\03\0a\02\1e\01;\01\01\01\0c\01\09\01(\01\03\017\01\01\03\05\03\01\04\07\02\0b\02\1d\01:\01\02\01\02\01\03\01\05\02\07\02\0b\02\1c\029\02\01\01\02\04\08\01\09\01\0a\02\1d\01H\01\04\01\02\03\01\01\08\01Q\01\02\07\0c\08b\01\02\09\0b\07I\02\1b\01\01\01\01\017\0e\01\05\01\02\05\0b\01$\09\01f\04\01\06\01\02\02\02\19\02\04\03\10\04\0d\01\02\02\06\01\0f\01\00\03\00\03\1d\02\1e\02\1e\02@\02\01\07\08\01\02\0b\09\01-\03\01\01u\02\22\01v\03\04\02\09\01\06\03\db\02\02\01:\01\01\07\01\01\01\01\02\08\06\0a\02\010\1f1\040\07\01\01\05\01(\09\0c\02 \04\02\02\01\038\01\01\02\03\01\01\03:\08\02\02\98\03\01\0d\01\07\04\01\06\01\03\02\c6@\00\01\c3!\00\03\8d\01` \00\06i\02\00\04\01\0a \02P\02\00\01\03\01\04\01\19\02\05\01\97\02\1a\12\0d\01&\08\19\0b.\030\01\02\04\02\02'\01C\06\02\02\02\02\0c\01\08\01/\013\01\01\03\02\02\05\02\01\01*\02\08\01\ee\01\02\01\04\01\00\01\00\10\10\10\00\02\00\01\e2\01\95\05\00\03\01\02\05\04(\03\04\01\a5\02\00\04\00\02P\03F\0b1\04{\016\0f)\01\02\02\0a\031\04\02\02\07\01=\03$\05\01\08>\01\0c\024\09\0a\04\02\01_\03\02\01\01\02\06\01\02\01\9d\01\03\08\15\029\02\01\01\01\01\16\01\0e\07\03\05\c3\08\02\03\01\01\17\01Q\01\02\06\01\01\02\01\01\02\01\02\eb\01\02\04\06\02\01\02\1b\02U\08\02\01\01\02j\01\01\01\02\06\01\01e\03\02\04\01\05\00\09\01\02\f5\01\0a\02\01\01\04\01\90\04\02\02\04\01 \0a(\06\02\04\08\01\09\06\02\03.\0d\01\02\00\07\01\06\01\01R\16\02\07\01\02\01\02z\06\03\01\01\02\01\07\01\01H\02\03\01\01\01\00\02\0b\024\05\05\01\01\01\00\01\06\0f\00\05;\07\00\01?\04Q\01\00\02\00.\02\17\00\01\01\03\04\05\08\08\02\07\1e\04\94\03\007\042\08\01\0e\01\16\05\01\0f\00\07\01\11\02\07\01\02\01\05d\01\a0\07\00\01=\04\00\04\00\07m\07\00`\80\f0\00")
  (data $.data (;1;) (i32.const 1063588) "\01\00\00\00\ff\ff\ff\ff8#\10\00")
  (@producers
    (language "Rust" "")
    (processed-by "rustc" "1.77.0-nightly (bf8716f1c 2023-12-24)")
    (processed-by "clang" "16.0.4 (https://github.com/llvm/llvm-project ae42196bc493ffe877a7e3dff8be32035dea4d07)")
  )
)