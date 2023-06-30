# XCM Pallet

## Overview

Dictionary meaning:
![](../../img/pallet-xcm-7.png)

XCMP vs XCM:
![](../../img/pallet-xcm-8.png)

XCMP: Cross-chain messaging protocol
![](../../img/pallet-xcm-9.png)

![](../../img/pallet-xcm-10.png)

2 channels open during XCMP:

- Sending message
- Receiving message

There is a handshake 🤝 mechanism followed in order to avoid malicious messages coming from malicious parachains. The XCMP is only allowed if both chains agree to open respective channels.

**VMP: Vertical Message Passing**
![](../../img/pallet-xcm-11.png)

We generally are not supposed to have any message format as it would consider them as bytes.

But, we do have a message standard defined.
![](../../img/pallet-xcm-12.png)

That's where XCM comes in:
![](../../img/pallet-xcm-13.png)

Cross-consensus messaging
![](../../img/pallet-xcm-14.png)

It is designed to work across any consensus systems and not just polkadot or parachains.
![](../../img/pallet-xcm-15.png)

![](../../img/pallet-xcm-16.png)

Dig deeper? Here it is simplified:

![](../../img/pallet-xcm-17.png)

![](../../img/pallet-xcm-18.png)

![](../../img/pallet-xcm-19.png)

![](../../img/pallet-xcm-20.png)

![](../../img/pallet-xcm-21.png)

![](../../img/pallet-xcm-22.png)

![](../../img/pallet-xcm-23.png)

![](../../img/pallet-xcm-24.png)

Where does XCM fit in the Polkadot ecosystem?
![](../../img/pallet-xcm-25.png)

Applications:

![](../../img/pallet-xcm-29.png)

![](../../img/pallet-xcm-30.png)

![](../../img/pallet-xcm-31.png)

![](../../img/pallet-xcm-32.png)

## Notes

### Theory

![](../../img/pallet-xcm-1.png)

![](../../img/pallet-xcm-2.png)

Here, only XCMP is used, it works fine:
![](../../img/pallet-xcm-3.png)

But the problem happens when one of the parachains is upgraded. The coordination of upgrade can be very difficult. Hence, the messages are getting sent to the wrong place and wrong actions take place.  
![](../../img/pallet-xcm-4.png)

So, XCM defines a common language for parachains to communicate with each other.
![](../../img/pallet-xcm-26.png)

So, that's why we need a XCM pallet (executer). It's a pallet that allows us to send messages between parachains irrespective of whether a parachain is upgraded or not.
![](../../img/pallet-xcm-5.png)

```mermaid
graph LR
A[Send Transfer Message on Parachain-1] --> B[Well formed instructions] --> C[bytes] --> D[XCM Executor] --> E[Execute Transfer Message on Parachain-2]
```

![](../../img/pallet-xcm-27.png)

![](../../img/pallet-xcm-28.png)

![](../../img/pallet-xcm-6.png)

![](../../img/pallet-xcm-33.png)

![](../../img/pallet-xcm-34.png)

### Coding

- **Q**. How can an asset be withdrawn and deposited? Which assets can be transacted?

  - **Asset Transactor** (as shown in `XCM Executer` diagram).
    ![](../../img/pallet-xcm-35.png)

<!-- TODO: Cover later after all the pallets -->

## References

- [Shawn Tabrizi: XCM - The Backbone Of A Multichain Future | Polkadot Decoded 2022](https://www.youtube.com/watch?v=2tmspefsygQ) ✅
- [XCM Overview | Polkadot Deep Dives](https://www.youtube.com/watch?v=kAAzgpTAMZ4&list=PLOyWqupZ-WGsfnlpkk0KWX3uS4yg6ZztG&index=20)
- [XCM Config & Pallet-XCM | Polkadot Deep Dives](https://www.youtube.com/watch?v=bFMvWmU1pYI&list=PLOyWqupZ-WGsfnlpkk0KWX3uS4yg6ZztG&index=27) 🧑🏻‍💻
- [XCM v3 | Polkadot Deep Dives](https://www.youtube.com/watch?v=MMIPNR3SuB4&list=PLOyWqupZ-WGsfnlpkk0KWX3uS4yg6ZztG&index=18)
- [Workshop 11 | “Getting Started with XCM" by Steve Degosserie | IBC Continuum 2022-23](https://www.youtube.com/watch?v=D90bbadkNcE)
