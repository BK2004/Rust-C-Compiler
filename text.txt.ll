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
	%9 = 	call i64 @fib(i64 %8)
	%10 = load i64, i64* %n
	%11 = sub nsw i64 %10, 2
	%12 = 	call i64 @fib(i64 %11)
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

define dso_local void @return_nothing(i1 %arg.0) #0 {
	%x = alloca i1
	store i1 %arg.0, i1* %x
	%1 = load i1, i1* %x
	call i32(i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @print_int_fstring, i32 0, i32 0), i1 %1)
	%3 = load i1, i1* %x
	br i1 %3, label %label.7, label %label.8
label.7:
	%4 = load i1, i1* %x
	call i32(i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @print_int_fstring, i32 0, i32 0), i1 %4)
	br label %label.8
label.8:
	ret void
}

define dso_local i64 @add(i64 %arg.0,i64 %arg.1) #0 {
	%x = alloca i64
	store i64 %arg.0, i64* %x
	%y = alloca i64
	store i64 %arg.1, i64* %y
	%1 = load i64, i64* %x
	%2 = load i64, i64* %y
	%3 = add nsw i64 %1, %2
	ret i64 %3
}

define dso_local i64 @main() #0 {
	%i = alloca i64
	store i64 1, i64* %i
	br label %label.9
label.9:
	%1 = load i64, i64* %i
	%2 = icmp slt i64 %1, 10
	br i1 %2, label %label.10, label %label.11
label.10:
	%3 = load i64, i64* %i
	call i32(i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @print_int_fstring, i32 0, i32 0), i64 %3)
	%5 = load i64, i64* %i
	%6 = 	call i1 @is_even(i64 %5)
	call i32(i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @print_int_fstring, i32 0, i32 0), i1 %6)
	%8 = load i64, i64* %i
	%9 = add nsw i64 %8, 1
	store i64 %9, i64* %i
	br label %label.9
label.11:
	%10 = icmp sgt i64 2, 1
		call void @return_nothing(i1 %10)
	call i32(i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @print_int_fstring, i32 0, i32 0), i64 333)
	%12 = icmp sgt i64 1, 2
		call void @return_nothing(i1 %12)
	%13 = load i64, i64* %i
	%14 = 	call i64 @add(i64 3,i64 %13)
	call i32(i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @print_int_fstring, i32 0, i32 0), i64 %14)
	%16 = icmp eq i64 2, 3
		call void @return_nothing(i1 %16)
