# BLS Vote Sigverify Benchmark Runs

## Run 1

- Date: 2026-03-25
- Branch: `zz/integrate_bls_cache`
- Command: `cargo bench --bench bls_vote_sigverify`
- Notes: `Gnuplot not found, using plotters backend`

### verify_single_signature

| Benchmark | Time |
| --- | --- |
| `verify_single_signature/1_item` | `797.83 µs 797.88 µs 797.95 µs` |
| `verify_single_signature_with_prepared_message/1_item` | `610.50 µs 610.57 µs 610.64 µs` |

### verify_votes_optimistic

| Benchmark | Time | Notes |
| --- | --- | --- |
| `verify_votes_optimistic/msgs_1/batch_8` | `825.95 µs 827.56 µs 829.20 µs` |  |
| `verify_votes_optimistic/msgs_2/batch_8` | `1.0503 ms 1.0526 ms 1.0550 ms` | Warning: increase target time to `5.5s`, enable flat sampling, or reduce sample count to `60` |
| `verify_votes_optimistic/msgs_4/batch_8` | `1.4354 ms 1.4364 ms 1.4373 ms` | Warning: increase target time to `7.3s`, enable flat sampling, or reduce sample count to `50` |
| `verify_votes_optimistic/msgs_8/batch_8` | `2.2231 ms 2.2241 ms 2.2251 ms` |  |
| `verify_votes_optimistic/msgs_1/batch_16` | `875.87 µs 877.66 µs 879.48 µs` |  |
| `verify_votes_optimistic/msgs_2/batch_16` | `1.1408 ms 1.1437 ms 1.1470 ms` | Warning: increase target time to `5.8s`, enable flat sampling, or reduce sample count to `60` |
| `verify_votes_optimistic/msgs_4/batch_16` | `1.4868 ms 1.4876 ms 1.4884 ms` | Warning: increase target time to `7.5s`, enable flat sampling, or reduce sample count to `50` |
| `verify_votes_optimistic/msgs_8/batch_16` | `2.2885 ms 2.2902 ms 2.2919 ms` |  |
| `verify_votes_optimistic/msgs_16/batch_16` | `3.9043 ms 3.9063 ms 3.9082 ms` |  |
| `verify_votes_optimistic/msgs_1/batch_32` | `1.0277 ms 1.0295 ms 1.0313 ms` | Warning: increase target time to `5.2s`, enable flat sampling, or reduce sample count to `60` |
| `verify_votes_optimistic/msgs_2/batch_32` | `1.2698 ms 1.2704 ms 1.2710 ms` | Warning: increase target time to `6.4s`, enable flat sampling, or reduce sample count to `60` |
| `verify_votes_optimistic/msgs_4/batch_32` | `1.6622 ms 1.6637 ms 1.6652 ms` | Warning: increase target time to `8.4s`, enable flat sampling, or reduce sample count to `50` |
| `verify_votes_optimistic/msgs_8/batch_32` | `2.4737 ms 2.4751 ms 2.4768 ms` |  |
| `verify_votes_optimistic/msgs_16/batch_32` | `4.0987 ms 4.0999 ms 4.1011 ms` |  |
| `verify_votes_optimistic/msgs_1/batch_64` | `1.3458 ms 1.3462 ms 1.3466 ms` | Warning: increase target time to `6.8s`, enable flat sampling, or reduce sample count to `60` |
| `verify_votes_optimistic/msgs_2/batch_64` | `1.5990 ms 1.5993 ms 1.5996 ms` | Warning: increase target time to `8.1s`, enable flat sampling, or reduce sample count to `50` |
| `verify_votes_optimistic/msgs_4/batch_64` | `1.9989 ms 2.0012 ms 2.0037 ms` |  |
| `verify_votes_optimistic/msgs_8/batch_64` | `2.8151 ms 2.8170 ms 2.8188 ms` |  |
| `verify_votes_optimistic/msgs_16/batch_64` | `4.4357 ms 4.4371 ms 4.4384 ms` |  |
| `verify_votes_optimistic/msgs_1/batch_128` | `2.0016 ms 2.0023 ms 2.0029 ms` |  |
| `verify_votes_optimistic/msgs_2/batch_128` | `2.2526 ms 2.2533 ms 2.2540 ms` |  |
| `verify_votes_optimistic/msgs_4/batch_128` | `2.6301 ms 2.6305 ms 2.6310 ms` |  |
| `verify_votes_optimistic/msgs_8/batch_128` | `3.5222 ms 3.5263 ms 3.5305 ms` |  |
| `verify_votes_optimistic/msgs_16/batch_128` | `5.0711 ms 5.0739 ms 5.0767 ms` |  |

