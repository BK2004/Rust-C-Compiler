; ModuleID = 'text.txt.ll'
source_filename = "text.txt.ll"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

@print_int_fstring = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1

; Function Attrs: noinline nounwind optnone uwtable
define dso_local i32 @main() #0 {
	%1 = alloca i32, align 4
	%2 = alloca i32, align 4
	%3 = alloca i32, align 4
	store i32 2, i32* %3
	store i32 3, i32* %2
	store i32 5, i32* %1
	%4 = load i32, i32* %2
	%5 = load i32, i32* %1
	%6 = mul nsw i32 %4, %5
	%7 = load i32, i32* %3
	%8 = sub nsw i32 %7, %6
	call i32(i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @print_int_fstring, i32 0, i32 0), i32 %8)
	%10 = alloca i32, align 4
	%11 = alloca i32, align 4
	%12 = alloca i32, align 4
	%13 = alloca i32, align 4
	%14 = alloca i32, align 4
	%15 = alloca i32, align 4
	%16 = alloca i32, align 4
	store i32 7, i32* %16
	store i32 32, i32* %15
	%17 = load i32, i32* %16
	%18 = load i32, i32* %15
	%19 = mul nsw i32 %17, %18
	store i32 8, i32* %14
	store i32 2, i32* %13
	%20 = load i32, i32* %14
	%21 = load i32, i32* %13
	%22 = udiv i32 %20, %21
	store i32 3, i32* %12
	%23 = load i32, i32* %12
	%24 = mul nsw i32 %22, %23
	%25 = sub nsw i32 %19, %24
	store i32 5, i32* %11
	store i32 2, i32* %10
	%26 = load i32, i32* %11
	%27 = load i32, i32* %10
	%28 = mul nsw i32 %26, %27
	%29 = add nsw i32 %25, %28
	call i32(i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @print_int_fstring, i32 0, i32 0), i32 %29)
	%31 = alloca i32, align 4
	%32 = alloca i32, align 4
	%33 = alloca i32, align 4
	%34 = alloca i32, align 4
	%35 = alloca i32, align 4
	store i32 5, i32* %35
	store i32 9, i32* %34
	store i32 3, i32* %33
	%36 = load i32, i32* %34
	%37 = load i32, i32* %33
	%38 = udiv i32 %36, %37
	%39 = load i32, i32* %35
	%40 = add nsw i32 %39, %38
	store i32 3, i32* %32
	%41 = load i32, i32* %32
	%42 = add nsw i32 %40, %41
	store i32 1, i32* %31
	%43 = load i32, i32* %31
	%44 = sub nsw i32 %42, %43
	%45 = add i32 1024, 0
	call i32(i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @print_int_fstring, i32 0, i32 0), i32 %45)
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