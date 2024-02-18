; ModuleID = 'text.txt.ll'
source_filename = "text.txt.ll"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

@print_int_fstring = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1

; Function Attrs: noinline nounwind optnone uwtable
define dso_local i32 @main() #0 {
	%x = alloca i64
	%1 = mul nsw i64 3, 2
	%2 = add nsw i64 7, %1
	store i64 %2, i64* %x
	%y = alloca i64
	%3 = udiv i64 12, 3
	store i64 %3, i64* %y
	%z = alloca i64
	%4 = load i64, i64* %x
	%5 = load i64, i64* %y
	%6 = sub nsw i64 %4, %5
	store i64 %6, i64* %z
	%7 = load i64, i64* %x
	%8 = load i64, i64* %y
	%9 = mul nsw i64 %7, %8
	%10 = load i64, i64* %x
	%11 = load i64, i64* %z
	%12 = mul nsw i64 %10, %11
	%13 = add nsw i64 %9, %12
	call i32(i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @print_int_fstring, i32 0, i32 0), i64 %13)
	store i64 2, i64* %x
	store i64 3, i64* %y
	%15 = load i64, i64* %x
	%16 = load i64, i64* %y
	%17 = mul nsw i64 %15, %16
	%18 = load i64, i64* %x
	%19 = load i64, i64* %z
	%20 = mul nsw i64 %18, %19
	%21 = add nsw i64 %17, %20
	call i32(i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @print_int_fstring, i32 0, i32 0), i64 %21)
	%abra_cada_bra = alloca i64
	store i64 2, i64* %abra_cada_bra
	%23 = load i64, i64* %x
	%24 = load i64, i64* %abra_cada_bra
	%25 = mul nsw i64 %23, %24
	%26 = load i64, i64* %y
	%27 = mul nsw i64 %25, %26
	%28 = load i64, i64* %z
	%29 = load i64, i64* %abra_cada_bra
	%30 = mul nsw i64 %28, %29
	%31 = sub nsw i64 %27, %30
	call i32(i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @print_int_fstring, i32 0, i32 0), i64 %31)
	ret i32 0
}

declare i32 @printf(i8*, ...) #1

attributes #0 = { noinline nounwind optnone uwtable "frame-pointer"="all" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #1 = { "frame-pointer"="all" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }

!llvm.module.flags = !{!0, !1, !2, !3, !4}
!llvm.ident = !{!5}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 7, !"PIC Level", i32 2}
!2 = !{i32 7, !"PIE Level", i32 2}
!3 = !{i32 7, !"uwtable", i32 1}
!4 = !{i32 7, !"frame-pointer", i32 2}
!5 = !{!"ICD compiler"}