### aggregate_pubkeys

| Benchmark | Time |
| --- | --- |
| `aggregate_pubkeys/msgs_1/batch_8` | `241.45 µs 241.77 µs 242.05 µs` |
| `aggregate_pubkeys/msgs_2/batch_8` | `249.84 µs 250.05 µs 250.28 µs` |
| `aggregate_pubkeys/msgs_4/batch_8` | `277.03 µs 277.91 µs 278.80 µs` |
| `aggregate_pubkeys/msgs_8/batch_8` | `264.20 µs 267.10 µs 270.21 µs` |
| `aggregate_pubkeys/msgs_1/batch_16` | `285.90 µs 286.41 µs 287.00 µs` |
| `aggregate_pubkeys/msgs_2/batch_16` | `290.03 µs 291.41 µs 293.41 µs` |
| `aggregate_pubkeys/msgs_4/batch_16` | `319.58 µs 321.90 µs 324.41 µs` |
| `aggregate_pubkeys/msgs_8/batch_16` | `388.21 µs 390.99 µs 393.96 µs` |
| `aggregate_pubkeys/msgs_16/batch_16` | `397.26 µs 400.72 µs 404.12 µs` |
| `aggregate_pubkeys/msgs_1/batch_32` | `359.65 µs 360.70 µs 361.78 µs` |
| `aggregate_pubkeys/msgs_2/batch_32` | `365.81 µs 366.91 µs 367.90 µs` |
| `aggregate_pubkeys/msgs_4/batch_32` | `392.54 µs 395.52 µs 398.91 µs` |
| `aggregate_pubkeys/msgs_8/batch_32` | `465.40 µs 467.97 µs 470.92 µs` |
| `aggregate_pubkeys/msgs_16/batch_32` | `567.48 µs 570.05 µs 572.75 µs` |
| `aggregate_pubkeys/msgs_1/batch_64` | `415.74 µs 416.92 µs 418.12 µs` |
| `aggregate_pubkeys/msgs_2/batch_64` | `411.98 µs 413.15 µs 414.36 µs` |
| `aggregate_pubkeys/msgs_4/batch_64` | `440.20 µs 443.30 µs 446.73 µs` |
| `aggregate_pubkeys/msgs_8/batch_64` | `522.79 µs 526.71 µs 530.84 µs` |
| `aggregate_pubkeys/msgs_16/batch_64` | `636.92 µs 639.50 µs 642.29 µs` |
| `aggregate_pubkeys/msgs_1/batch_128` | `404.26 µs 405.92 µs 407.47 µs` |
| `aggregate_pubkeys/msgs_2/batch_128` | `390.54 µs 391.83 µs 393.13 µs` |
| `aggregate_pubkeys/msgs_4/batch_128` | `416.77 µs 418.63 µs 420.41 µs` |
| `aggregate_pubkeys/msgs_8/batch_128` | `501.09 µs 504.29 µs 507.48 µs` |
| `aggregate_pubkeys/msgs_16/batch_128` | `647.56 µs 650.12 µs 652.70 µs` |

### aggregate_signatures

| Benchmark | Time |
| --- | --- |
| `aggregate_signatures/batch_8` | `95.232 µs 95.615 µs 96.027 µs` |
| `aggregate_signatures/batch_16` | `145.53 µs 145.75 µs 145.98 µs` |
| `aggregate_signatures/batch_32` | `208.95 µs 209.27 µs 209.61 µs` |
| `aggregate_signatures/batch_64` | `286.41 µs 286.89 µs 287.42 µs` |
| `aggregate_signatures/batch_128` | `430.42 µs 430.92 µs 431.43 µs` |

### verify_votes_fallback

