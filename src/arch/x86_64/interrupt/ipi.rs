
use device::local_apic::LOCAL_APIC;

interrupt!(ipi, {
    LOCAL_APIC.eoi();
});

interrupt!(pit, {
    LOCAL_APIC.eoi();

/*    if PIT_TICKS.fetch_add(1, Ordering::SeqCst) >= 10 {
        let _ = context::switch();
    }
*/
});
