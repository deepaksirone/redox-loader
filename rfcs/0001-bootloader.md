- Feature Name: multiboot
- Start Date: 2016-06-13
- RFC PR:
- pambu-os Issue:

# Summary
[summary]: #summary

Kernel conforms to the multiboot specification v1.6

# Motivation
[motivation]: #motivation

Please refer http://nongnu.askapache.com/grub/phcoder/multiboot.pdf

# Detailed design
[design]: #detailed-design


# Drawbacks
[drawbacks]: #drawbacks

The specification doesn't define 64bit entry points, and doesn't define the 64bit processor handoff states. In reality, this introduces some issues in some special case. for example, when the Bootloader is 64bit and the OS image is also 64bit, then if both bootloader and OS are multiboot-compliant, the extra code logics must be introduced: 1) for bootloader, it must have to do processor mode switch from 64bit to 32bit in order to call the 32bit entry of OS image, and 2) for OS, it must also have to add a similar code stub to do processor mode in reverse order.
Ref. http://hypervsir.blogspot.in/2014/09/limitations-of-multiboot-specification.html

# Alternatives
[alternatives]: #alternatives

Custom specification. Needs a custom bootloader as well.

# Unresolved questions
[unresolved]: #unresolved-questions