| Benchmark | Time | Notes |
| --- | --- | --- |
| `verify_votes_fallback/batch_8` | `1.4608 ms 1.4614 ms 1.4622 ms` | Warning: increase target time to `7.4s`, enable flat sampling, or reduce sample count to `50` |
| `verify_votes_fallback/batch_16` | `2.7210 ms 2.7221 ms 2.7238 ms` |  |
| `verify_votes_fallback/batch_32` | `5.2178 ms 5.2181 ms 5.2185 ms` |  |
| `verify_votes_fallback/batch_64` | `10.232 ms 10.233 ms 10.234 ms` |  |
| `verify_votes_fallback/batch_128` | `20.491 ms 20.501 ms 20.511 ms` |  |

## Run 2

- Date: 2026-03-25
- Branch: `main`
- Command: `cargo bench --bench bls_vote_sigverify`
- Build: `Finished bench profile [optimized] target(s) in 40.91s`
- Notes: `Gnuplot not found, using plotters backend`

### verify_single_signature

| Benchmark | Time | Change vs previous baseline |
| --- | --- | --- |
| `verify_single_signature/1_item` | `801.59 µs 803.59 µs 805.98 µs` | `+0.6746% +0.8842% +1.0946%` |

### verify_votes_optimistic

| Benchmark | Time | Change vs previous baseline | Notes |
| --- | --- | --- | --- |
| `verify_votes_optimistic/msgs_1/batch_8` | `881.02 µs 881.28 µs 881.57 µs` | `+6.1960% +6.4038% +6.6126%` | Performance regressed |
| `verify_votes_optimistic/msgs_2/batch_8` | `1.0340 ms 1.0350 ms 1.0366 ms` | `-1.5684% -1.2865% -0.9755%` | Warning: increase target time to `5.3s`, change within noise threshold |
| `verify_votes_optimistic/msgs_4/batch_8` | `1.3128 ms 1.3130 ms 1.3133 ms` | `-8.6574% -8.5661% -8.4730%` | Warning: increase target time to `6.6s`, performance improved |
| `verify_votes_optimistic/msgs_8/batch_8` | `2.0529 ms 2.0534 ms 2.0539 ms` | `-7.7236% -7.6760% -7.6301%` | Performance improved |
| `verify_votes_optimistic/msgs_1/batch_16` | `969.73 µs 969.89 µs 970.05 µs` | `+10.144% +10.379% +10.602%` | Performance regressed |
| `verify_votes_optimistic/msgs_2/batch_16` | `1.1281 ms 1.1288 ms 1.1295 ms` | `-1.9329% -1.6811% -1.4329%` | Warning: increase target time to `5.7s`, performance improved |
| `verify_votes_optimistic/msgs_4/batch_16` | `1.4048 ms 1.4063 ms 1.4079 ms` | `-5.5914% -5.4594% -5.3070%` | Warning: increase target time to `7.1s`, performance improved |
| `verify_votes_optimistic/msgs_8/batch_16` | `2.1588 ms 2.1596 ms 2.1605 ms` | `-5.7798% -5.6992% -5.6197%` | Performance improved |
| `verify_votes_optimistic/msgs_16/batch_16` | `3.6624 ms 3.6635 ms 3.6646 ms` | `-6.2674% -6.2144% -6.1591%` | Performance improved |
| `verify_votes_optimistic/msgs_1/batch_32` | `1.1416 ms 1.1419 ms 1.1422 ms` | `+11.241% +11.388% +11.524%` | Warning: increase target time to `5.8s`, performance regressed |
| `verify_votes_optimistic/msgs_2/batch_32` | `1.3054 ms 1.3060 ms 1.3066 ms` | `+2.7772% +2.8620% +2.9464%` | Warning: increase target time to `6.6s`, performance regressed |
| `verify_votes_optimistic/msgs_4/batch_32` | `1.5896 ms 1.5900 ms 1.5904 ms` | `-4.5837% -4.4612% -4.3457%` | Warning: increase target time to `8.0s`, performance improved |
| `verify_votes_optimistic/msgs_8/batch_32` | `2.3347 ms 2.3351 ms 2.3356 ms` | `-5.7197% -5.6525% -5.5982%` | Performance improved |
| `verify_votes_optimistic/msgs_16/batch_32` | `3.8200 ms 3.8222 ms 3.8244 ms` | `-6.8343% -6.7735% -6.7168%` | Performance improved |
| `verify_votes_optimistic/msgs_1/batch_64` | `1.4925 ms 1.4929 ms 1.4934 ms` | `+10.953% +11.046% +11.203%` | Warning: increase target time to `7.5s`, performance regressed |
| `verify_votes_optimistic/msgs_2/batch_64` | `1.6465 ms 1.6469 ms 1.6473 ms` | `+2.9390% +2.9797% +3.0215%` | Warning: increase target time to `8.3s`, performance regressed |
| `verify_votes_optimistic/msgs_4/batch_64` | `1.9226 ms 1.9233 ms 1.9240 ms` | `-3.9960% -3.8746% -3.7542%` | Warning: increase target time to `9.7s`, performance improved |
| `verify_votes_optimistic/msgs_8/batch_64` | `2.6613 ms 2.6620 ms 2.6627 ms` | `-5.5691% -5.5016% -5.4347%` | Performance improved |
| `verify_votes_optimistic/msgs_16/batch_64` | `4.1579 ms 4.1619 ms 4.1685 ms` | `-6.3044% -6.2010% -6.0442%` | Performance improved |
| `verify_votes_optimistic/msgs_1/batch_128` | `2.1473 ms 2.1484 ms 2.1500 ms` | `+7.2294% +7.2994% +7.3859%` | Performance regressed |
| `verify_votes_optimistic/msgs_2/batch_128` | `2.2976 ms 2.2983 ms 2.2991 ms` | `+1.9502% +1.9985% +2.0435%` | Performance regressed |
| `verify_votes_optimistic/msgs_4/batch_128` | `2.5814 ms 2.5827 ms 2.5844 ms` | `-1.8658% -1.8182% -1.7643%` | Performance improved |
| `verify_votes_optimistic/msgs_8/batch_128` | `3.3340 ms 3.3345 ms 3.3351 ms` | `-5.5513% -5.4385% -5.3250%` | Performance improved |
| `verify_votes_optimistic/msgs_16/batch_128` | `4.8280 ms 4.8290 ms 4.8301 ms` | `-4.8835% -4.8269% -4.7715%` | Performance improved |

