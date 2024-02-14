; ModuleID = 'text.txt.ll'
source_filename = "text.txt.ll"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

@print_int_fstring = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1

; Function Attrs: noinline nounwind optnone uwtable
define dso_local i32 @main() #0 {
	%x = alloca i32
	%1 = mul nsw i32 3, 2
	%2 = add nsw i32 7, %1
	store i32 %2, i32* %x
	%y = alloca i32
	%3 = udiv i32 12, 3
	store i32 %3, i32* %y
	%z = alloca i32
	%4 = load i32, i32* %x
	%5 = load i32, i32* %y
	%6 = sub nsw i32 %4, %5
	store i32 %6, i32* %z
	%7 = load i32, i32* %x
	%8 = load i32, i32* %y
	%9 = mul nsw i32 %7, %8
	%10 = load i32, i32* %x
	%11 = load i32, i32* %z
	%12 = mul nsw i32 %10, %11
	%13 = add nsw i32 %9, %12
	call i32(i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @print_int_fstring, i32 0, i32 0), i32 %13)
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