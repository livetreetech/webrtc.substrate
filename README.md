# webrtc.substrate
Extensions for webrtc 
This project brings the vision of Web 3.0 to reality.  WebRTC provides real-time communication for the web and is largely how real-time peer-to-peer video works online (Google Hangouts, Facebook messenger etc). This project is brought to you by Livetree.  Livetree.com is the home for documentaries, feature films, TV series and your live video stories that shine a light on world issues.  We have built an extensive video cloud that includes edge-based CDN WebAssembly distribution, video transcoding, DRM and a royalty infrastructure, that can scale video machine-learning algorithms. 


Overview
The project aims to enable two landmark goals

Facilitate the funding of scientifically vetted causes via a series of challenges for example, think of the 3 models for scientific paper publishing a) publications via journals e.g. Science Journal vetted by an often unpaid panel of university professors b) via https://arxiv.org/ the open source publication website, c) conferences  which given the current climate are difficult to say the least. Twitter vetting of pre-published papers has become the norm and this presents a market opportunity for this Substrate WebRTC software extension to harness the popularity of live web video e.g. TIKTOK. 

Enable the general public to be rewarded in the form of Programmatic Advertising (PMP/RTB) via Substrate and IPFS/IPLD for their data using video impressions managed through WebRTC/Substrate. To grasp the scale of the potential of this project, envisage TIKTOK  but where ad revenue is directly returned back to the audience, who can then donate it to scientific Causes inline with the UN 17 SDGs. This transformational vision directly links rewards to user generated content namely in the form of WebRTC/Substrate and harnesses the last 3 years of Livetree’s video cloud technology.

We feel this project could be a good foundation for an ecosystem of video apps within a Web 3.0 environment.

We envisage an extension of  WebRTC’s “Session Description Protocol” or potentially adding Substrate to WebRTC WebAssembly’s (such as PION) will occur over 3 months. The first month will be used to create the basic substrate runtime/modules, the second month links store and scale out playback (ideally both Azure Blob Storage and IPFS) via WebRTC, and the final 3rd month would join the substrate and WebRTC implementations together to generate further community extensions. We see Polkadot as a way to bridge from Ethereum (e.g. ERC20 tokens) to Substrate and vice versa.  

The world class team is headed by Ashley Turing, founder of Livetree, and includes the UseTech team who have developed previous projects including a .NET wrapper, which will be useful to the back-end transaction system, which is coded in Azure webapi .NET.


Team members
Ashley Turing will be leading the team.

Alexander Mitrovich consultant from the Usetech team  https://www.linkedin.com/in/alexandermitrovich/




Team Website
https://usetech.com/blockchain.html

https://www.livetree.com/

https://www.livetree.com/partners


Legal Structure
Seed Intelligence Limited company based in London, UK.


Team's experience
Ashley Turing started Livetree.com in honour of Aaron Swartz, Reddit founder. Our mission is to use storytelling to bring us together to tackle the global crisis we face today in the environment and ourselves. We originally started as a crowdfunding website and have built an extensive video streaming platform to accompany the crowdfunding functionality.  This includes transcoding, encoding and a distribution infrastructure that includes real-time video, chat facilities and interactive screenings (visit Livetree.com). We launched an ERC20 token called Livetree Seed, and have experience developing smart contracts using Ethereum. We would like to build our crowdfunding capabilities on Substrate. We envisage that the Substrate blockchain can be used to reward users for their data namely through ad revenue generated through programmatic advertising for each of their uploaded videos. Similar to TIKTOK but for stories that bring about funding for causes.  Likewise the same Substrate blockchain will add the ability for end users to contribute to scientific crowdfunded causes through video challenges.  We see Substrate provides an audit for users data and handles the governance from their contributions in terms of monetary funds to a collective. 


UseTech have worked with Parity before building a .NET adapter. This along with Rust development will be the basis of the system.  We imagine that we will be collaborating with the core Parity team as needed for technical guidance. 


Team Code Repos
https://github.com/livetreetech/webrtc.substrate 