### aggregate_pubkeys

| Benchmark | Time | Change vs previous baseline | Notes |
| --- | --- | --- | --- |
| `aggregate_pubkeys/msgs_1/batch_8` | `54.507 µs 54.626 µs 54.732 µs` | `-77.561% -77.509% -77.463%` | Performance improved |
| `aggregate_pubkeys/msgs_2/batch_8` | `54.849 µs 54.988 µs 55.116 µs` | `-78.119% -78.075% -78.025%` | Performance improved |
| `aggregate_pubkeys/msgs_4/batch_8` | `54.890 µs 54.988 µs 55.092 µs` | `-80.399% -80.293% -80.189%` | Performance improved |
| `aggregate_pubkeys/msgs_8/batch_8` | `58.038 µs 58.202 µs 58.356 µs` | `-78.672% -78.425% -78.186%` | Performance improved |
| `aggregate_pubkeys/msgs_1/batch_16` | `96.591 µs 96.754 µs 96.925 µs` | `-66.712% -66.618% -66.524%` | Performance improved |
| `aggregate_pubkeys/msgs_2/batch_16` | `97.056 µs 97.237 µs 97.427 µs` | `-66.874% -66.645% -66.471%` | Performance improved |
| `aggregate_pubkeys/msgs_4/batch_16` | `97.195 µs 97.379 µs 97.562 µs` | `-69.872% -69.659% -69.439%` | Performance improved |
| `aggregate_pubkeys/msgs_8/batch_16` | `99.942 µs 100.09 µs 100.24 µs` | `-74.718% -74.475% -74.227%` | Performance improved |
| `aggregate_pubkeys/msgs_16/batch_16` | `107.93 µs 108.22 µs 108.50 µs` | `-73.116% -72.910% -72.699%` | Performance improved |
| `aggregate_pubkeys/msgs_1/batch_32` | `162.75 µs 163.28 µs 163.79 µs` | `-54.822% -54.661% -54.487%` | Performance improved |
| `aggregate_pubkeys/msgs_2/batch_32` | `164.69 µs 165.07 µs 165.45 µs` | `-54.885% -54.714% -54.536%` | Performance improved |
| `aggregate_pubkeys/msgs_4/batch_32` | `165.29 µs 165.76 µs 166.19 µs` | `-58.670% -58.243% -57.837%` | Performance improved |
| `aggregate_pubkeys/msgs_8/batch_32` | `168.71 µs 169.20 µs 169.69 µs` | `-64.618% -64.343% -64.067%` | Performance improved |
| `aggregate_pubkeys/msgs_16/batch_32` | `174.58 µs 175.01 µs 175.44 µs` | `-69.524% -69.364% -69.191%` | Performance improved |
| `aggregate_pubkeys/msgs_1/batch_64` | `212.35 µs 213.19 µs 213.99 µs` | `-49.142% -48.841% -48.529%` | Performance improved |
| `aggregate_pubkeys/msgs_2/batch_64` | `216.61 µs 217.42 µs 218.29 µs` | `-47.683% -47.351% -47.004%` | Performance improved |
| `aggregate_pubkeys/msgs_4/batch_64` | `216.65 µs 217.51 µs 218.38 µs` | `-51.604% -51.146% -50.715%` | Performance improved |
| `aggregate_pubkeys/msgs_8/batch_64` | `223.26 µs 224.07 µs 224.88 µs` | `-58.046% -57.718% -57.391%` | Performance improved |
| `aggregate_pubkeys/msgs_16/batch_64` | `228.55 µs 229.47 µs 230.41 µs` | `-64.468% -64.230% -63.998%` | Performance improved |
| `aggregate_pubkeys/msgs_1/batch_128` | `195.37 µs 196.27 µs 197.13 µs` | `-52.150% -51.804% -51.431%` | Performance improved |
| `aggregate_pubkeys/msgs_2/batch_128` | `198.61 µs 199.69 µs 200.75 µs` | `-49.180% -48.814% -48.394%` | Performance improved |
| `aggregate_pubkeys/msgs_4/batch_128` | `202.49 µs 203.68 µs 204.86 µs` | `-51.454% -51.017% -50.485%` | Performance improved |
| `aggregate_pubkeys/msgs_8/batch_128` | `205.59 µs 206.61 µs 207.65 µs` | `-58.971% -58.556% -58.120%` | Performance improved |
| `aggregate_pubkeys/msgs_16/batch_128` | `216.35 µs 217.50 µs 218.64 µs` | `-67.109% -66.825% -66.551%` | Performance improved |

