; ModuleID = 'text.txt.ll'
source_filename = "text.txt.ll"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

@print_int_fstring = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1

; Function Attrs: noinline nounwind optnone uwtable
define dso_local i32 @main() #0 {
	%1 = icmp ne i64 3, 2
	%ne = alloca i1
	store i1 %1, i1* %ne
	%2 = icmp eq i64 3, 2
	%eq = alloca i1
	store i1 %2, i1* %eq
	%3 = icmp sgt i64 3, 2
	%gt = alloca i1
	store i1 %3, i1* %gt
	%4 = icmp sge i64 3, 2
	%ge = alloca i1
	store i1 %4, i1* %ge
	%5 = icmp slt i64 3, 2
	%lt = alloca i1
	store i1 %5, i1* %lt
	%6 = icmp slt i64 3, 2
	%le = alloca i1
	store i1 %6, i1* %le
	%xyz = alloca i64
	%7 = icmp slt i64 6, 2
