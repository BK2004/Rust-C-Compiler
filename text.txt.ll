; ModuleID = 'text.txt.ll'
source_filename = "text.txt.ll"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

@print_int_fstring = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1

define dso_local i64 @help(i64 %arg.0,i64 %arg.1) #0 {
	%x = alloca i64
	store i64 %arg.0, i64* %x
	%z = alloca i64
	store i64 %arg.1, i64* %z
	call i32(i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @print_int_fstring, i32 0, i32 0), i64 3)
	%2 = load i64, i64* %x
	%3 = load i64, i64* %z
	%4 = icmp slt i64 %2, %3
	br i1 %4, label %label.0, label %label.1
label.0:
	%5 = load i64, i64* %x
	ret i64 %5
	br label %label.2
label.1:
	%7 = load i64, i64* %z
	ret i64 %7
	br label %label.2
label.2:
	ret i64 0
}

define dso_local i64 @mult(i64 %arg.0,i64 %arg.1) #0 {
	%x = alloca i64
	store i64 %arg.0, i64* %x
	%y = alloca i64
	store i64 %arg.1, i64* %y
	%1 = load i64, i64* %x
	%2 = load i64, i64* %y
	%3 = mul nsw i64 %1, %2
	%mult_res = alloca i64
	store i64 %3, i64* %mult_res
	%4 = load i64, i64* %mult_res
	ret i64 %4
}

define dso_local i64 @fib(i64 %arg.0) #0 {
	%n = alloca i64
	store i64 %arg.0, i64* %n
	%1 = load i64, i64* %n
	%2 = icmp eq i64 %1, 1
	br i1 %2, label %label.3, label %label.4
label.3:
	ret i64 1
	br label %label.4
label.4:
	%4 = load i64, i64* %n
	%5 = icmp eq i64 %4, 0
	br i1 %5, label %label.5, label %label.6
label.5:
	ret i64 0
	br label %label.6
label.6:
	%7 = load i64, i64* %n
	%8 = sub nsw i64 %7, 1
	%9 = call i64 @fib(i64 %8)
	%10 = load i64, i64* %n
	%11 = sub nsw i64 %10, 2
	%12 = call i64 @fib(i64 %11)
	%13 = add nsw i64 %9, %12
	ret i64 %13
}

define dso_local i1 @is_even(i64 %arg.0) #0 {
	%n = alloca i64
	store i64 %arg.0, i64* %n
	%1 = load i64, i64* %n
	%2 = udiv i64 %1, 2
	%3 = mul nsw i64 %2, 2
	%4 = load i64, i64* %n
	%5 = icmp eq i64 %3, %4
	ret i1 %5
}

define dso_local i64 @main() #0 {
	%i = alloca i64
	store i64 1, i64* %i
	br label %label.7
label.7:
	%1 = load i64, i64* %i
	%2 = icmp slt i64 %1, 200
	br i1 %2, label %label.8, label %label.9
label.8:
	%3 = load i64, i64* %i
	call i32(i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @print_int_fstring, i32 0, i32 0), i64 %3)
	%5 = load i64, i64* %i
	%6 = call i1 @is_even(i64 %5)
	call i32(i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @print_int_fstring, i32 0, i32 0), i1 %6)
	%8 = load i64, i64* %i
	%9 = add nsw i64 %8, 1
	store i64 %9, i64* %i
	br label %label.7
label.9:
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