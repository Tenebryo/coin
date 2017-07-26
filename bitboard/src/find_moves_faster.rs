///This is an attempt to port the Edax move generation function to rust.

#[allow(non_snake_case)]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn fast_find_moves(P : u64, O : u64) -> u64 {
    const mask_7e : u64 = 0x7e7e7e7e7e7e7e7eu64;
    let mut moves : u64 = 0;

    unsafe{
        asm!("
            movl   $3, %esi

            movq   $1, %mm7

            movl   $4, %edi

            movq   $2, %mm6

            movl   %esi, %eax
            movq   %mm7, %mm0

            movq   $5, %mm5

            shrl   $$1, %eax

            psrlq  $$8, %mm0

            andl   $$2122219134, %edi
            pand   %mm6, %mm5

            andl   %edi, %eax
            pand   %mm6, %mm0
            movl   %eax, %edx
            movq   %mm0, %mm1

            shrl   $$1, %eax

            psrlq  $$8, %mm0

            movl   %edi, %ecx
            movq   %mm6, %mm3

            andl   %edi, %eax
            pand   %mm6, %mm0
            shrl   $1, %ecx

            psrlq  $$8, %mm3

            orl    %edx, %eax
            por    %mm1, %mm0
            andl   %edi, %ecx
            pand   %mm6, %mm3
            movl   %eax, %edx
            movq   %mm0, %mm4

            shrl   $$2, %eax

            psrlq  $$16, %mm0

            andl   %ecx, %eax
            pand   %mm3, %mm0
            orl    %eax, %edx
            por    %mm0, %mm4

            shrl   $$2, %eax

            psrlq  $$16, %mm0

            andl   %ecx, %eax
            pand   %mm3, %mm0
            orl    %edx, %eax
            por    %mm0, %mm4

            shrl   $$1, %eax

            psrlq  $$8, %mm4
            movq   %mm7, %mm0

            addl   %esi, %esi
            psllq  $$8, %mm0

            andl   %edi, %esi
            pand   %mm6, %mm0

            movl   %esi, %edx
            movq   %mm0, %mm1

            addl   %esi, %esi
            psllq  $$8, %mm0

            andl   %edi, %esi
            pand   %mm6, %mm0

            orl    %esi, %edx
            por    %mm1, %mm0

            addl   %ecx, %ecx
            psllq  $$8, %mm3

            movq   %mm0, %mm1

            leal   (,%edx,4), %esi
            psllq  $$16, %mm0

            andl   %ecx, %esi
            pand   %mm3, %mm0

            orl    %esi, %edx
            por    %mm0, %mm1

            shll   $$2, %esi

            psllq  $$16, %mm0

            andl   %ecx, %esi
            pand   %mm3, %mm0

            orl    %edx, %esi
            por    %mm1, %mm0

            addl   %esi, %esi
            psllq  $$8, %mm0

            orl    %eax, %esi
            por    %mm0, %mm4

            movq   %mm7, %mm0

            movd   %esi, %mm1

            psrlq  $$7, %mm0

            psllq  $$32, %mm1

            pand   %mm5, %mm0

            por    %mm1, %mm4

            movq   %mm0, %mm1

            psrlq  $$7, %mm0

            pand   %mm5, %mm0

            movq   %mm5, %mm3

            por    %mm1, %mm0

            psrlq  $$7, %mm3

            movq   %mm0, %mm1

            pand   %mm5, %mm3

            psrlq  $$14, %mm0

            pand   %mm3, %mm0

            movl   $1, %esi

            por    %mm0, %mm1

            movl   $2, %edi

            psrlq  $$14, %mm0

            andl   $$2122219134, %edi
            pand   %mm3, %mm0

            movl   %edi, %ecx
            por    %mm1, %mm0

            shrl   $$1, %ecx

            psrlq  $$7, %mm0

            andl   %edi, %ecx
            por    %mm0, %mm4

            movl   %esi, %eax
            movq   %mm7, %mm0

            shrl   $$1, %eax

            psllq  $$7, %mm0

            andl   %edi, %eax
            pand   %mm5, %mm0

            movl   %eax, %edx
            movq   %mm0, %mm1

            shrl   $$1, %eax

            psllq  $$7, %mm0

            andl   %edi, %eax
            pand   %mm5, %mm0

            orl    %edx, %eax
            por    %mm1, %mm0

            psllq  $$7, %mm3

            movl   %eax, %edx
            movq   %mm0, %mm1

            shrl   $$2, %eax

            psllq  $$14, %mm0

            andl   %ecx, %eax
            pand   %mm3, %mm0

            orl    %eax, %edx
            por    %mm0, %mm1

            shrl   $$2, %eax

            psllq  $$14, %mm0

            andl   %ecx, %eax
            pand   %mm3, %mm0

            orl    %edx, %eax
            por    %mm1, %mm0

            shrl   $$1, %eax

            psllq  $$7, %mm0

            por    %mm0, %mm4

            movq   %mm7, %mm0

            addl   %esi, %esi
            psrlq  $$9, %mm0

            andl   %edi, %esi
            pand   %mm5, %mm0

            movl   %esi, %edx
            movq   %mm0, %mm1

            addl   %esi, %esi
            psrlq  $$9, %mm0

            andl   %edi, %esi
            pand   %mm5, %mm0

            movq   %mm5, %mm3

            orl    %esi, %edx
            por    %mm1, %mm0

            psrlq  $$9, %mm3

            movq   %mm0, %mm1

            addl   %ecx, %ecx
            pand   %mm5, %mm3

            leal   (,%edx,4), %esi
            psrlq  $$18, %mm0

            andl   %ecx, %esi
            pand   %mm3, %mm0

            orl    %esi, %edx
            por    %mm0, %mm1

            shll   $$2, %esi

            psrlq  $$18, %mm0

            andl   %ecx, %esi
            pand   %mm3, %mm0

            orl    %edx, %esi
            por    %mm1, %mm0

            addl   %esi, %esi
            psrlq  $$9, %mm0

            orl    %eax, %esi
            por    %mm0, %mm4

            movq   %mm7, %mm0

            movd   %esi, %mm1

            psllq  $$9, %mm0

            por    %mm1, %mm4

            pand   %mm5, %mm0

            movq   %mm0, %mm1

            psllq  $$9, %mm0

            pand   %mm5, %mm0

            por    %mm1, %mm0

            psllq  $$9, %mm3

            movq   %mm0, %mm1

            psllq  $$18, %mm0

            pand   %mm3, %mm0

            por    %mm0, %mm1

            psllq  $$18, %mm0

            pand   %mm3, %mm0

            por    %mm1, %mm0

            psllq  $$9, %mm0

            por    %mm0, %mm4

            por    %mm6, %mm7

            pandn  %mm4, %mm7

            movq   %mm7, $0

            emms
        "
        : "=r" (moves) : "m" (P), "m" (O), "m" ((P >> 32) as u32), "m" ((O >> 32) as u32), "m" (mask_7e) : "eax", "edx", "ecx", "esi", "edi" : "volatile");
    }
    moves
}