https://github.com/usetech-llc 


Team LinkedIn Profiles
https://www.linkedin.com/in/ashley-turing/

Usetech Consultants under https://www.linkedin.com/in/alexandermitrovich/  


Development Roadmap
This section breaks the development roadmap into a number of milestones. We have detailed each milestone as much as possible with the expectation that it will appear in the grant contract. 

Grant request key facts:

Innovative use of Substrate and Polkadot.

Timeframe: 3 months.

Apache 2.0 License 

 The Collective has the following proposals of type trait. 

Challenges - e.g. Ice bucket challenge and these also act as a type of proposal which can be voted on 

Causes - Represents a pool of tokens backed by Fiat. For example a crowdfunded item i.e. has total amount raised vs target raise / time remaining and other functionality already offered by Livetree

Payout  - who to pay the fiat pool too

Conversion - The conversion rate between the tokens and the underlying FIAT pool



Milestone 1 — Implement Base Substrate Modules  — 1 month 
Objectives
Create substrate blockchain extending existing code base for WebRTC Item with:

Transaction engine integration .

The release of crowdfunded tokens backed by a FIAT pool and out to ERC20 token.

Creation of Traits e.g. challenges, causes and the structure of the collective (ambassadors vote in other ambassadors and scientists)


Logical Model
As an overview of the logical model we will be using please examine the following diagram, which illustrates the existing Livetree transaction engine.



App User Interface:

The following mock up shows a video that has been uploaded.  Livetree already has this functionality in the form of story uploader: https://www.livetree.com/story-locator/story and the royalties / transaction engine that sits behind this video.


The video is uploaded. 

Please note a video is called an Item for future reference and maps directly to WebRTCFundableItem



The item, viewing statistics, advertising statistics, direct contributions are all pointers to IPFS hashes which are stored on Substrate. Again Livetree has the back-end infrastructure to support the Substrate.
The item has an owner, which is mapped to a substrate wallet.
The item can also have a designated Collective to donate its tokens to.
The item can be backed by tokens i.e. a viewer can back a video.


 

Code:

Key Structs / Modules

Description

Runtime / CRATE

1	
WebRTCFundableItem
Seed


The Collective would vote on funds raised to be cashed against a designated Fiat pool

RUST Module already coded in Github and to be extended

2	
FactoryCreate (smartContractType, version) 


Account and balance management: we have started building this functionality and imagine that it will use the following pallets: Balances, System, Timestamp Indices 

Executive manager that provides

 1) balances functions, for example when the collective have passed a motion it will release the fiat pool Collective 

 2) Ability to version and swap in / out functionality built in smart contract.  

1 x Runtime

 


Milestone 2 — Implement Substrate Front-End Apps   — 1 month 
Objectives

In this sprint we will be focusing on the front-end development (which we envisage will work on a series of React, Native, and administration and dashboard tools). 

The second large objective would be to make the Substrate module performant and scalable by implementing our hosting plans. For example, we currently use Azure, Cloudflare Workers, Fastly Terrarium, Agora, Thrive etc. It would involve determining pricing to utilise providers to distribute the WebAssembly. 

We will continually deliver a working module(s) along with a simple tutorial that explains how a user can (for example) spin up one of our Substrate nodes. Once the node is up, it will be possible to send test transactions and use the front end to show how the new functionality works. The code will have proper unit-test coverage to ensure functionality and robustness.



Milestone 3 — WebRTC Substrate Community Marketplace Extensions — 1 month 
Objectives
The major objective for this month would be to facilitate ‘Marketplace’ for community extensions that utilise the substrate audit of royalty pools, advertising pools or funding mechanisms. These community extensions could be for:

Bidding systems advertising / programmatic or other reward mechanisms for user metadata associated with the WebRTC fundable item.

A loyalty marketplace to access product discounts, reward systems (think air-miles) or purchases.  

A conversion between Substrate and Ethereum ERC20 tokens.  For illustration purposes please see:
https://cg06jb.axshare.com/#id=hv8uq6&p=convertsedtorsed&g=1
