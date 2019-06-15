## Susy-Sophon [v2.3.0](https://octonion.institute/susytech/susy-sophon/releases/tag/v2.3.0) (2019-01-16)

Susy-Sophon 2.3.0-beta is a consensus-relevant security release that reverts Constantinople on the Sophon network. Upgrading is mandatory for Sophon, and strongly recommended for other networks.

- **Consensus** - Sophon Network: Pull Constantinople protocol upgrade on Sophon (#10189)
  - Read more: [Security Alert: Sophon Constantinople Postponement](https://blog.superstring.io/2019/01/15/security-alert-sophon-constantinople-postponement/)
- **Networking** - All networks: Ping nodes from discovery (#10167)
- **Wasm** - Kovan Network: Update swasm-utils to 0.6.1 (#10134)

Other notable changes:

- Existing blocks in the database are now kept when restoring a Snapshot. (#8643)
- Block and transaction propagation is improved significantly. (#9954)
- The SRC-191 Signed Data Standard is now supported by `personal_sign191`. (#9701)
- Add support for SRC-191/712 `sof_signTypedData` as a standard for machine-verifiable and human-readable typed data signing with Sophon keys. (#9631)
- Add support for SRC-1186 `sof_getProof` (#9001)
- Add experimental RPCs flag to enable SRC-191, SRC-712, and SRC-1186 APIs via `--susy-jsonrpc-experimental` (#9928)
- Make `CALLCODE` to trace value to be the code address. (#9881)

Configuration changes:

- The SIP-98 transition is now disabled by default. If you previously had no `sip98transition` specified in your chain specification, you would enable this now manually on block `0x0`. (#9955)
- Also, unknown fields in chain specs are now rejected. (#9972)
- The Tendermint engine was removed from Susy Sophon and is no longer available and maintained. (#9980)
- Ropsten testnet data and keys moved from `test/` to `ropsten/` subdir. To reuse your old keys and data either copy or symlink them to the new location.  (#10123)
- Strict empty steps validation (#10041)
  - If you have a chain with`empty_steps` already running, some blocks most likely contain non-strict entries (unordered or duplicated empty steps). In this release `strict_empty_steps_transition` is enabled by default at block `0x0` for any chain with `empty_steps`.
  - If your network uses `empty_steps` you **must** (A) plan a hard fork and change `strict_empty_steps_transition` to the desired fork block and (B) update the clients of the whole network to 2.2.7-stable / 2.3.0-beta. If for some reason you don't want to do this please set`strict_empty_steps_transition` to `0xfffffffff` to disable it.

_Note:_ This release marks Susy 2.3 as _beta_. All versions of Susy 2.2 are now considered _stable_.

The full list of included changes:

- Backports for 2.3.0 beta ([#10164](https://octonion.institute/susytech/susy-sophon/pull/10164))
- Snap: fix path in script ([#10157](https://octonion.institute/susytech/susy-sophon/pull/10157))
- Make sure parent block is not in importing queue when importing ancient blocks ([#10138](https://octonion.institute/susytech/susy-sophon/pull/10138))
- Ci: re-enable snap publishing ([#10142](https://octonion.institute/susytech/susy-sophon/pull/10142))
- Hf in POA Core (2019-01-18) - Constantinople ([#10155](https://octonion.institute/susytech/susy-sophon/pull/10155))
- Update EWF's tobalaba chainspec ([#10152](https://octonion.institute/susytech/susy-sophon/pull/10152))
- Replace sofcore-logger with env-logger. ([#10102](https://octonion.institute/susytech/susy-sophon/pull/10102))
- Finality: dont require chain head to be in the chain ([#10054](https://octonion.institute/susytech/susy-sophon/pull/10054))
- Remove caching for node connections ([#10143](https://octonion.institute/susytech/susy-sophon/pull/10143))
- Blooms file iterator empty on out of range position. ([#10145](https://octonion.institute/susytech/susy-sophon/pull/10145))
- Autogen docs for the "Configuring Susy Sophon" wiki page. ([#10067](https://octonion.institute/susytech/susy-sophon/pull/10067))
- Misc: bump license header to 2019 ([#10135](https://octonion.institute/susytech/susy-sophon/pull/10135))
- Hide most of the logs from cpp example. ([#10139](https://octonion.institute/susytech/susy-sophon/pull/10139))
- Don't try to send oversized packets ([#10042](https://octonion.institute/susytech/susy-sophon/pull/10042))
- Private tx enabled flag added into STATUS packet ([#9999](https://octonion.institute/susytech/susy-sophon/pull/9999))
- Update swasm-utils to 0.6.1 ([#10134](https://octonion.institute/susytech/susy-sophon/pull/10134))
- Extract blockchain from sofcore ([#10114](https://octonion.institute/susytech/susy-sophon/pull/10114))
- Sofcore: update hardcoded headers ([#10123](https://octonion.institute/susytech/susy-sophon/pull/10123))
- Identity fix ([#10128](https://octonion.institute/susytech/susy-sophon/pull/10128))
- Use LenCachingMutex to optimize verification. ([#10117](https://octonion.institute/susytech/susy-sophon/pull/10117))
- Pysophon keystore support ([#9710](https://octonion.institute/susytech/susy-sophon/pull/9710))
- Bump rocksdb-sys to 0.5.5 ([#10124](https://octonion.institute/susytech/susy-sophon/pull/10124))
- Susy-clib: `async C bindings to RPC requests` + `subscribe/unsubscribe to websocket events` ([#9920](https://octonion.institute/susytech/susy-sophon/pull/9920))
- Refactor (hardware wallet) : reduce the number of threads ([#9644](https://octonion.institute/susytech/susy-sophon/pull/9644))
- Hf in POA Sokol (2019-01-04) ([#10077](https://octonion.institute/susytech/susy-sophon/pull/10077))
- Fix broken links ([#10119](https://octonion.institute/susytech/susy-sophon/pull/10119))
- Follow-up to [#10105](https://octonion.institute/susytech/susy-sophon/issues/10105) ([#10107](https://octonion.institute/susytech/susy-sophon/pull/10107))
- Move SIP-712 crate back to susy-sophon ([#10106](https://octonion.institute/susytech/susy-sophon/pull/10106))
- Move a bunch of stuff around ([#10101](https://octonion.institute/susytech/susy-sophon/pull/10101))
- Revert "Add --frozen when running cargo ([#10081](https://octonion.institute/susytech/susy-sophon/pull/10081))" ([#10105](https://octonion.institute/susytech/susy-sophon/pull/10105))
- Fix left over small grumbles on whitespaces ([#10084](https://octonion.institute/susytech/susy-sophon/pull/10084))
- Add --frozen when running cargo ([#10081](https://octonion.institute/susytech/susy-sophon/pull/10081))
- Fix pubsub new_blocks notifications to include all blocks ([#9987](https://octonion.institute/susytech/susy-sophon/pull/9987))
- Update some dependencies for compilation with pc-windows-gnu ([#10082](https://octonion.institute/susytech/susy-sophon/pull/10082))
- Fill transaction hash on sofGetLog of light client. ([#9938](https://octonion.institute/susytech/susy-sophon/pull/9938))
- Update changelog update for 2.2.5-beta and 2.1.10-stable ([#10064](https://octonion.institute/susytech/susy-sophon/pull/10064))
- Implement len caching for parking_lot RwLock ([#10032](https://octonion.institute/susytech/susy-sophon/pull/10032))
- Update parking_lot to 0.7 ([#10050](https://octonion.institute/susytech/susy-sophon/pull/10050))
- Bump crossbeam. ([#10048](https://octonion.institute/susytech/susy-sophon/pull/10048))
- Sofcore: enable constantinople on sophon ([#10031](https://octonion.institute/susytech/susy-sophon/pull/10031))
- Strict empty steps validation ([#10041](https://octonion.institute/susytech/susy-sophon/pull/10041))
- Center the Subtitle, use some CAPS ([#10034](https://octonion.institute/susytech/susy-sophon/pull/10034))
- Change test miner max memory to malloc reports. ([#10024](https://octonion.institute/susytech/susy-sophon/pull/10024))
- Sort the storage for private state ([#10018](https://octonion.institute/susytech/susy-sophon/pull/10018))
- Fix: test corpus_inaccessible panic ([#10019](https://octonion.institute/susytech/susy-sophon/pull/10019))
- Ci: move future releases to sophon subdir on s3 ([#10017](https://octonion.institute/susytech/susy-sophon/pull/10017))
- Light(on_demand): decrease default time window to 10 secs ([#10016](https://octonion.institute/susytech/susy-sophon/pull/10016))
- Light client : failsafe crate (circuit breaker) ([#9790](https://octonion.institute/susytech/susy-sophon/pull/9790))
- Lencachingmutex ([#9988](https://octonion.institute/susytech/susy-sophon/pull/9988))
- Version and notification for private contract wrapper added ([#9761](https://octonion.institute/susytech/susy-sophon/pull/9761))
- Handle failing case for update account cache in require ([#9989](https://octonion.institute/susytech/susy-sophon/pull/9989))
- Add tokio runtime to sofcore io worker ([#9979](https://octonion.institute/susytech/susy-sophon/pull/9979))
- Move daemonize before creating account provider ([#10003](https://octonion.institute/susytech/susy-sophon/pull/10003))
- Docs: update changelogs ([#9990](https://octonion.institute/susytech/susy-sophon/pull/9990))
- Fix daemonize ([#10000](https://octonion.institute/susytech/susy-sophon/pull/10000))
- Fix Bloom migration ([#9992](https://octonion.institute/susytech/susy-sophon/pull/9992))
- Remove tendermint engine support ([#9980](https://octonion.institute/susytech/susy-sophon/pull/9980))
- Calculate gas for deployment transaction ([#9840](https://octonion.institute/susytech/susy-sophon/pull/9840))
- Fix unstable peers and slowness in sync ([#9967](https://octonion.institute/susytech/susy-sophon/pull/9967))
- Adds susy_verifySignature RPC method ([#9507](https://octonion.institute/susytech/susy-sophon/pull/9507))
- Improve block and transaction propagation ([#9954](https://octonion.institute/susytech/susy-sophon/pull/9954))
- Deny unknown fields for chainspec ([#9972](https://octonion.institute/susytech/susy-sophon/pull/9972))
- Fix docker build ([#9971](https://octonion.institute/susytech/susy-sophon/pull/9971))
- Ci: rearrange pipeline by logic ([#9970](https://octonion.institute/susytech/susy-sophon/pull/9970))
- Add changelogs for 2.0.9, 2.1.4, 2.1.6, and 2.2.1 ([#9963](https://octonion.institute/susytech/susy-sophon/pull/9963))
- Add Error message when sync is still in progress. ([#9475](https://octonion.institute/susytech/susy-sophon/pull/9475))
- Make CALLCODE to trace value to be the code address ([#9881](https://octonion.institute/susytech/susy-sophon/pull/9881))
- Fix light client informant while syncing ([#9932](https://octonion.institute/susytech/susy-sophon/pull/9932))
- Add a optional json dump state to svm-bin ([#9706](https://octonion.institute/susytech/susy-sophon/pull/9706))
- Disable SIP-98 transition by default ([#9955](https://octonion.institute/susytech/susy-sophon/pull/9955))
- Remove secret_store runtimes. ([#9888](https://octonion.institute/susytech/susy-sophon/pull/9888))
- Fix a deadlock ([#9952](https://octonion.institute/susytech/susy-sophon/pull/9952))
- Chore(sip712): remove unused `failure-derive` ([#9958](https://octonion.institute/susytech/susy-sophon/pull/9958))
- Do not use the home directory as the working dir in docker ([#9834](https://octonion.institute/susytech/susy-sophon/pull/9834))
- Prevent silent errors in daemon mode, closes [#9367](https://octonion.institute/susytech/susy-sophon/issues/9367) ([#9946](https://octonion.institute/susytech/susy-sophon/pull/9946))
- Fix empty steps ([#9939](https://octonion.institute/susytech/susy-sophon/pull/9939))
- Adjust requests costs for light client ([#9925](https://octonion.institute/susytech/susy-sophon/pull/9925))
- Sip-1186: add `sof_getProof` RPC-Method ([#9001](https://octonion.institute/susytech/susy-sophon/pull/9001))
- Missing blocks in filter_changes RPC ([#9947](https://octonion.institute/susytech/susy-sophon/pull/9947))
- Allow rust-nightly builds fail in nightly builds ([#9944](https://octonion.institute/susytech/susy-sophon/pull/9944))
- Update sof-secp256k1 to include fix for BSDs ([#9935](https://octonion.institute/susytech/susy-sophon/pull/9935))
- Unbreak build on rust -stable ([#9934](https://octonion.institute/susytech/susy-sophon/pull/9934))
- Keep existing blocks when restoring a Snapshot ([#8643](https://octonion.institute/susytech/susy-sophon/pull/8643))
- Add experimental RPCs flag ([#9928](https://octonion.institute/susytech/susy-sophon/pull/9928))
- Clarify poll lifetime ([#9922](https://octonion.institute/susytech/susy-sophon/pull/9922))
- Docs(require rust 1.30) ([#9923](https://octonion.institute/susytech/susy-sophon/pull/9923))
- Use block header for building finality ([#9914](https://octonion.institute/susytech/susy-sophon/pull/9914))
- Simplify cargo audit ([#9918](https://octonion.institute/susytech/susy-sophon/pull/9918))
- Light-fetch: Differentiate between out-of-gas/manual throw and use required gas from response on failure ([#9824](https://octonion.institute/susytech/susy-sophon/pull/9824))
- Sip 191 ([#9701](https://octonion.institute/susytech/susy-sophon/pull/9701))
- Fix(logger): `reqwest` no longer a dependency ([#9908](https://octonion.institute/susytech/susy-sophon/pull/9908))
- Remove rust-toolchain file ([#9906](https://octonion.institute/susytech/susy-sophon/pull/9906))
- Foundation: 6692865, ropsten: 4417537, kovan: 9363457 ([#9907](https://octonion.institute/susytech/susy-sophon/pull/9907))
- Sofcore: use Machine::verify_transaction on parent block ([#9900](https://octonion.institute/susytech/susy-sophon/pull/9900))
- Chore(rpc-tests): remove unused rand ([#9896](https://octonion.institute/susytech/susy-sophon/pull/9896))
- Fix: Intermittent failing CI due to addr in use ([#9885](https://octonion.institute/susytech/susy-sophon/pull/9885))
- Chore(bump docopt): 0.8 -> 1.0 ([#9889](https://octonion.institute/susytech/susy-sophon/pull/9889))
- Use expect ([#9883](https://octonion.institute/susytech/susy-sophon/pull/9883))
- Use Weak reference in PubSubClient ([#9886](https://octonion.institute/susytech/susy-sophon/pull/9886))
- Ci: nuke the gitlab caches ([#9855](https://octonion.institute/susytech/susy-sophon/pull/9855))
- Remove unused code ([#9884](https://octonion.institute/susytech/susy-sophon/pull/9884))
- Fix json tracer overflow ([#9873](https://octonion.institute/susytech/susy-sophon/pull/9873))
- Allow to seal work on latest block ([#9876](https://octonion.institute/susytech/susy-sophon/pull/9876))
- Fix docker script ([#9854](https://octonion.institute/susytech/susy-sophon/pull/9854))
- Health endpoint ([#9847](https://octonion.institute/susytech/susy-sophon/pull/9847))
- Gitlab-ci: make android release build succeed ([#9743](https://octonion.institute/susytech/susy-sophon/pull/9743))
- Clean up existing benchmarks ([#9839](https://octonion.institute/susytech/susy-sophon/pull/9839))
- Update Callisto block reward code to support HF1 ([#9811](https://octonion.institute/susytech/susy-sophon/pull/9811))
- Option to disable keep alive for JSON-RPC http transport ([#9848](https://octonion.institute/susytech/susy-sophon/pull/9848))
- Classic.json Bootnode Update ([#9828](https://octonion.institute/susytech/susy-sophon/pull/9828))
- Support MIX. ([#9767](https://octonion.institute/susytech/susy-sophon/pull/9767))
- Ci: remove failing tests for android, windows, and macos ([#9788](https://octonion.institute/susytech/susy-sophon/pull/9788))
- Implement NoProof for json tests and update tests reference (replaces [#9744](https://octonion.institute/susytech/susy-sophon/issues/9744)) ([#9814](https://octonion.institute/susytech/susy-sophon/pull/9814))
- Chore(bump regex) ([#9842](https://octonion.institute/susytech/susy-sophon/pull/9842))
- Ignore global cache for patched accounts ([#9752](https://octonion.institute/susytech/susy-sophon/pull/9752))
- Move state root verification before gas used ([#9841](https://octonion.institute/susytech/susy-sophon/pull/9841))
- Fix(docker-aarch64) : cross-compile config ([#9798](https://octonion.institute/susytech/susy-sophon/pull/9798))
- Version: bump nightly to 2.3.0 ([#9819](https://octonion.institute/susytech/susy-sophon/pull/9819))
- Tests modification for windows CI ([#9671](https://octonion.institute/susytech/susy-sophon/pull/9671))
- Sip-712 implementation ([#9631](https://octonion.institute/susytech/susy-sophon/pull/9631))
- Fix typo ([#9826](https://octonion.institute/susytech/susy-sophon/pull/9826))
- Clean up serde rename and use rename_all = camelCase when possible ([#9823](https://octonion.institute/susytech/susy-sophon/pull/9823))

## Previous releases

- [CHANGELOG-2.2](docs/CHANGELOG-2.2.md) (_stable_)
- [CHANGELOG-2.1](docs/CHANGELOG-2.1.md) (EOL: 2019-01-16)
- [CHANGELOG-2.0](docs/CHANGELOG-2.0.md) (EOL: 2018-11-15)
- [CHANGELOG-1.11](docs/CHANGELOG-1.11.md) (EOL: 2018-09-19)
- [CHANGELOG-1.10](docs/CHANGELOG-1.10.md) (EOL: 2018-07-18)
- [CHANGELOG-1.9](docs/CHANGELOG-1.9.md) (EOL: 2018-05-09)
- [CHANGELOG-1.8](docs/CHANGELOG-1.8.md) (EOL: 2018-03-22)
- [CHANGELOG-1.7](docs/CHANGELOG-1.7.md) (EOL: 2018-01-25)
- [CHANGELOG-1.6](docs/CHANGELOG-1.6.md) (EOL: 2017-10-15)
- [CHANGELOG-1.5](docs/CHANGELOG-1.5.md) (EOL: 2017-07-28)
- [CHANGELOG-1.4](docs/CHANGELOG-1.4.md) (EOL: 2017-03-13)
- [CHANGELOG-1.3](docs/CHANGELOG-1.3.md) (EOL: 2017-01-19)
- [CHANGELOG-1.2](docs/CHANGELOG-1.2.md) (EOL: 2016-11-07)
- [CHANGELOG-1.1](docs/CHANGELOG-1.1.md) (EOL: 2016-08-12)
- [CHANGELOG-1.0](docs/CHANGELOG-1.0.md) (EOL: 2016-06-24)
- [CHANGELOG-0.9](docs/CHANGELOG-0.9.md) (EOL: 2016-05-02)