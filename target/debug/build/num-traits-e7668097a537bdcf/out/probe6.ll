; ModuleID = 'probe6.3d6d369e-cgu.0'
source_filename = "probe6.3d6d369e-cgu.0"
target datalayout = "e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128"
target triple = "aarch64-unknown-linux-gnu"

%"core::panic::location::Location" = type { { [0 x i8]*, i64 }, i32, i32 }

@alloc1 = private unnamed_addr constant <{ [71 x i8] }> <{ [71 x i8] c"/build/rustc-gI3M04/rustc-1.57.0+dfsg1+llvm/library/core/src/num/mod.rs" }>, align 1
@alloc2 = private unnamed_addr constant <{ i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [71 x i8] }>, <{ [71 x i8] }>* @alloc1, i32 0, i32 0, i32 0), [16 x i8] c"G\00\00\00\00\00\00\00$\03\00\00\05\00\00\00" }>, align 8
@str.0 = internal constant [25 x i8] c"attempt to divide by zero"

; probe6::probe
; Function Attrs: nonlazybind uwtable
define void @_ZN6probe65probe17h05c2bc642e4ecbefE() unnamed_addr #0 {
start:
  %0 = call i1 @llvm.expect.i1(i1 false, i1 false)
  br i1 %0, label %panic.i, label %"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h2f5a5646e2a317e2E.exit"

panic.i:                                          ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h04849d7997617cfbE([0 x i8]* nonnull align 1 bitcast ([25 x i8]* @str.0 to [0 x i8]*), i64 25, %"core::panic::location::Location"* align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @alloc2 to %"core::panic::location::Location"*)) #3
  unreachable

"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h2f5a5646e2a317e2E.exit": ; preds = %start
  br label %bb1

bb1:                                              ; preds = %"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h2f5a5646e2a317e2E.exit"
  ret void
}

; Function Attrs: nofree nosync nounwind readnone willreturn
declare i1 @llvm.expect.i1(i1, i1) #1

; core::panicking::panic
; Function Attrs: cold noinline noreturn nonlazybind uwtable
declare void @_ZN4core9panicking5panic17h04849d7997617cfbE([0 x i8]* nonnull align 1, i64, %"core::panic::location::Location"* align 8 dereferenceable(24)) unnamed_addr #2

attributes #0 = { nonlazybind uwtable "target-cpu"="generic" }
attributes #1 = { nofree nosync nounwind readnone willreturn }
attributes #2 = { cold noinline noreturn nonlazybind uwtable "target-cpu"="generic" }
attributes #3 = { noreturn }

!llvm.module.flags = !{!0, !1}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}