### aggregate_signatures

| Benchmark | Time | Change vs previous baseline | Notes |
| --- | --- | --- | --- |
| `aggregate_signatures/batch_8` | `94.946 µs 95.669 µs 96.442 µs` | `-1.8227% -1.1087% -0.3848%` | Change within noise threshold |
| `aggregate_signatures/batch_16` | `146.58 µs 147.01 µs 147.44 µs` | `+0.4531% +0.7435% +1.0353%` | Change within noise threshold |
| `aggregate_signatures/batch_32` | `210.90 µs 211.27 µs 211.65 µs` | `+0.0255% +0.3444% +0.6364%` | Change within noise threshold |
| `aggregate_signatures/batch_64` | `288.06 µs 288.63 µs 289.24 µs` | `+0.2924% +0.5570% +0.8079%` | Change within noise threshold |
| `aggregate_signatures/batch_128` | `433.27 µs 433.82 µs 434.43 µs` | `+0.6319% +0.8137% +1.0157%` | Change within noise threshold |

### verify_votes_fallback

| Benchmark | Time | Change vs previous baseline | Notes |
| --- | --- | --- | --- |
| `verify_votes_fallback/batch_8` | `1.6425 ms 1.6426 ms 1.6428 ms` | `+12.375% +12.517% +12.750%` | Warning: increase target time to `8.3s`, performance regressed |
| `verify_votes_fallback/batch_16` | `3.2851 ms 3.2877 ms 3.2918 ms` | `+20.655% +20.781% +20.947%` | Performance regressed |
| `verify_votes_fallback/batch_32` | `6.5355 ms 6.5360 ms 6.5366 ms` | `+25.241% +25.256% +25.270%` | Performance regressed |
| `verify_votes_fallback/batch_64` | `13.033 ms 13.035 ms 13.037 ms` | `+27.356% +27.378% +27.403%` | Performance regressed |
| `verify_votes_fallback/batch_128` | `26.017 ms 26.019 ms 26.022 ms` | `+26.853% +26.915% +26.978%` | Performance regressed |
