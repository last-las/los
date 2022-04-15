use super::*;
use user_lib::syscall::{virt_to_phys, continuous_alloc};

type VirtAddr = usize;
type PhysAddr = usize;

pub struct DMA {
    paddr: u32,
    pages: u32,
}

impl DMA {
    pub fn new(pages: usize) -> Result<Self> {
        let paddr = unsafe { virtio_dma_alloc(pages) };
        if paddr == 0 {
            return Err(Error::DmaError);
        }
        Ok(DMA {
            paddr: paddr as u32,
            pages: pages as u32,
        })
    }

    pub fn paddr(&self) -> usize {
        self.paddr as usize
    }

    pub fn vaddr(&self) -> usize {
        unsafe {
            self.paddr() + OFFSET
        }
    }

    /// Page frame number
    pub fn pfn(&self) -> u32 {
        self.paddr >> 12
    }

    /// Convert to a buffer
    pub unsafe fn as_buf(&self) -> &'static mut [u8] {
        core::slice::from_raw_parts_mut(self.vaddr() as _, PAGE_SIZE * self.pages as usize)
    }
}

impl Drop for DMA {
    fn drop(&mut self) {
        let err = unsafe { virtio_dma_dealloc(self.paddr as usize, self.pages as usize) };
        assert_eq!(err, 0, "failed to deallocate DMA");
    }
}

static mut OFFSET: usize = 0;

/// This function will only be invoked by virtio_driver once when creating `DMA` structure.
pub fn virtio_dma_alloc(pages: usize) -> usize {
    let size = pages * PAGE_SIZE;
    let virt_addr = continuous_alloc(size).unwrap();
    let phys_addr = virt_to_phys(virt_addr).unwrap();
    unsafe {
        OFFSET = virt_addr - phys_addr;
    }

    phys_addr
}

/// This function will only be invoked by virtio_driver once when the driver is terminated,
/// So actually we don't need to free anything, because the kernel will recycle the memory in the end.
pub fn virtio_dma_dealloc(_: usize, _: usize) -> i32 {
    0
}

/*extern "C" {
    fn virtio_dma_alloc(pages: usize) -> PhysAddr;
    fn virtio_dma_dealloc(paddr: PhysAddr, pages: usize) -> i32;
    fn virtio_phys_to_virt(paddr: PhysAddr) -> VirtAddr;
    fn virtio_virt_to_phys(vaddr: VirtAddr) -> PhysAddr;
}*/