; ModuleID = 'text.txt.ll'
source_filename = "text.txt.ll"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

@print_int_fstring = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1

; Function Attrs: noinline nounwind optnone uwtable
define dso_local i32 @main() #0 {
	%x = alloca i64
	%1 = add nsw i64 7, 3
	store i64 %1, i64* %x
	%y = alloca i64
	%2 = udiv i64 12, 3
	store i64 %2, i64* %y
	%z = alloca i64
	%3 = load i64, i64* %x
	%4 = load i64, i64* %y
	%5 = sub nsw i64 %3, %4
	store i64 %5, i64* %z
	%6 = load i64, i64* %x
	%7 = load i64, i64* %y
	%8 = mul nsw i64 %6, %7
	%9 = load i64, i64* %x
	%10 = load i64, i64* %z
	%11 = mul nsw i64 %9, %10
	%12 = add nsw i64 %8, %11
	call i32(i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @print_int_fstring, i32 0, i32 0), i64 %12)
	store i64 2, i64* %x
	store i64 3, i64* %y
	%14 = load i64, i64* %x
	%15 = load i64, i64* %y
	%16 = mul nsw i64 %14, %15
	%17 = load i64, i64* %x
	%18 = load i64, i64* %z
	%19 = mul nsw i64 %17, %18
	%20 = add nsw i64 %16, %19
	call i32(i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @print_int_fstring, i32 0, i32 0), i64 %20)
	%abra_cada_bra = alloca i64
	store i64 2, i64* %abra_cada_bra
	%22 = load i64, i64* %x
	%23 = load i64, i64* %abra_cada_bra
	%24 = mul nsw i64 %22, %23
	%25 = load i64, i64* %y
	%26 = mul nsw i64 %24, %25
	%27 = load i64, i64* %z
	%28 = load i64, i64* %abra_cada_bra
	%29 = mul nsw i64 %27, %28
	%30 = sub nsw i64 %26, %29
	call i32(i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @print_int_fstring, i32 0, i32 0), i64 %30)
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