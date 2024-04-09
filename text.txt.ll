; ModuleID = 'text.txt.ll'
source_filename = "text.txt.ll"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

@print_int_fstring = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1

define dso_local i64 @main() #0 {
	%i = alloca i64
	store i64 1, i64* %i
	br label %label.0
label.0:
	%1 = load i64, i64* %i
	%2 = icmp slt i64 %1, 10
	br i1 %2, label %label.1, label %label.2
label.1:
	%3 = load i64, i64* %i
	call i32(i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @print_int_fstring, i32 0, i32 0), i64 %3)
	%5 = load i64, i64* %i
	%6 = add nsw i64 %5, 1
	store i64 %6, i64* %i
	br label %label.0
label.2:
	%x = alloca i64*
	store i64* %i, i64** %x
	%7 = load i64*, i64** %x
	store i64 7, i64* %7
	%8 = load i64*, i64** %x
	%9 = load i64, i64* %8
	%y = alloca i64
	store i64 %9, i64* %y
	%10 = load i64*, i64** %x
	%11 = load i64, i64* %10
	call i32(i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @print_int_fstring, i32 0, i32 0), i64 %11)
	ret i64 0
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