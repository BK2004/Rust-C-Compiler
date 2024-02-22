; ModuleID = 'text.txt.ll'
source_filename = "text.txt.ll"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

@print_int_fstring = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1

; Function Attrs: noinline nounwind optnone uwtable
define dso_local i32 @main() #0 {
	%x = alloca i64
	store i64 3, i64* %x
	%y = alloca i64
	store i64 2, i64* %y
	%1 = load i64, i64* %x
	%2 = load i64, i64* %y
	%3 = add nsw i64 %1, %2
	%a = alloca i64
	store i64 %3, i64* %a
	%4 = load i64, i64* %x
	%5 = load i64, i64* %y
	%6 = sub nsw i64 %4, %5
	%b = alloca i64
	store i64 %6, i64* %b
	%7 = load i64, i64* %a
	%8 = load i64, i64* %b
	%9 = icmp sgt i64 %7, %8
	br i1 %9, label %label.0, label %label.1
label.0:
	%10 = load i64, i64* %a
	call i32(i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @print_int_fstring, i32 0, i32 0), i64 %10)
	%12 = load i64, i64* %b
	%13 = load i64, i64* %a
	%14 = sub nsw i64 %12, %13
	%15 = load i64, i64* %y
	%16 = icmp sgt i64 %14, %15
	br i1 %16, label %label.3, label %label.4
label.3:
	%17 = load i64, i64* %y
	call i32(i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @print_int_fstring, i32 0, i32 0), i64 %17)
	br label %label.5
label.4:
	%19 = load i64, i64* %x
	%20 = load i64, i64* %y
	%21 = icmp sgt i64 %19, %20
	br i1 %21, label %label.6, label %label.7
label.6:
	%22 = load i64, i64* %x
	call i32(i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @print_int_fstring, i32 0, i32 0), i64 %22)
	br label %label.8
label.7:
	%24 = load i64, i64* %y
	call i32(i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @print_int_fstring, i32 0, i32 0), i64 %24)
	br label %label.8
label.8:
	br label %label.5
label.5:
	br label %label.2
label.1:
	%26 = load i64, i64* %b
	call i32(i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @print_int_fstring, i32 0, i32 0), i64 %26)
	br label %label.2
label.2:
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