diff --git a/frontend/package.json b/frontend/package.json
index 972d1bb..0089285 100644
--- a/frontend/package.json
+++ b/frontend/package.json
@@ -14,6 +14,7 @@
   },
   "dependencies": {
     "@radix-ui/react-slot": "^1.2.4",
+    "@tailwindcss/typography": "^0.5.19",
     "@tailwindcss/vite": "^4.1.18",
     "@tanstack/react-query": "^5.90.20",
     "@tanstack/react-router": "^1.158.1",
@@ -29,6 +30,8 @@
     "react": "^19.2.0",
     "react-dom": "^19.2.0",
     "react-i18next": "^16.5.4",
+    "react-markdown": "^10.1.0",
+    "remark-gfm": "^4.0.1",
     "tailwind-merge": "^3.4.0",
     "tailwindcss": "^4.1.18"
   },
diff --git a/frontend/pnpm-lock.yaml b/frontend/pnpm-lock.yaml
index c7f11cb..91be1cf 100644
--- a/frontend/pnpm-lock.yaml
+++ b/frontend/pnpm-lock.yaml
@@ -11,6 +11,9 @@ importers:
       '@radix-ui/react-slot':
         specifier: ^1.2.4
         version: 1.2.4(@types/react@19.2.11)(react@19.2.4)
+      '@tailwindcss/typography':
+        specifier: ^0.5.19
+        version: 0.5.19(tailwindcss@4.1.18)
       '@tailwindcss/vite':
         specifier: ^4.1.18
         version: 4.1.18(vite@7.3.1(@types/node@24.10.10)(jiti@2.6.1)(lightningcss@1.30.2)(tsx@4.21.0))
@@ -56,6 +59,12 @@ importers:
       react-i18next:
         specifier: ^16.5.4
         version: 16.5.4(i18next@25.8.3(typescript@5.9.3))(react-dom@19.2.4(react@19.2.4))(react@19.2.4)(typescript@5.9.3)
+      react-markdown:
+        specifier: ^10.1.0
+        version: 10.1.0(@types/react@19.2.11)(react@19.2.4)
+      remark-gfm:
+        specifier: ^4.0.1
+        version: 4.0.1
       tailwind-merge:
         specifier: ^3.4.0
         version: 3.4.0
@@ -1450,6 +1459,11 @@ packages:
     resolution: {integrity: sha512-EgCR5tTS5bUSKQgzeMClT6iCY3ToqE1y+ZB0AKldj809QXk1Y+3jB0upOYZrn9aGIzPtUsP7sX4QQ4XtjBB95A==}
     engines: {node: '>= 10'}
 
+  '@tailwindcss/typography@0.5.19':
+    resolution: {integrity: sha512-w31dd8HOx3k9vPtcQh5QHP9GwKcgbMp87j58qi6xgiBnFFtKEAgCWnDw4qUT8aHwkCp8bKvb/KGKWWHedP0AAg==}
+    peerDependencies:
+      tailwindcss: '>=3.0.0 || insiders || >=4.0.0-alpha.20 || >=4.0.0-beta.1'
+
   '@tailwindcss/vite@4.1.18':
     resolution: {integrity: sha512-jVA+/UpKL1vRLg6Hkao5jldawNmRo7mQYrZtNHMIVpLfLhDml5nMRUo/8MwoX2vNXvnaXNNMedrMfMugAVX1nA==}
     peerDependencies:
@@ -1694,18 +1708,33 @@ packages:
   '@types/d3@7.4.3':
     resolution: {integrity: sha512-lZXZ9ckh5R8uiFVt8ogUNf+pIrK4EsWrx2Np75WvF/eTpJ0FMHNhjXk8CKEx/+gpHbNQyJWehbFaTvqmHWB3ww==}
 
+  '@types/debug@4.1.13':
+    resolution: {integrity: sha512-KSVgmQmzMwPlmtljOomayoR89W4FynCAi3E8PPs7vmDVPe84hT+vGPKkJfThkmXs0x0jAaa9U8uW8bbfyS2fWw==}
+
   '@types/deep-eql@4.0.2':
     resolution: {integrity: sha512-c9h9dVVMigMPc4bwTvC5dxqtqJZwQPePsWjPlpSOnojbor6pGqdk541lfA7AqFQr5pB1BRdq0juY9db81BwyFw==}
 
+  '@types/estree-jsx@1.0.5':
+    resolution: {integrity: sha512-52CcUVNFyfb1A2ALocQw/Dd1BQFNmSdkuC3BkZ6iqhdMfQz7JWOFRuJFloOzjk+6WijU56m9oKXFAXc7o3Towg==}
+
   '@types/estree@1.0.8':
     resolution: {integrity: sha512-dWHzHa2WqEXI/O1E9OjrocMTKJl2mSrEolh1Iomrv6U+JuNwaHXsXx9bLu5gG7BUWFIN0skIQJQ/L1rIex4X6w==}
 
   '@types/geojson@7946.0.16':
     resolution: {integrity: sha512-6C8nqWur3j98U6+lXDfTUWIfgvZU+EumvpHKcYjujKH7woYyLj2sUmff0tRhrqM7BohUw7Pz3ZB1jj2gW9Fvmg==}
 
+  '@types/hast@3.0.4':
+    resolution: {integrity: sha512-WPs+bbQw5aCj+x6laNGWLH3wviHtoCv/P3+otBhbOhJgG8qtpdAMlTCxLtsTWA7LH1Oh/bFCHsBn0TPS5m30EQ==}
+
   '@types/json-schema@7.0.15':
     resolution: {integrity: sha512-5+fP8P8MFNC+AyZCDxrB2pkZFPGzqQWUzpSeuuVLvm8VMcorNYavBqoFcxK8bQz4Qsbn4oUEEem4wDLfcysGHA==}
 
+  '@types/mdast@4.0.4':
+    resolution: {integrity: sha512-kGaNbPh1k7AFzgpud/gMdvIm5xuECykRR+JnWKQno9TAXVa6WIVCGTPvYGekIDL4uwCZQSYbUxNBSb1aUo79oA==}
+
+  '@types/ms@2.1.0':
+    resolution: {integrity: sha512-GsCCIZDE/p3i96vtEqx+7dBUGXrc7zeSK3wwPHIaRThS+9OhWIXRqzs4d6k1SVU8g91DrNRWxWUGhp5KXQb2VA==}
+
   '@types/node@24.10.10':
     resolution: {integrity: sha512-+0/4J266CBGPUq/ELg7QUHhN25WYjE0wYTPSQJn1xeu8DOlIOPxXxrNGiLmfAWl7HMMgWFWXpt9IDjMWrF5Iow==}
 
@@ -1717,6 +1746,12 @@ packages:
   '@types/react@19.2.11':
     resolution: {integrity: sha512-tORuanb01iEzWvMGVGv2ZDhYZVeRMrw453DCSAIn/5yvcSVnMoUMTyf33nQJLahYEnv9xqrTNbgz4qY5EfSh0g==}
 
+  '@types/unist@2.0.11':
+    resolution: {integrity: sha512-CmBKiL6NNo/OqgmMn95Fk9Whlp2mtvIv+KNpQKN2F4SjvrEesubTRWGYSg+BnWZOnlCaSTU1sMpsBOzgbYhnsA==}
+
+  '@types/unist@3.0.3':
+    resolution: {integrity: sha512-ko/gIFJRv177XgZsZcBwnqJN5x/Gien8qNOn0D5bQU/zAzVf9Zt3BlcUiLqhV9y4ARk0GbT3tnUiPNgnTXzc/Q==}
+
   '@typescript-eslint/eslint-plugin@8.54.0':
     resolution: {integrity: sha512-hAAP5io/7csFStuOmR782YmTthKBJ9ND3WVL60hcOjvtGFb+HJxH4O5huAcmcZ9v9G8P+JETiZ/G1B8MALnWZQ==}
     engines: {node: ^18.18.0 || ^20.9.0 || >=21.1.0}
@@ -1776,6 +1811,9 @@ packages:
     resolution: {integrity: sha512-VFlhGSl4opC0bprJiItPQ1RfUhGDIBokcPwaFH4yiBCaNPeld/9VeXbiPO1cLyorQi1G1vL+ecBk1x8o1axORA==}
     engines: {node: ^18.18.0 || ^20.9.0 || >=21.1.0}
 
+  '@ungap/structured-clone@1.3.0':
+    resolution: {integrity: sha512-WmoN8qaIAo7WTYWbAZuG8PYEhn5fkz7dZrqTBZ7dtt//lL2Gwms1IcnQ5yHqjDfX8Ft5j4YzDM23f87zBfDe9g==}
+
   '@vitejs/plugin-react@5.1.3':
     resolution: {integrity: sha512-NVUnA6gQCl8jfoYqKqQU5Clv0aPw14KkZYCsX6T9Lfu9slI0LOU10OTwFHS/WmptsMMpshNd/1tuWsHQ2Uk+cg==}
     engines: {node: ^20.19.0 || >=22.12.0}
@@ -1875,6 +1913,9 @@ packages:
   babel-dead-code-elimination@1.0.12:
     resolution: {integrity: sha512-GERT7L2TiYcYDtYk1IpD+ASAYXjKbLTDPhBtYj7X1NuRMDTMtAx9kyBenub1Ev41lo91OHCKdmP+egTDmfQ7Ig==}
 
+  bail@2.0.2:
+    resolution: {integrity: sha512-0xO6mYd7JB2YesxDKplafRpsiOzPt9V02ddPCLbY1xYGPOX24NTyN50qnUxgCPcSoYMhKpAuBTjQoRZCAkUDRw==}
+
   balanced-match@1.0.2:
     resolution: {integrity: sha512-3oSeUO0TMV67hN1AmbXsK4yaqU7tjiHlbxRDZOpH0KW9+CeX4bRAaX0Anxt0tx2MrpRpWwQaPwIlISEJhYU5Pw==}
 
@@ -1919,6 +1960,9 @@ packages:
   caniuse-lite@1.0.30001768:
     resolution: {integrity: sha512-qY3aDRZC5nWPgHUgIB84WL+nySuo19wk0VJpp/XI9T34lrvkyhRvNVOFJOp2kxClQhiFBu+TaUSudf6oa3vkSA==}
 
+  ccount@2.0.1:
+    resolution: {integrity: sha512-eyrF0jiFpY+3drT6383f1qhkbGsLSifNAjA61IUjZjmLCWjItY6LB9ft9YhoDgwfmclB2zhu51Lc7+95b8NRAg==}
+
   chai@6.2.2:
     resolution: {integrity: sha512-NUPRluOfOiTKBKvWPtSD4PhFvWCqOi0BGStNWs57X9js7XGTprSmFoz5F0tWhR4WPjNeR9jXqdC7/UpSJTnlRg==}
     engines: {node: '>=18'}
@@ -1927,6 +1971,18 @@ packages:
     resolution: {integrity: sha512-oKnbhFyRIXpUuez8iBMmyEa4nbj4IOQyuhc/wy9kY7/WVPcwIO9VA668Pu8RkO7+0G76SLROeyw9CpQ061i4mA==}
     engines: {node: '>=10'}
 
+  character-entities-html4@2.1.0:
+    resolution: {integrity: sha512-1v7fgQRj6hnSwFpq1Eu0ynr/CDEw0rXo2B61qXrLNdHZmPKgb7fqS1a2JwF0rISo9q77jDI8VMEHoApn8qDoZA==}
+
+  character-entities-legacy@3.0.0:
+    resolution: {integrity: sha512-RpPp0asT/6ufRm//AJVwpViZbGM/MkjQFxJccQRHmISF/22NBtsHqAWmL+/pmkPWoIUJdWyeVleTl1wydHATVQ==}
+
+  character-entities@2.0.2:
+    resolution: {integrity: sha512-shx7oQ0Awen/BRIdkjkvz54PnEEI/EjwXDSIZp86/KKdbafHh1Df/RYGBhn4hbe2+uKC9FnT5UCEdyPz3ai9hQ==}
+
+  character-reference-invalid@2.0.1:
+    resolution: {integrity: sha512-iBZ4F4wRbyORVsu0jPV7gXkOsGYjGHPmAyv+HiHG8gi5PtC9KI2j1+v8/tlibRvjoWX027ypmG/n0HtO5t7unw==}
+
   chokidar@3.6.0:
     resolution: {integrity: sha512-7VT13fmjotKpGipCW9JEQAusEPE+Ei8nl6/g4FBAmIm0GOOLMua9NDDo/DWp0ZAxCr3cPq5ZpBqmPAQgDda2Pw==}
     engines: {node: '>= 8.10.0'}
@@ -1949,6 +2005,9 @@ packages:
     resolution: {integrity: sha512-FQN4MRfuJeHf7cBbBMJFXhKSDq+2kAArBlmRBvcvFE5BB1HZKXtSFASDhdlz9zOYwxh8lDdnvmMOe/+5cdoEdg==}
     engines: {node: '>= 0.8'}
 
+  comma-separated-tokens@2.0.3:
+    resolution: {integrity: sha512-Fu4hJdvzeylCfQPp9SGWidpzrMs7tTrlu6Vb8XGaRGck8QSNZJJp538Wrb60Lax4fPwR64ViY468OIUTbRlGZg==}
+
   commander@7.2.0:
     resolution: {integrity: sha512-QrWXB+ZQSVPmIWIhtEO9H+gwHaMGYiF5ChvoJ+K9ZGHG/sVsa6yiesAD1GC/x46sET00Xlwo1u49RVVVzvcSkw==}
     engines: {node: '>= 10'}
@@ -1976,6 +2035,11 @@ packages:
   css.escape@1.5.1:
     resolution: {integrity: sha512-YUifsXXuknHlUsmlgyY0PKzgPOr7/FjCePfHNt0jxm83wHZi44VDMQ7/fGNkjY3/jV1MC+1CmZbaHzugyeRtpg==}
 
+  cssesc@3.0.0:
+    resolution: {integrity: sha512-/Tb/JcjK111nNScGob5MNtsntNM1aCNUDipB/TkwZFhyDrrE47SOx/18wF2bbjgc3ZzCSKW1T5nt5EbFoAz/Vg==}
+    engines: {node: '>=4'}
+    hasBin: true
+
   csstype@3.2.3:
     resolution: {integrity: sha512-z1HGKcYy2xA8AGQfwrn0PAy+PB7X/GSj3UVJW9qKyn43xWa+gl5nXmU4qqLMRzWVLFC8KusUX8T/0kCiOYpAIQ==}
 
@@ -2122,6 +2186,9 @@ packages:
   decimal.js@10.6.0:
     resolution: {integrity: sha512-YpgQiITW3JXGntzdUmyUR1V812Hn8T1YVXhCu+wO3OpS4eU9l4YdD3qjyiKdV6mvV29zapkMeD390UVEf2lkUg==}
 
+  decode-named-character-reference@1.3.0:
+    resolution: {integrity: sha512-GtpQYB283KrPp6nRw50q3U9/VfOutZOe103qlN7BPP6Ad27xYnOIWv4lPzo8HCAL+mMZofJ9KEy30fq6MfaK6Q==}
+
   deep-is@0.1.4:
     resolution: {integrity: sha512-oIPzksmTg4/MriiaYGO+okXDT7ztn/w3Eptv/+gSIdMdKsJo0u4CfYNFJPy+4SKMuCqGw2wxnA+URMg3t8a/bQ==}
 
@@ -2143,6 +2210,9 @@ packages:
   detect-node-es@1.1.0:
     resolution: {integrity: sha512-ypdmJU/TbBby2Dxibuv7ZLW3Bs1QEmM7nHjEANfohJLvE0XVujisn1qPJcZxg+qDucsr+bP6fLD1rPS3AhJ7EQ==}
 
+  devlop@1.1.0:
+    resolution: {integrity: sha512-RWmIqhcFf1lRYBvNmr7qTNuyCt/7/ns2jbpp1+PalgE/rDQcBT0fioSMUpJ93irlUhC5hrg4cYqe6U+0ImW0rA==}
+
   diff@8.0.3:
     resolution: {integrity: sha512-qejHi7bcSD4hQAZE0tNAawRK1ZtafHDmMTMkrrIGgSLl7hTnQHmKCeB45xAcbfTqK2zowkM3j3bHt/4b/ARbYQ==}
     engines: {node: '>=0.3.1'}
@@ -2200,6 +2270,10 @@ packages:
     resolution: {integrity: sha512-TtpcNJ3XAzx3Gq8sWRzJaVajRs0uVxA2YAkdb1jm2YkPz4G6egUFAyA3n5vtEIZefPk5Wa4UXbKuS5fKkJWdgA==}
     engines: {node: '>=10'}
 
+  escape-string-regexp@5.0.0:
+    resolution: {integrity: sha512-/veY75JbMK4j1yjvuUxuVsiS/hr/4iHs9FTT6cgTexxdE0Ly/glccBAkloH/DofkjRbZU3bnoj38mOmhkZ0lHw==}
+    engines: {node: '>=12'}
+
   eslint-config-prettier@10.1.8:
     resolution: {integrity: sha512-82GZUjRS0p/jganf6q1rEO25VSoHH0hKPCTrgillPjdI/3bgBhAE1QzHrHTizjpRvy6pGAvKjDJtk2pF9NDq8w==}
     hasBin: true
@@ -2260,6 +2334,9 @@ packages:
     resolution: {integrity: sha512-MMdARuVEQziNTeJD8DgMqmhwR11BRQ/cBP+pLtYdSTnf3MIO8fFeiINEbX36ZdNlfU/7A9f3gUw49B3oQsvwBA==}
     engines: {node: '>=4.0'}
 
+  estree-util-is-identifier-name@3.0.0:
+    resolution: {integrity: sha512-hFtqIDZTIUZ9BXLb8y4pYGyk6+wekIivNVTcmvk8NoOh+VeRn5y6cEHzbURrWbfp1fIqdVipilzj+lfaadNZmg==}
+
   estree-walker@3.0.3:
     resolution: {integrity: sha512-7RUKfXgSMMkzt6ZuXmqapOurLGPPfgj6l9uRZ7lRGolvk0y2yocc35LdcxKC5PQZdn2DMqioAQ2NoWcrTKmm6g==}
 
@@ -2271,6 +2348,9 @@ packages:
     resolution: {integrity: sha512-knvyeauYhqjOYvQ66MznSMs83wmHrCycNEN6Ao+2AeYEfxUIkuiVxdEa1qlGEPK+We3n0THiDciYSsCcgW/DoA==}
     engines: {node: '>=12.0.0'}
 
+  extend@3.0.2:
+    resolution: {integrity: sha512-fjquC59cD7CyW6urNXK0FBufkZcoiGG80wTuPujX590cB5Ttln20E2UB4S/WARVqhXffZl2LNgS+gQdPIIim/g==}
+
   fast-deep-equal@3.1.3:
     resolution: {integrity: sha512-f3qQ9oQy9j2AhBe/H9VC91wLmKBCCU/gDOnKNAYG5hswO7BLKj09Hc5HYNz9cGI++xlpDCIgDaitVs03ATR84Q==}
 
@@ -2392,6 +2472,12 @@ packages:
     resolution: {integrity: sha512-0hJU9SCPvmMzIBdZFqNPXWa6dqh7WdH0cII9y+CyS8rG3nL48Bclra9HmKhVVUHyPWNH5Y7xDwAB7bfgSjkUMQ==}
     engines: {node: '>= 0.4'}
 
+  hast-util-to-jsx-runtime@2.3.6:
+    resolution: {integrity: sha512-zl6s8LwNyo1P9uw+XJGvZtdFF1GdAkOg8ujOw+4Pyb76874fLps4ueHXDhXWdk6YHQ6OgUtinliG7RsYvCbbBg==}
+
+  hast-util-whitespace@3.0.0:
+    resolution: {integrity: sha512-88JUN06ipLwsnv+dVn+OIYOvAuvBMy/Qoi6O7mQHxdPXpjy+Cd6xRkWwux7DKO+4sYILtLBRIKgsdpS2gQc7qw==}
+
   hermes-estree@0.25.1:
     resolution: {integrity: sha512-0wUoCcLp+5Ev5pDW2OriHC2MJCbwLwuRx+gAqMTOkGKJJiBCLjtrvy4PWUGn6MIVefecRpzoOZ/UV6iGdOr+Cw==}
 
@@ -2405,6 +2491,9 @@ packages:
   html-parse-stringify@3.0.1:
     resolution: {integrity: sha512-KknJ50kTInJ7qIScF3jeaFRpMpE8/lfiTdzf/twXyPBLAGrLRTmkz3AdTnKeh40X8k9L2fdYwEp/42WGXIRGcg==}
 
+  html-url-attributes@3.0.1:
+    resolution: {integrity: sha512-ol6UPyBWqsrO6EJySPz2O7ZSr856WDrEzM5zMqp+FJJLGMW35cLYmmZnl0vztAZxRUoNZJFTCohfjuIJ8I4QBQ==}
+
   html2canvas@1.4.1:
     resolution: {integrity: sha512-fPU6BHNpsyIhr8yyMpTLLxAbkaK8ArIBcmZIRiBLiDhjeqvXolaEmDGmELFuX9I4xDcaKKcJl+TKZLqruBbmWA==}
     engines: {node: '>=8.0.0'}
@@ -2444,14 +2533,26 @@ packages:
     resolution: {integrity: sha512-EdDDZu4A2OyIK7Lr/2zG+w5jmbuk1DVBnEwREQvBzspBJkCEbRa8GxU1lghYcaGJCnRWibjDXlq779X1/y5xwg==}
     engines: {node: '>=8'}
 
+  inline-style-parser@0.2.7:
+    resolution: {integrity: sha512-Nb2ctOyNR8DqQoR0OwRG95uNWIC0C1lCgf5Naz5H6Ji72KZ8OcFZLz2P5sNgwlyoJ8Yif11oMuYs5pBQa86csA==}
+
   internmap@2.0.3:
     resolution: {integrity: sha512-5Hh7Y1wQbvY5ooGgPbDaL5iYLAPzMTUrjMulskHLH6wnv/A+1q5rgEaiuqEjB+oxGXIVZs1FF+R/KPN3ZSQYYg==}
     engines: {node: '>=12'}
 
+  is-alphabetical@2.0.1:
+    resolution: {integrity: sha512-FWyyY60MeTNyeSRpkM2Iry0G9hpr7/9kD40mD/cGQEuilcZYS4okz8SN2Q6rLCJ8gbCt6fN+rC+6tMGS99LaxQ==}
+
+  is-alphanumerical@2.0.1:
+    resolution: {integrity: sha512-hmbYhX/9MUMF5uh7tOXyK/n0ZvWpad5caBA17GsC6vyuCqaWliRG5K1qS9inmUhEMaOBIW7/whAnSwveW/LtZw==}
+
   is-binary-path@2.1.0:
     resolution: {integrity: sha512-ZMERYes6pDydyuGidse7OsHxtbI7WVeUEozgR/g7rd0xUimYNlvZRE/K2MgZTjWy725IfelLeVcEM97mmtRGXw==}
     engines: {node: '>=8'}
 
+  is-decimal@2.0.1:
+    resolution: {integrity: sha512-AAB9hiomQs5DXWcRB1rqsxGUstbRroFOPPVAomNk/3XHR5JyEZChOyTWe2oayKnsSsr/kcGqF+z6yuH6HHpN0A==}
+
   is-extglob@2.1.1:
     resolution: {integrity: sha512-SbKbANkN603Vi4jEZv49LeVJMn4yGwsbzZworEoyEiutsN3nJYdbO36zfhGJ6QEDpOZIFkDtnq5JRxmvl3jsoQ==}
     engines: {node: '>=0.10.0'}
@@ -2460,10 +2561,17 @@ packages:
     resolution: {integrity: sha512-xelSayHH36ZgE7ZWhli7pW34hNbNl8Ojv5KVmkJD4hBdD3th8Tfk9vYasLM+mXWOZhFkgZfxhLSnrwRr4elSSg==}
     engines: {node: '>=0.10.0'}
 
+  is-hexadecimal@2.0.1:
+    resolution: {integrity: sha512-DgZQp241c8oO6cA1SbTEWiXeoxV42vlcJxgH+B3hi1AiqqKruZR3ZGF8In3fj4+/y/7rHvlOZLZtgJ/4ttYGZg==}
+
   is-number@7.0.0:
     resolution: {integrity: sha512-41Cifkg6e8TylSpdtTpeLVMqvSBEVzTttHvERD741+pnZ8ANv0004MRL43QKPDlK9cGvNp6NZWZUBlbGXYxxng==}
     engines: {node: '>=0.12.0'}
 
+  is-plain-obj@4.1.0:
+    resolution: {integrity: sha512-+Pgi+vMuUNkJyExiMBt5IlFoMyKnr5zhJ4Uspz58WOhBF5QoIZkFyNHIbBAtHwzVAgk5RtndVNsDRN61/mmDqg==}
+    engines: {node: '>=12'}
+
   is-potential-custom-element-name@1.0.1:
     resolution: {integrity: sha512-bCYeRA2rVibKZd+s2625gGnGF/t7DSqDs4dP7CrLA1m7jKWz6pps0LpYLJN8Q64HtmPKJ1hrN3nzPNKFEKOUiQ==}
 
@@ -2597,6 +2705,9 @@ packages:
   lodash.merge@4.6.2:
     resolution: {integrity: sha512-0KpjqXRVvrYyCsX1swR/XTK0va6VQkQM6MNo7PqW77ByjAhoARA8EfrP1N4+KlKj8YS0ZUCtRT/YUuhyYDujIQ==}
 
+  longest-streak@3.1.0:
+    resolution: {integrity: sha512-9Ri+o0JYgehTaVBBDoMqIl8GXtbWg711O3srftcHhZ0dqnETqLaoIK0x17fUw9rFSlK/0NlsKe0Ahhyl5pXE2g==}
+
   lru-cache@11.2.7:
     resolution: {integrity: sha512-aY/R+aEsRelme17KGQa/1ZSIpLpNYYrhcrepKTZgE+W3WM16YMCaPwOHLHsmopZHELU0Ojin1lPVxKR0MihncA==}
     engines: {node: 20 || >=22}
@@ -2616,13 +2727,145 @@ packages:
   magic-string@0.30.21:
     resolution: {integrity: sha512-vd2F4YUyEXKGcLHoq+TEyCjxueSeHnFxyyjNp80yg0XV4vUhnDer/lvvlqM/arB5bXQN5K2/3oinyCRyx8T2CQ==}
 
+  markdown-table@3.0.4:
+    resolution: {integrity: sha512-wiYz4+JrLyb/DqW2hkFJxP7Vd7JuTDm77fvbM8VfEQdmSMqcImWeeRbHwZjBjIFki/VaMK2BhFi7oUUZeM5bqw==}
+
   math-intrinsics@1.1.0:
     resolution: {integrity: sha512-/IXtbwEk5HTPyEwyKX6hGkYXxM9nbj64B+ilVJnC/R6B0pH5G4V3b0pVbL7DBj4tkhBAppbQUlf6F6Xl9LHu1g==}
     engines: {node: '>= 0.4'}
 
+  mdast-util-find-and-replace@3.0.2:
+    resolution: {integrity: sha512-Tmd1Vg/m3Xz43afeNxDIhWRtFZgM2VLyaf4vSTYwudTyeuTneoL3qtWMA5jeLyz/O1vDJmmV4QuScFCA2tBPwg==}
+
+  mdast-util-from-markdown@2.0.3:
+    resolution: {integrity: sha512-W4mAWTvSlKvf8L6J+VN9yLSqQ9AOAAvHuoDAmPkz4dHf553m5gVj2ejadHJhoJmcmxEnOv6Pa8XJhpxE93kb8Q==}
+
+  mdast-util-gfm-autolink-literal@2.0.1:
+    resolution: {integrity: sha512-5HVP2MKaP6L+G6YaxPNjuL0BPrq9orG3TsrZ9YXbA3vDw/ACI4MEsnoDpn6ZNm7GnZgtAcONJyPhOP8tNJQavQ==}
+
+  mdast-util-gfm-footnote@2.1.0:
+    resolution: {integrity: sha512-sqpDWlsHn7Ac9GNZQMeUzPQSMzR6Wv0WKRNvQRg0KqHh02fpTz69Qc1QSseNX29bhz1ROIyNyxExfawVKTm1GQ==}
+
+  mdast-util-gfm-strikethrough@2.0.0:
+    resolution: {integrity: sha512-mKKb915TF+OC5ptj5bJ7WFRPdYtuHv0yTRxK2tJvi+BDqbkiG7h7u/9SI89nRAYcmap2xHQL9D+QG/6wSrTtXg==}
+
+  mdast-util-gfm-table@2.0.0:
+    resolution: {integrity: sha512-78UEvebzz/rJIxLvE7ZtDd/vIQ0RHv+3Mh5DR96p7cS7HsBhYIICDBCu8csTNWNO6tBWfqXPWekRuj2FNOGOZg==}
+
+  mdast-util-gfm-task-list-item@2.0.0:
+    resolution: {integrity: sha512-IrtvNvjxC1o06taBAVJznEnkiHxLFTzgonUdy8hzFVeDun0uTjxxrRGVaNFqkU1wJR3RBPEfsxmU6jDWPofrTQ==}
+
+  mdast-util-gfm@3.1.0:
+    resolution: {integrity: sha512-0ulfdQOM3ysHhCJ1p06l0b0VKlhU0wuQs3thxZQagjcjPrlFRqY215uZGHHJan9GEAXd9MbfPjFJz+qMkVR6zQ==}
+
+  mdast-util-mdx-expression@2.0.1:
+    resolution: {integrity: sha512-J6f+9hUp+ldTZqKRSg7Vw5V6MqjATc+3E4gf3CFNcuZNWD8XdyI6zQ8GqH7f8169MM6P7hMBRDVGnn7oHB9kXQ==}
+
+  mdast-util-mdx-jsx@3.2.0:
+    resolution: {integrity: sha512-lj/z8v0r6ZtsN/cGNNtemmmfoLAFZnjMbNyLzBafjzikOM+glrjNHPlf6lQDOTccj9n5b0PPihEBbhneMyGs1Q==}
+
+  mdast-util-mdxjs-esm@2.0.1:
+    resolution: {integrity: sha512-EcmOpxsZ96CvlP03NghtH1EsLtr0n9Tm4lPUJUBccV9RwUOneqSycg19n5HGzCf+10LozMRSObtVr3ee1WoHtg==}
+
+  mdast-util-phrasing@4.1.0:
+    resolution: {integrity: sha512-TqICwyvJJpBwvGAMZjj4J2n0X8QWp21b9l0o7eXyVJ25YNWYbJDVIyD1bZXE6WtV6RmKJVYmQAKWa0zWOABz2w==}
+
+  mdast-util-to-hast@13.2.1:
+    resolution: {integrity: sha512-cctsq2wp5vTsLIcaymblUriiTcZd0CwWtCbLvrOzYCDZoWyMNV8sZ7krj09FSnsiJi3WVsHLM4k6Dq/yaPyCXA==}
+
+  mdast-util-to-markdown@2.1.2:
+    resolution: {integrity: sha512-xj68wMTvGXVOKonmog6LwyJKrYXZPvlwabaryTjLh9LuvovB/KAH+kvi8Gjj+7rJjsFi23nkUxRQv1KqSroMqA==}
+
+  mdast-util-to-string@4.0.0:
+    resolution: {integrity: sha512-0H44vDimn51F0YwvxSJSm0eCDOJTRlmN0R1yBh4HLj9wiV1Dn0QoXGbvFAWj2hSItVTlCmBF1hqKlIyUBVFLPg==}
+
   mdn-data@2.27.1:
     resolution: {integrity: sha512-9Yubnt3e8A0OKwxYSXyhLymGW4sCufcLG6VdiDdUGVkPhpqLxlvP5vl1983gQjJl3tqbrM731mjaZaP68AgosQ==}
 
+  micromark-core-commonmark@2.0.3:
+    resolution: {integrity: sha512-RDBrHEMSxVFLg6xvnXmb1Ayr2WzLAWjeSATAoxwKYJV94TeNavgoIdA0a9ytzDSVzBy2YKFK+emCPOEibLeCrg==}
+
+  micromark-extension-gfm-autolink-literal@2.1.0:
+    resolution: {integrity: sha512-oOg7knzhicgQ3t4QCjCWgTmfNhvQbDDnJeVu9v81r7NltNCVmhPy1fJRX27pISafdjL+SVc4d3l48Gb6pbRypw==}
+
+  micromark-extension-gfm-footnote@2.1.0:
+    resolution: {integrity: sha512-/yPhxI1ntnDNsiHtzLKYnE3vf9JZ6cAisqVDauhp4CEHxlb4uoOTxOCJ+9s51bIB8U1N1FJ1RXOKTIlD5B/gqw==}
+
+  micromark-extension-gfm-strikethrough@2.1.0:
+    resolution: {integrity: sha512-ADVjpOOkjz1hhkZLlBiYA9cR2Anf8F4HqZUO6e5eDcPQd0Txw5fxLzzxnEkSkfnD0wziSGiv7sYhk/ktvbf1uw==}
+
+  micromark-extension-gfm-table@2.1.1:
+    resolution: {integrity: sha512-t2OU/dXXioARrC6yWfJ4hqB7rct14e8f7m0cbI5hUmDyyIlwv5vEtooptH8INkbLzOatzKuVbQmAYcbWoyz6Dg==}
+
+  micromark-extension-gfm-tagfilter@2.0.0:
+    resolution: {integrity: sha512-xHlTOmuCSotIA8TW1mDIM6X2O1SiX5P9IuDtqGonFhEK0qgRI4yeC6vMxEV2dgyr2TiD+2PQ10o+cOhdVAcwfg==}
+
+  micromark-extension-gfm-task-list-item@2.1.0:
+    resolution: {integrity: sha512-qIBZhqxqI6fjLDYFTBIa4eivDMnP+OZqsNwmQ3xNLE4Cxwc+zfQEfbs6tzAo2Hjq+bh6q5F+Z8/cksrLFYWQQw==}
+
+  micromark-extension-gfm@3.0.0:
+    resolution: {integrity: sha512-vsKArQsicm7t0z2GugkCKtZehqUm31oeGBV/KVSorWSy8ZlNAv7ytjFhvaryUiCUJYqs+NoE6AFhpQvBTM6Q4w==}
+
+  micromark-factory-destination@2.0.1:
+    resolution: {integrity: sha512-Xe6rDdJlkmbFRExpTOmRj9N3MaWmbAgdpSrBQvCFqhezUn4AHqJHbaEnfbVYYiexVSs//tqOdY/DxhjdCiJnIA==}
+
+  micromark-factory-label@2.0.1:
+    resolution: {integrity: sha512-VFMekyQExqIW7xIChcXn4ok29YE3rnuyveW3wZQWWqF4Nv9Wk5rgJ99KzPvHjkmPXF93FXIbBp6YdW3t71/7Vg==}
+
+  micromark-factory-space@2.0.1:
+    resolution: {integrity: sha512-zRkxjtBxxLd2Sc0d+fbnEunsTj46SWXgXciZmHq0kDYGnck/ZSGj9/wULTV95uoeYiK5hRXP2mJ98Uo4cq/LQg==}
+
+  micromark-factory-title@2.0.1:
+    resolution: {integrity: sha512-5bZ+3CjhAd9eChYTHsjy6TGxpOFSKgKKJPJxr293jTbfry2KDoWkhBb6TcPVB4NmzaPhMs1Frm9AZH7OD4Cjzw==}
+
+  micromark-factory-whitespace@2.0.1:
+    resolution: {integrity: sha512-Ob0nuZ3PKt/n0hORHyvoD9uZhr+Za8sFoP+OnMcnWK5lngSzALgQYKMr9RJVOWLqQYuyn6ulqGWSXdwf6F80lQ==}
+
+  micromark-util-character@2.1.1:
+    resolution: {integrity: sha512-wv8tdUTJ3thSFFFJKtpYKOYiGP2+v96Hvk4Tu8KpCAsTMs6yi+nVmGh1syvSCsaxz45J6Jbw+9DD6g97+NV67Q==}
+
+  micromark-util-chunked@2.0.1:
+    resolution: {integrity: sha512-QUNFEOPELfmvv+4xiNg2sRYeS/P84pTW0TCgP5zc9FpXetHY0ab7SxKyAQCNCc1eK0459uoLI1y5oO5Vc1dbhA==}
+
+  micromark-util-classify-character@2.0.1:
+    resolution: {integrity: sha512-K0kHzM6afW/MbeWYWLjoHQv1sgg2Q9EccHEDzSkxiP/EaagNzCm7T/WMKZ3rjMbvIpvBiZgwR3dKMygtA4mG1Q==}
+
+  micromark-util-combine-extensions@2.0.1:
+    resolution: {integrity: sha512-OnAnH8Ujmy59JcyZw8JSbK9cGpdVY44NKgSM7E9Eh7DiLS2E9RNQf0dONaGDzEG9yjEl5hcqeIsj4hfRkLH/Bg==}
+
+  micromark-util-decode-numeric-character-reference@2.0.2:
+    resolution: {integrity: sha512-ccUbYk6CwVdkmCQMyr64dXz42EfHGkPQlBj5p7YVGzq8I7CtjXZJrubAYezf7Rp+bjPseiROqe7G6foFd+lEuw==}
+
+  micromark-util-decode-string@2.0.1:
+    resolution: {integrity: sha512-nDV/77Fj6eH1ynwscYTOsbK7rR//Uj0bZXBwJZRfaLEJ1iGBR6kIfNmlNqaqJf649EP0F3NWNdeJi03elllNUQ==}
+
+  micromark-util-encode@2.0.1:
+    resolution: {integrity: sha512-c3cVx2y4KqUnwopcO9b/SCdo2O67LwJJ/UyqGfbigahfegL9myoEFoDYZgkT7f36T0bLrM9hZTAaAyH+PCAXjw==}
+
+  micromark-util-html-tag-name@2.0.1:
+    resolution: {integrity: sha512-2cNEiYDhCWKI+Gs9T0Tiysk136SnR13hhO8yW6BGNyhOC4qYFnwF1nKfD3HFAIXA5c45RrIG1ub11GiXeYd1xA==}
+
+  micromark-util-normalize-identifier@2.0.1:
+    resolution: {integrity: sha512-sxPqmo70LyARJs0w2UclACPUUEqltCkJ6PhKdMIDuJ3gSf/Q+/GIe3WKl0Ijb/GyH9lOpUkRAO2wp0GVkLvS9Q==}
+
+  micromark-util-resolve-all@2.0.1:
+    resolution: {integrity: sha512-VdQyxFWFT2/FGJgwQnJYbe1jjQoNTS4RjglmSjTUlpUMa95Htx9NHeYW4rGDJzbjvCsl9eLjMQwGeElsqmzcHg==}
+
+  micromark-util-sanitize-uri@2.0.1:
+    resolution: {integrity: sha512-9N9IomZ/YuGGZZmQec1MbgxtlgougxTodVwDzzEouPKo3qFWvymFHWcnDi2vzV1ff6kas9ucW+o3yzJK9YB1AQ==}
+
+  micromark-util-subtokenize@2.1.0:
+    resolution: {integrity: sha512-XQLu552iSctvnEcgXw6+Sx75GflAPNED1qx7eBJ+wydBb2KCbRZe+NwvIEEMM83uml1+2WSXpBAcp9IUCgCYWA==}
+
+  micromark-util-symbol@2.0.1:
+    resolution: {integrity: sha512-vs5t8Apaud9N28kgCrRUdEed4UJ+wWNvicHLPxCa9ENlYuAY31M0ETy5y1vA33YoNPDFTghEbnh6efaE8h4x0Q==}
+
+  micromark-util-types@2.0.2:
+    resolution: {integrity: sha512-Yw0ECSpJoViF1qTU4DC6NwtC4aWGt1EkzaQB8KPPyCRR8z9TWeV0HbEFGTO+ZY1wB22zmxnJqhPyTpOVCpeHTA==}
+
+  micromark@4.0.2:
+    resolution: {integrity: sha512-zpe98Q6kvavpCr1NPVSCMebCKfD7CA2NqZ+rykeNhONIJBpc1tFKt9hucLGwha3jNTNI8lHpctWJWoimVF4PfA==}
+
   mime-db@1.52.0:
     resolution: {integrity: sha512-sPU4uV7dYlvtWJxwwxHD0PuihVNiE7TyAbQ5SWxDCB9mUYvOgroQOwYQQOKPJ8CIbE+1ETVlOoK1UC2nU3gYvg==}
     engines: {node: '>= 0.6'}
@@ -2679,6 +2922,9 @@ packages:
     resolution: {integrity: sha512-GQ2EWRpQV8/o+Aw8YqtfZZPfNRWZYkbidE9k5rpl/hC3vtHHBfGm2Ifi6qWV+coDGkrUKZAxE3Lot5kcsRlh+g==}
     engines: {node: '>=6'}
 
+  parse-entities@4.0.2:
+    resolution: {integrity: sha512-GG2AQYWoLgL877gQIKeRPGO1xF9+eG1ujIb5soS5gPvLQ1y2o8FL90w2QWNdf9I361Mpp7726c+lj3U0qK1uGw==}
+
   parse5@8.0.0:
     resolution: {integrity: sha512-9m4m5GSgXjL4AjumKzq1Fgfp3Z8rsvjRNbnkVwfu2ImRqE5D0LnY2QfDen18FSY9C573YU5XxSapdHZTZ2WolA==}
 
@@ -2704,6 +2950,10 @@ packages:
     resolution: {integrity: sha512-5gTmgEY/sqK6gFXLIsQNH19lWb4ebPDLA4SdLP7dsWkIXHWlG66oPuVvXSGFPppYZz8ZDZq0dYYrbHfBCVUb1Q==}
     engines: {node: '>=12'}
 
+  postcss-selector-parser@6.0.10:
+    resolution: {integrity: sha512-IQ7TZdoaqbT+LCpShg46jnZVlhWD2w6iQYAcYXfHARZ7X1t/UGhhceQDs5X0cGqKvYlHNOuv7Oa1xmb0oQuA3w==}
+    engines: {node: '>=4'}
+
   postcss@8.5.6:
     resolution: {integrity: sha512-3Ybi1tAuwAP9s0r1UQ2J4n5Y0G05bJkpUIO0/bI9MhwmD70S5aTWbXGBwxHrelT+XM1k6dM0pk+SwNkpTRN7Pg==}
     engines: {node: ^10 || ^12 || >=14}
@@ -2721,6 +2971,9 @@ packages:
     resolution: {integrity: sha512-Qb1gy5OrP5+zDf2Bvnzdl3jsTf1qXVMazbvCoKhtKqVs4/YK4ozX4gKQJJVyNe+cajNPn0KoC0MC3FUmaHWEmQ==}
     engines: {node: ^10.13.0 || ^12.13.0 || ^14.15.0 || >=15.0.0}
 
+  property-information@7.1.0:
+    resolution: {integrity: sha512-TwEZ+X+yCJmYfL7TPUOcvBZ4QfoT5YenQiJuX//0th53DE6w0xxLEtfK3iyryQFddXuvkIk51EEgrJQ0WJkOmQ==}
+
   proxy-from-env@1.1.0:
     resolution: {integrity: sha512-D+zkORCbA9f1tdWRK0RaCR3GPv50cMxcrz4X8k5LTSUD1Dkw47mKJEZQNunItRTkWwgtaUSo1RVFRIG9ZXiFYg==}
 
@@ -2765,6 +3018,12 @@ packages:
   react-is@17.0.2:
     resolution: {integrity: sha512-w2GsyukL62IJnlaff/nRegPQR94C/XXamvMWmSHRJ4y7Ts/4ocGRmTHvOs8PSE6pB3dWOrD/nueuU5sduBsQ4w==}
 
+  react-markdown@10.1.0:
+    resolution: {integrity: sha512-qKxVopLT/TyA6BX3Ue5NwabOsAzm0Q7kAPwq6L+wWDwisYs7R8vZ0nRXqq6rkueboxpkjvLGU9fWifiX/ZZFxQ==}
+    peerDependencies:
+      '@types/react': '>=18'
+      react: '>=18'
+
   react-refresh@0.18.0:
     resolution: {integrity: sha512-QgT5//D3jfjJb6Gsjxv0Slpj23ip+HtOpnNgnb2S5zU3CB26G/IDPGoy4RJB42wzFE46DRsstbW6tKHoKbhAxw==}
     engines: {node: '>=0.10.0'}
@@ -2815,6 +3074,18 @@ packages:
     resolution: {integrity: sha512-6tDA8g98We0zd0GvVeMT9arEOnTw9qM03L9cJXaCjrip1OO764RDBLBfrB4cwzNGDj5OA5ioymC9GkizgWJDUg==}
     engines: {node: '>=8'}
 
+  remark-gfm@4.0.1:
+    resolution: {integrity: sha512-1quofZ2RQ9EWdeN34S79+KExV1764+wCUGop5CPL1WGdD0ocPpu91lzPGbwWMECpEpd42kJGQwzRfyov9j4yNg==}
+
+  remark-parse@11.0.0:
+    resolution: {integrity: sha512-FCxlKLNGknS5ba/1lmpYijMUzX2esxW5xQqjWxw2eHFfS2MSdaHVINFmhjo+qN1WhZhNimq0dZATN9pH0IDrpA==}
+
+  remark-rehype@11.1.2:
+    resolution: {integrity: sha512-Dh7l57ianaEoIpzbp0PC9UKAdCSVklD8E5Rpw7ETfbTl3FqcOOgq5q2LVDhgGCkaBv7p24JXikPdvhhmHvKMsw==}
+
+  remark-stringify@11.0.0:
+    resolution: {integrity: sha512-1OSmLd3awB/t8qdoEOMazZkNsfVTeY4fTsgzcQFdXNq8ToTN4ZGwrMnlda4K6smTFKD+GRV6O48i6Z4iKgPPpw==}
+
   require-from-string@2.0.2:
     resolution: {integrity: sha512-Xf0nWe6RseziFMu+Ap9biiUbmplq6S9/p+7w7YXP/JBHhrUDDUhwa+vANyubuqfZWTveU//DYVGsDG7RKL/vEw==}
     engines: {node: '>=0.10.0'}
@@ -2889,12 +3160,18 @@ packages:
     resolution: {integrity: sha512-i5uvt8C3ikiWeNZSVZNWcfZPItFQOsYTUAOkcUPGd8DqDy1uOUikjt5dG+uRlwyvR108Fb9DOd4GvXfT0N2/uQ==}
     engines: {node: '>= 12'}
 
+  space-separated-tokens@2.0.2:
+    resolution: {integrity: sha512-PEGlAwrG8yXGXRjW32fGbg66JAlOAwbObuqVoJpv/mRgoWDQfgH1wDPvtzWyUSNAXBGSk8h755YDbbcEy3SH2Q==}
+
   stackback@0.0.2:
     resolution: {integrity: sha512-1XMJE5fQo1jGH6Y/7ebnwPOBEkIEnT4QF32d5R1+VXdXveM0IBMJt8zfaxX1P3QhVwrYe+576+jkANtSS2mBbw==}
 
   std-env@3.10.0:
     resolution: {integrity: sha512-5GS12FdOZNliM5mAOxFRg7Ir0pWz8MdpYm6AY6VPkGpbA7ZzmbzNcBJQ0GPvvyWgcY7QAhCgf9Uy89I03faLkg==}
 
+  stringify-entities@4.0.4:
+    resolution: {integrity: sha512-IwfBptatlO+QCJUo19AqvrPNqlVMpW9YEL2LIVY+Rpv2qsjCGxaDLNRgeGsQWJhfItebuJhsGSLjaBbNSQ+ieg==}
+
   strip-indent@3.0.0:
     resolution: {integrity: sha512-laJTa3Jb+VQpaC6DseHhF7dXVqHTfJPCRDaEbid/drOhgitgYku/letMUqOXFoWV0zIIUbjpdH2t+tYj4bQMRQ==}
     engines: {node: '>=8'}
@@ -2903,6 +3180,12 @@ packages:
     resolution: {integrity: sha512-6fPc+R4ihwqP6N/aIv2f1gMH8lOVtWQHoqC4yK6oSDVVocumAsfCqjkXnqiYMhmMwS/mEHLp7Vehlt3ql6lEig==}
     engines: {node: '>=8'}
 
+  style-to-js@1.1.21:
+    resolution: {integrity: sha512-RjQetxJrrUJLQPHbLku6U/ocGtzyjbJMP9lCNK7Ag0CNh690nSH8woqWH9u16nMjYBAok+i7JO1NP2pOy8IsPQ==}
+
+  style-to-object@1.0.14:
+    resolution: {integrity: sha512-LIN7rULI0jBscWQYaSswptyderlarFkjQ+t79nzty8tcIAceVomEVlLzH5VP4Cmsv6MtKhs7qaAiwlcp+Mgaxw==}
+
   supports-color@7.2.0:
     resolution: {integrity: sha512-qpCAvRl9stuOHveKsn7HncJRvv501qIacKzQlO/+Lwxc9+0q2wLyv4Dfvt80/DPn2pqOBsJdDiogXGR9+OvwRw==}
     engines: {node: '>=8'}
@@ -2963,6 +3246,12 @@ packages:
     resolution: {integrity: sha512-bLVMLPtstlZ4iMQHpFHTR7GAGj2jxi8Dg0s2h2MafAE4uSWF98FC/3MomU51iQAMf8/qDUbKWf5GxuvvVcXEhw==}
     engines: {node: '>=20'}
 
+  trim-lines@3.0.1:
+    resolution: {integrity: sha512-kRj8B+YHZCc9kQYdWfJB2/oUl9rA99qbowYYBtr4ui4mZyAQ2JpvVBd/6U2YloATfqBhBTSMhTpgBHtU0Mf3Rg==}
+
+  trough@2.2.0:
+    resolution: {integrity: sha512-tmMpK00BjZiUyVyvrBK7knerNgmgvcV/KLVyuma/SC+TQN167GrMRciANTz09+k3zW8L8t60jWO1GpfkZdjTaw==}
+
   ts-api-utils@2.4.0:
     resolution: {integrity: sha512-3TaVTaAv2gTiMB35i3FiGJaRfwb3Pyn/j3m/bfAvGe8FB7CF6u+LMYqYlDh7reQf7UNvoTvdfAqHGmPGOSsPmA==}
     engines: {node: '>=18.12'}
@@ -3000,6 +3289,24 @@ packages:
     resolution: {integrity: sha512-BM/JzwwaRXxrLdElV2Uo6cTLEjhSb3WXboncJamZ15NgUURmvlXvxa6xkwIOILIjPNo9i8ku136ZvWV0Uly8+w==}
     engines: {node: '>=20.18.1'}
 
+  unified@11.0.5:
+    resolution: {integrity: sha512-xKvGhPWw3k84Qjh8bI3ZeJjqnyadK+GEFtazSfZv/rKeTkTjOJho6mFqh2SM96iIcZokxiOpg78GazTSg8+KHA==}
+
+  unist-util-is@6.0.1:
+    resolution: {integrity: sha512-LsiILbtBETkDz8I9p1dQ0uyRUWuaQzd/cuEeS1hoRSyW5E5XGmTzlwY1OrNzzakGowI9Dr/I8HVaw4hTtnxy8g==}
+
+  unist-util-position@5.0.0:
+    resolution: {integrity: sha512-fucsC7HjXvkB5R3kTCO7kUjRdrS0BJt3M/FPxmHMBOm8JQi2BsHAHFsy27E0EolP8rp0NzXsJ+jNPyDWvOJZPA==}
+
+  unist-util-stringify-position@4.0.0:
+    resolution: {integrity: sha512-0ASV06AAoKCDkS2+xw5RXJywruurpbC4JZSm7nr7MOt1ojAzvyyaO+UxZf18j8FCF6kmzCZKcAgN/yu2gm2XgQ==}
+
+  unist-util-visit-parents@6.0.2:
+    resolution: {integrity: sha512-goh1s1TBrqSqukSc8wrjwWhL0hiJxgA8m4kFxGlQ+8FYQ3C/m11FcTs4YYem7V664AhHVvgoQLk890Ssdsr2IQ==}
+
+  unist-util-visit@5.1.0:
+    resolution: {integrity: sha512-m+vIdyeCOpdr/QeQCu2EzxX/ohgS8KbnPDgFni4dQsfSCtpz8UqDyY5GjRru8PDKuYn7Fq19j1CQ+nJSsGKOzg==}
+
   unplugin@2.3.11:
     resolution: {integrity: sha512-5uKD0nqiYVzlmCRs01Fhs2BdkEgBS3SAVP6ndrBsuK42iC2+JHyxM05Rm9G8+5mkmRtzMZGY8Ct5+mliZxU/Ww==}
     engines: {node: '>=18.12.0'}
@@ -3038,9 +3345,18 @@ packages:
     peerDependencies:
       react: ^16.8.0 || ^17.0.0 || ^18.0.0 || ^19.0.0
 
+  util-deprecate@1.0.2:
+    resolution: {integrity: sha512-EPD5q1uXyFxJpCrLnCc1nHnq3gOa6DZBocAIiI2TaSCA7VCJ1UJDMagCzIkXNsUYfD1daK//LTEQ8xiIbrHtcw==}
+
   utrie@1.0.2:
     resolution: {integrity: sha512-1MLa5ouZiOmQzUbjbu9VmjLzn1QLXBhwpUa7kdLUQK+KQ5KA9I1vk5U4YHe/X2Ch7PYnJfWuWT+VbuxbGwljhw==}
 
+  vfile-message@4.0.3:
+    resolution: {integrity: sha512-QTHzsGd1EhbZs4AsQ20JX1rC3cOlt/IWJruk893DfLRr57lcnOeMaWG4K0JrRta4mIJZKth2Au3mM3u03/JWKw==}
+
+  vfile@6.0.3:
+    resolution: {integrity: sha512-KzIbH/9tXat2u30jf+smMwFCsno4wHVdNmzFyL+T/L3UGqqk6JKfVqOFOZEpZSHADH1k40ab6NUIXZq422ov3Q==}
+
   vite@7.3.1:
     resolution: {integrity: sha512-w+N7Hifpc3gRjZ63vYBXA56dvvRlNWRczTdmCBBa+CotUzAPf5b7YMdMR/8CQoeYE5LX3W4wj6RYTgonm1b9DA==}
     engines: {node: ^20.19.0 || >=22.12.0}
@@ -3178,6 +3494,9 @@ packages:
   zod@4.3.6:
     resolution: {integrity: sha512-rftlrkhHZOcjDwkGlnUtZZkvaPHCsDATp4pGpuOOMDaTdDDXF91wuVDJoWoPsKX/3YPQ5fHuF3STjcYyKr+Qhg==}
 
+  zwitch@2.0.4:
+    resolution: {integrity: sha512-bXE4cR/kVZhKZX/RjPEflHaKVhUVl85noU3v6b8apfQEc1x4A+zBxjZ4lN8LqGd6WZ3dl98pY4o717VFmoPp+A==}
+
 snapshots:
 
   '@adobe/css-tools@4.4.4': {}
@@ -4419,6 +4738,11 @@ snapshots:
       '@tailwindcss/oxide-win32-arm64-msvc': 4.1.18
       '@tailwindcss/oxide-win32-x64-msvc': 4.1.18
 
+  '@tailwindcss/typography@0.5.19(tailwindcss@4.1.18)':
+    dependencies:
+      postcss-selector-parser: 6.0.10
+      tailwindcss: 4.1.18
+
   '@tailwindcss/vite@4.1.18(vite@7.3.1(@types/node@24.10.10)(jiti@2.6.1)(lightningcss@1.30.2)(tsx@4.21.0))':
     dependencies:
       '@tailwindcss/node': 4.1.18
@@ -4727,14 +5051,32 @@ snapshots:
       '@types/d3-transition': 3.0.9
       '@types/d3-zoom': 3.0.8
 
+  '@types/debug@4.1.13':
+    dependencies:
+      '@types/ms': 2.1.0
+
   '@types/deep-eql@4.0.2': {}
 
+  '@types/estree-jsx@1.0.5':
+    dependencies:
+      '@types/estree': 1.0.8
+
   '@types/estree@1.0.8': {}
 
   '@types/geojson@7946.0.16': {}
 
+  '@types/hast@3.0.4':
+    dependencies:
+      '@types/unist': 3.0.3
+
   '@types/json-schema@7.0.15': {}
 
+  '@types/mdast@4.0.4':
+    dependencies:
+      '@types/unist': 3.0.3
+
+  '@types/ms@2.1.0': {}
+
   '@types/node@24.10.10':
     dependencies:
       undici-types: 7.16.0
@@ -4747,6 +5089,10 @@ snapshots:
     dependencies:
       csstype: 3.2.3
 
+  '@types/unist@2.0.11': {}
+
+  '@types/unist@3.0.3': {}
+
   '@typescript-eslint/eslint-plugin@8.54.0(@typescript-eslint/parser@8.54.0(eslint@9.39.2(jiti@2.6.1))(typescript@5.9.3))(eslint@9.39.2(jiti@2.6.1))(typescript@5.9.3)':
     dependencies:
       '@eslint-community/regexpp': 4.12.2
@@ -4838,6 +5184,8 @@ snapshots:
       '@typescript-eslint/types': 8.54.0
       eslint-visitor-keys: 4.2.1
 
+  '@ungap/structured-clone@1.3.0': {}
+
   '@vitejs/plugin-react@5.1.3(vite@7.3.1(@types/node@24.10.10)(jiti@2.6.1)(lightningcss@1.30.2)(tsx@4.21.0))':
     dependencies:
       '@babel/core': 7.29.0
@@ -4954,6 +5302,8 @@ snapshots:
     transitivePeerDependencies:
       - supports-color
 
+  bail@2.0.2: {}
+
   balanced-match@1.0.2: {}
 
   base64-arraybuffer@1.0.2: {}
@@ -4996,6 +5346,8 @@ snapshots:
 
   caniuse-lite@1.0.30001768: {}
 
+  ccount@2.0.1: {}
+
   chai@6.2.2: {}
 
   chalk@4.1.2:
@@ -5003,6 +5355,14 @@ snapshots:
       ansi-styles: 4.3.0
       supports-color: 7.2.0
 
+  character-entities-html4@2.1.0: {}
+
+  character-entities-legacy@3.0.0: {}
+
+  character-entities@2.0.2: {}
+
+  character-reference-invalid@2.0.1: {}
+
   chokidar@3.6.0:
     dependencies:
       anymatch: 3.1.3
@@ -5031,6 +5391,8 @@ snapshots:
     dependencies:
       delayed-stream: 1.0.0
 
+  comma-separated-tokens@2.0.3: {}
+
   commander@7.2.0: {}
 
   concat-map@0.0.1: {}
@@ -5056,6 +5418,8 @@ snapshots:
 
   css.escape@1.5.1: {}
 
+  cssesc@3.0.0: {}
+
   csstype@3.2.3: {}
 
   d3-array@3.2.4:
@@ -5223,6 +5587,10 @@ snapshots:
 
   decimal.js@10.6.0: {}
 
+  decode-named-character-reference@1.3.0:
+    dependencies:
+      character-entities: 2.0.2
+
   deep-is@0.1.4: {}
 
   delaunator@5.0.1:
@@ -5237,6 +5605,10 @@ snapshots:
 
   detect-node-es@1.1.0: {}
 
+  devlop@1.1.0:
+    dependencies:
+      dequal: 2.0.3
+
   diff@8.0.3: {}
 
   dom-accessibility-api@0.5.16: {}
@@ -5308,6 +5680,8 @@ snapshots:
 
   escape-string-regexp@4.0.0: {}
 
+  escape-string-regexp@5.0.0: {}
+
   eslint-config-prettier@10.1.8(eslint@9.39.2(jiti@2.6.1)):
     dependencies:
       eslint: 9.39.2(jiti@2.6.1)
@@ -5395,6 +5769,8 @@ snapshots:
 
   estraverse@5.3.0: {}
 
+  estree-util-is-identifier-name@3.0.0: {}
+
   estree-walker@3.0.3:
     dependencies:
       '@types/estree': 1.0.8
@@ -5403,6 +5779,8 @@ snapshots:
 
   expect-type@1.3.0: {}
 
+  extend@3.0.2: {}
+
   fast-deep-equal@3.1.3: {}
 
   fast-json-stable-stringify@2.1.0: {}
@@ -5506,6 +5884,30 @@ snapshots:
     dependencies:
       function-bind: 1.1.2
 
+  hast-util-to-jsx-runtime@2.3.6:
+    dependencies:
+      '@types/estree': 1.0.8
+      '@types/hast': 3.0.4
+      '@types/unist': 3.0.3
+      comma-separated-tokens: 2.0.3
+      devlop: 1.1.0
+      estree-util-is-identifier-name: 3.0.0
+      hast-util-whitespace: 3.0.0
+      mdast-util-mdx-expression: 2.0.1
+      mdast-util-mdx-jsx: 3.2.0
+      mdast-util-mdxjs-esm: 2.0.1
+      property-information: 7.1.0
+      space-separated-tokens: 2.0.2
+      style-to-js: 1.1.21
+      unist-util-position: 5.0.0
+      vfile-message: 4.0.3
+    transitivePeerDependencies:
+      - supports-color
+
+  hast-util-whitespace@3.0.0:
+    dependencies:
+      '@types/hast': 3.0.4
+
   hermes-estree@0.25.1: {}
 
   hermes-parser@0.25.1:
@@ -5522,6 +5924,8 @@ snapshots:
     dependencies:
       void-elements: 3.1.0
 
+  html-url-attributes@3.0.1: {}
+
   html2canvas@1.4.1:
     dependencies:
       css-line-break: 2.1.0
@@ -5554,20 +5958,35 @@ snapshots:
 
   indent-string@4.0.0: {}
 
+  inline-style-parser@0.2.7: {}
+
   internmap@2.0.3: {}
 
+  is-alphabetical@2.0.1: {}
+
+  is-alphanumerical@2.0.1:
+    dependencies:
+      is-alphabetical: 2.0.1
+      is-decimal: 2.0.1
+
   is-binary-path@2.1.0:
     dependencies:
       binary-extensions: 2.3.0
 
+  is-decimal@2.0.1: {}
+
   is-extglob@2.1.1: {}
 
   is-glob@4.0.3:
     dependencies:
       is-extglob: 2.1.1
 
+  is-hexadecimal@2.0.1: {}
+
   is-number@7.0.0: {}
 
+  is-plain-obj@4.1.0: {}
+
   is-potential-custom-element-name@1.0.1: {}
 
   isbot@5.1.34: {}
@@ -5682,6 +6101,8 @@ snapshots:
 
   lodash.merge@4.6.2: {}
 
+  longest-streak@3.1.0: {}
+
   lru-cache@11.2.7: {}
 
   lru-cache@5.1.1:
@@ -5698,10 +6119,356 @@ snapshots:
     dependencies:
       '@jridgewell/sourcemap-codec': 1.5.5
 
+  markdown-table@3.0.4: {}
+
   math-intrinsics@1.1.0: {}
 
+  mdast-util-find-and-replace@3.0.2:
+    dependencies:
+      '@types/mdast': 4.0.4
+      escape-string-regexp: 5.0.0
+      unist-util-is: 6.0.1
+      unist-util-visit-parents: 6.0.2
+
+  mdast-util-from-markdown@2.0.3:
+    dependencies:
+      '@types/mdast': 4.0.4
+      '@types/unist': 3.0.3
+      decode-named-character-reference: 1.3.0
+      devlop: 1.1.0
+      mdast-util-to-string: 4.0.0
+      micromark: 4.0.2
+      micromark-util-decode-numeric-character-reference: 2.0.2
+      micromark-util-decode-string: 2.0.1
+      micromark-util-normalize-identifier: 2.0.1
+      micromark-util-symbol: 2.0.1
+      micromark-util-types: 2.0.2
+      unist-util-stringify-position: 4.0.0
+    transitivePeerDependencies:
+      - supports-color
+
+  mdast-util-gfm-autolink-literal@2.0.1:
+    dependencies:
+      '@types/mdast': 4.0.4
+      ccount: 2.0.1
+      devlop: 1.1.0
+      mdast-util-find-and-replace: 3.0.2
+      micromark-util-character: 2.1.1
+
+  mdast-util-gfm-footnote@2.1.0:
+    dependencies:
+      '@types/mdast': 4.0.4
+      devlop: 1.1.0
+      mdast-util-from-markdown: 2.0.3
+      mdast-util-to-markdown: 2.1.2
+      micromark-util-normalize-identifier: 2.0.1
+    transitivePeerDependencies:
+      - supports-color
+
+  mdast-util-gfm-strikethrough@2.0.0:
+    dependencies:
+      '@types/mdast': 4.0.4
+      mdast-util-from-markdown: 2.0.3
+      mdast-util-to-markdown: 2.1.2
+    transitivePeerDependencies:
+      - supports-color
+
+  mdast-util-gfm-table@2.0.0:
+    dependencies:
+      '@types/mdast': 4.0.4
+      devlop: 1.1.0
+      markdown-table: 3.0.4
+      mdast-util-from-markdown: 2.0.3
+      mdast-util-to-markdown: 2.1.2
+    transitivePeerDependencies:
+      - supports-color
+
+  mdast-util-gfm-task-list-item@2.0.0:
+    dependencies:
+      '@types/mdast': 4.0.4
+      devlop: 1.1.0
+      mdast-util-from-markdown: 2.0.3
+      mdast-util-to-markdown: 2.1.2
+    transitivePeerDependencies:
+      - supports-color
+
+  mdast-util-gfm@3.1.0:
+    dependencies:
+      mdast-util-from-markdown: 2.0.3
+      mdast-util-gfm-autolink-literal: 2.0.1
+      mdast-util-gfm-footnote: 2.1.0
+      mdast-util-gfm-strikethrough: 2.0.0
+      mdast-util-gfm-table: 2.0.0
+      mdast-util-gfm-task-list-item: 2.0.0
+      mdast-util-to-markdown: 2.1.2
+    transitivePeerDependencies:
+      - supports-color
+
+  mdast-util-mdx-expression@2.0.1:
+    dependencies:
+      '@types/estree-jsx': 1.0.5
+      '@types/hast': 3.0.4
+      '@types/mdast': 4.0.4
+      devlop: 1.1.0
+      mdast-util-from-markdown: 2.0.3
+      mdast-util-to-markdown: 2.1.2
+    transitivePeerDependencies:
+      - supports-color
+
+  mdast-util-mdx-jsx@3.2.0:
+    dependencies:
+      '@types/estree-jsx': 1.0.5
+      '@types/hast': 3.0.4
+      '@types/mdast': 4.0.4
+      '@types/unist': 3.0.3
+      ccount: 2.0.1
+      devlop: 1.1.0
+      mdast-util-from-markdown: 2.0.3
+      mdast-util-to-markdown: 2.1.2
+      parse-entities: 4.0.2
+      stringify-entities: 4.0.4
+      unist-util-stringify-position: 4.0.0
+      vfile-message: 4.0.3
+    transitivePeerDependencies:
+      - supports-color
+
+  mdast-util-mdxjs-esm@2.0.1:
+    dependencies:
+      '@types/estree-jsx': 1.0.5
+      '@types/hast': 3.0.4
+      '@types/mdast': 4.0.4
+      devlop: 1.1.0
+      mdast-util-from-markdown: 2.0.3
+      mdast-util-to-markdown: 2.1.2
+    transitivePeerDependencies:
+      - supports-color
+
+  mdast-util-phrasing@4.1.0:
+    dependencies:
+      '@types/mdast': 4.0.4
+      unist-util-is: 6.0.1
+
+  mdast-util-to-hast@13.2.1:
+    dependencies:
+      '@types/hast': 3.0.4
+      '@types/mdast': 4.0.4
+      '@ungap/structured-clone': 1.3.0
+      devlop: 1.1.0
+      micromark-util-sanitize-uri: 2.0.1
+      trim-lines: 3.0.1
+      unist-util-position: 5.0.0
+      unist-util-visit: 5.1.0
+      vfile: 6.0.3
+
+  mdast-util-to-markdown@2.1.2:
+    dependencies:
+      '@types/mdast': 4.0.4
+      '@types/unist': 3.0.3
+      longest-streak: 3.1.0
+      mdast-util-phrasing: 4.1.0
+      mdast-util-to-string: 4.0.0
+      micromark-util-classify-character: 2.0.1
+      micromark-util-decode-string: 2.0.1
+      unist-util-visit: 5.1.0
+      zwitch: 2.0.4
+
+  mdast-util-to-string@4.0.0:
+    dependencies:
+      '@types/mdast': 4.0.4
+
   mdn-data@2.27.1: {}
 
+  micromark-core-commonmark@2.0.3:
+    dependencies:
+      decode-named-character-reference: 1.3.0
+      devlop: 1.1.0
+      micromark-factory-destination: 2.0.1
+      micromark-factory-label: 2.0.1
+      micromark-factory-space: 2.0.1
+      micromark-factory-title: 2.0.1
+      micromark-factory-whitespace: 2.0.1
+      micromark-util-character: 2.1.1
+      micromark-util-chunked: 2.0.1
+      micromark-util-classify-character: 2.0.1
+      micromark-util-html-tag-name: 2.0.1
+      micromark-util-normalize-identifier: 2.0.1
+      micromark-util-resolve-all: 2.0.1
+      micromark-util-subtokenize: 2.1.0
+      micromark-util-symbol: 2.0.1
+      micromark-util-types: 2.0.2
+
+  micromark-extension-gfm-autolink-literal@2.1.0:
+    dependencies:
+      micromark-util-character: 2.1.1
+      micromark-util-sanitize-uri: 2.0.1
+      micromark-util-symbol: 2.0.1
+      micromark-util-types: 2.0.2
+
+  micromark-extension-gfm-footnote@2.1.0:
+    dependencies:
+      devlop: 1.1.0
+      micromark-core-commonmark: 2.0.3
+      micromark-factory-space: 2.0.1
+      micromark-util-character: 2.1.1
+      micromark-util-normalize-identifier: 2.0.1
+      micromark-util-sanitize-uri: 2.0.1
+      micromark-util-symbol: 2.0.1
+      micromark-util-types: 2.0.2
+
+  micromark-extension-gfm-strikethrough@2.1.0:
+    dependencies:
+      devlop: 1.1.0
+      micromark-util-chunked: 2.0.1
+      micromark-util-classify-character: 2.0.1
+      micromark-util-resolve-all: 2.0.1
+      micromark-util-symbol: 2.0.1
+      micromark-util-types: 2.0.2
+
+  micromark-extension-gfm-table@2.1.1:
+    dependencies:
+      devlop: 1.1.0
+      micromark-factory-space: 2.0.1
+      micromark-util-character: 2.1.1
+      micromark-util-symbol: 2.0.1
+      micromark-util-types: 2.0.2
+
+  micromark-extension-gfm-tagfilter@2.0.0:
+    dependencies:
+      micromark-util-types: 2.0.2
+
+  micromark-extension-gfm-task-list-item@2.1.0:
+    dependencies:
+      devlop: 1.1.0
+      micromark-factory-space: 2.0.1
+      micromark-util-character: 2.1.1
+      micromark-util-symbol: 2.0.1
+      micromark-util-types: 2.0.2
+
+  micromark-extension-gfm@3.0.0:
+    dependencies:
+      micromark-extension-gfm-autolink-literal: 2.1.0
+      micromark-extension-gfm-footnote: 2.1.0
+      micromark-extension-gfm-strikethrough: 2.1.0
+      micromark-extension-gfm-table: 2.1.1
+      micromark-extension-gfm-tagfilter: 2.0.0
+      micromark-extension-gfm-task-list-item: 2.1.0
+      micromark-util-combine-extensions: 2.0.1
+      micromark-util-types: 2.0.2
+
+  micromark-factory-destination@2.0.1:
+    dependencies:
+      micromark-util-character: 2.1.1
+      micromark-util-symbol: 2.0.1
+      micromark-util-types: 2.0.2
+
+  micromark-factory-label@2.0.1:
+    dependencies:
+      devlop: 1.1.0
+      micromark-util-character: 2.1.1
+      micromark-util-symbol: 2.0.1
+      micromark-util-types: 2.0.2
+
+  micromark-factory-space@2.0.1:
+    dependencies:
+      micromark-util-character: 2.1.1
+      micromark-util-types: 2.0.2
+
+  micromark-factory-title@2.0.1:
+    dependencies:
+      micromark-factory-space: 2.0.1
+      micromark-util-character: 2.1.1
+      micromark-util-symbol: 2.0.1
+      micromark-util-types: 2.0.2
+
+  micromark-factory-whitespace@2.0.1:
+    dependencies:
+      micromark-factory-space: 2.0.1
+      micromark-util-character: 2.1.1
+      micromark-util-symbol: 2.0.1
+      micromark-util-types: 2.0.2
+
+  micromark-util-character@2.1.1:
+    dependencies:
+      micromark-util-symbol: 2.0.1
+      micromark-util-types: 2.0.2
+
+  micromark-util-chunked@2.0.1:
+    dependencies:
+      micromark-util-symbol: 2.0.1
+
+  micromark-util-classify-character@2.0.1:
+    dependencies:
+      micromark-util-character: 2.1.1
+      micromark-util-symbol: 2.0.1
+      micromark-util-types: 2.0.2
+
+  micromark-util-combine-extensions@2.0.1:
+    dependencies:
+      micromark-util-chunked: 2.0.1
+      micromark-util-types: 2.0.2
+
+  micromark-util-decode-numeric-character-reference@2.0.2:
+    dependencies:
+      micromark-util-symbol: 2.0.1
+
+  micromark-util-decode-string@2.0.1:
+    dependencies:
+      decode-named-character-reference: 1.3.0
+      micromark-util-character: 2.1.1
+      micromark-util-decode-numeric-character-reference: 2.0.2
+      micromark-util-symbol: 2.0.1
+
+  micromark-util-encode@2.0.1: {}
+
+  micromark-util-html-tag-name@2.0.1: {}
+
+  micromark-util-normalize-identifier@2.0.1:
+    dependencies:
+      micromark-util-symbol: 2.0.1
+
+  micromark-util-resolve-all@2.0.1:
+    dependencies:
+      micromark-util-types: 2.0.2
+
+  micromark-util-sanitize-uri@2.0.1:
+    dependencies:
+      micromark-util-character: 2.1.1
+      micromark-util-encode: 2.0.1
+      micromark-util-symbol: 2.0.1
+
+  micromark-util-subtokenize@2.1.0:
+    dependencies:
+      devlop: 1.1.0
+      micromark-util-chunked: 2.0.1
+      micromark-util-symbol: 2.0.1
+      micromark-util-types: 2.0.2
+
+  micromark-util-symbol@2.0.1: {}
+
+  micromark-util-types@2.0.2: {}
+
+  micromark@4.0.2:
+    dependencies:
+      '@types/debug': 4.1.13
+      debug: 4.4.3
+      decode-named-character-reference: 1.3.0
+      devlop: 1.1.0
+      micromark-core-commonmark: 2.0.3
+      micromark-factory-space: 2.0.1
+      micromark-util-character: 2.1.1
+      micromark-util-chunked: 2.0.1
+      micromark-util-combine-extensions: 2.0.1
+      micromark-util-decode-numeric-character-reference: 2.0.2
+      micromark-util-encode: 2.0.1
+      micromark-util-normalize-identifier: 2.0.1
+      micromark-util-resolve-all: 2.0.1
+      micromark-util-sanitize-uri: 2.0.1
+      micromark-util-subtokenize: 2.1.0
+      micromark-util-symbol: 2.0.1
+      micromark-util-types: 2.0.2
+    transitivePeerDependencies:
+      - supports-color
+
   mime-db@1.52.0: {}
 
   mime-types@2.1.35:
@@ -5751,6 +6518,16 @@ snapshots:
     dependencies:
       callsites: 3.1.0
 
+  parse-entities@4.0.2:
+    dependencies:
+      '@types/unist': 2.0.11
+      character-entities-legacy: 3.0.0
+      character-reference-invalid: 2.0.1
+      decode-named-character-reference: 1.3.0
+      is-alphanumerical: 2.0.1
+      is-decimal: 2.0.1
+      is-hexadecimal: 2.0.1
+
   parse5@8.0.0:
     dependencies:
       entities: 6.0.1
@@ -5767,6 +6544,11 @@ snapshots:
 
   picomatch@4.0.3: {}
 
+  postcss-selector-parser@6.0.10:
+    dependencies:
+      cssesc: 3.0.0
+      util-deprecate: 1.0.2
+
   postcss@8.5.6:
     dependencies:
       nanoid: 3.3.11
@@ -5783,6 +6565,8 @@ snapshots:
       ansi-styles: 5.2.0
       react-is: 17.0.2
 
+  property-information@7.1.0: {}
+
   proxy-from-env@1.1.0: {}
 
   punycode@2.3.1: {}
@@ -5868,6 +6652,24 @@ snapshots:
 
   react-is@17.0.2: {}
 
+  react-markdown@10.1.0(@types/react@19.2.11)(react@19.2.4):
+    dependencies:
+      '@types/hast': 3.0.4
+      '@types/mdast': 4.0.4
+      '@types/react': 19.2.11
+      devlop: 1.1.0
+      hast-util-to-jsx-runtime: 2.3.6
+      html-url-attributes: 3.0.1
+      mdast-util-to-hast: 13.2.1
+      react: 19.2.4
+      remark-parse: 11.0.0
+      remark-rehype: 11.1.2
+      unified: 11.0.5
+      unist-util-visit: 5.1.0
+      vfile: 6.0.3
+    transitivePeerDependencies:
+      - supports-color
+
   react-refresh@0.18.0: {}
 
   react-remove-scroll-bar@2.3.8(@types/react@19.2.11)(react@19.2.4):
@@ -5916,6 +6718,40 @@ snapshots:
       indent-string: 4.0.0
       strip-indent: 3.0.0
 
+  remark-gfm@4.0.1:
+    dependencies:
+      '@types/mdast': 4.0.4
+      mdast-util-gfm: 3.1.0
+      micromark-extension-gfm: 3.0.0
+      remark-parse: 11.0.0
+      remark-stringify: 11.0.0
+      unified: 11.0.5
+    transitivePeerDependencies:
+      - supports-color
+
+  remark-parse@11.0.0:
+    dependencies:
+      '@types/mdast': 4.0.4
+      mdast-util-from-markdown: 2.0.3
+      micromark-util-types: 2.0.2
+      unified: 11.0.5
+    transitivePeerDependencies:
+      - supports-color
+
+  remark-rehype@11.1.2:
+    dependencies:
+      '@types/hast': 3.0.4
+      '@types/mdast': 4.0.4
+      mdast-util-to-hast: 13.2.1
+      unified: 11.0.5
+      vfile: 6.0.3
+
+  remark-stringify@11.0.0:
+    dependencies:
+      '@types/mdast': 4.0.4
+      mdast-util-to-markdown: 2.1.2
+      unified: 11.0.5
+
   require-from-string@2.0.2: {}
 
   resolve-from@4.0.0: {}
@@ -5989,16 +6825,31 @@ snapshots:
 
   source-map@0.7.6: {}
 
+  space-separated-tokens@2.0.2: {}
+
   stackback@0.0.2: {}
 
   std-env@3.10.0: {}
 
+  stringify-entities@4.0.4:
+    dependencies:
+      character-entities-html4: 2.1.0
+      character-entities-legacy: 3.0.0
+
   strip-indent@3.0.0:
     dependencies:
       min-indent: 1.0.1
 
   strip-json-comments@3.1.1: {}
 
+  style-to-js@1.1.21:
+    dependencies:
+      style-to-object: 1.0.14
+
+  style-to-object@1.0.14:
+    dependencies:
+      inline-style-parser: 0.2.7
+
   supports-color@7.2.0:
     dependencies:
       has-flag: 4.0.0
@@ -6048,6 +6899,10 @@ snapshots:
     dependencies:
       punycode: 2.3.1
 
+  trim-lines@3.0.1: {}
+
+  trough@2.2.0: {}
+
   ts-api-utils@2.4.0(typescript@5.9.3):
     dependencies:
       typescript: 5.9.3
@@ -6082,6 +6937,39 @@ snapshots:
 
   undici@7.24.4: {}
 
+  unified@11.0.5:
+    dependencies:
+      '@types/unist': 3.0.3
+      bail: 2.0.2
+      devlop: 1.1.0
+      extend: 3.0.2
+      is-plain-obj: 4.1.0
+      trough: 2.2.0
+      vfile: 6.0.3
+
+  unist-util-is@6.0.1:
+    dependencies:
+      '@types/unist': 3.0.3
+
+  unist-util-position@5.0.0:
+    dependencies:
+      '@types/unist': 3.0.3
+
+  unist-util-stringify-position@4.0.0:
+    dependencies:
+      '@types/unist': 3.0.3
+
+  unist-util-visit-parents@6.0.2:
+    dependencies:
+      '@types/unist': 3.0.3
+      unist-util-is: 6.0.1
+
+  unist-util-visit@5.1.0:
+    dependencies:
+      '@types/unist': 3.0.3
+      unist-util-is: 6.0.1
+      unist-util-visit-parents: 6.0.2
+
   unplugin@2.3.11:
     dependencies:
       '@jridgewell/remapping': 2.3.5
@@ -6118,10 +7006,22 @@ snapshots:
     dependencies:
       react: 19.2.4
 
+  util-deprecate@1.0.2: {}
+
   utrie@1.0.2:
     dependencies:
       base64-arraybuffer: 1.0.2
 
+  vfile-message@4.0.3:
+    dependencies:
+      '@types/unist': 3.0.3
+      unist-util-stringify-position: 4.0.0
+
+  vfile@6.0.3:
+    dependencies:
+      '@types/unist': 3.0.3
+      vfile-message: 4.0.3
+
   vite@7.3.1(@types/node@24.10.10)(jiti@2.6.1)(lightningcss@1.30.2)(tsx@4.21.0):
     dependencies:
       esbuild: 0.27.2
@@ -6221,3 +7121,5 @@ snapshots:
   zod@3.25.76: {}
 
   zod@4.3.6: {}
+
+  zwitch@2.0.4: {}
diff --git a/frontend/src/features/ontology/types/__tests__/types.test.ts b/frontend/src/features/ontology/types/__tests__/types.test.ts
new file mode 100644
index 0000000..c8d0037
--- /dev/null
+++ b/frontend/src/features/ontology/types/__tests__/types.test.ts
@@ -0,0 +1,40 @@
+import { describe, expect, it } from "vitest";
+import { toVerificationStatus } from "../index";
+
+describe("toVerificationStatus", () => {
+  it("returns 'unknown' for null input", () => {
+    expect(toVerificationStatus(null)).toBe("unknown");
+  });
+
+  it("returns 'verified' for 'verified' input", () => {
+    expect(toVerificationStatus("verified")).toBe("verified");
+  });
+
+  it("returns 'unknown' for an unrecognized string like 'banana'", () => {
+    expect(toVerificationStatus("banana")).toBe("unknown");
+  });
+
+  it("returns 'partially-verified' for 'partially-verified' input", () => {
+    expect(toVerificationStatus("partially-verified")).toBe("partially-verified");
+  });
+
+  it("returns 'needs-correction' for 'needs-correction' input", () => {
+    expect(toVerificationStatus("needs-correction")).toBe("needs-correction");
+  });
+
+  it("returns 'structure-verified' for 'structure-verified' input", () => {
+    expect(toVerificationStatus("structure-verified")).toBe("structure-verified");
+  });
+
+  it("returns 'corrected' for 'corrected' input", () => {
+    expect(toVerificationStatus("corrected")).toBe("corrected");
+  });
+
+  it("returns 'unverified' for 'unverified' input", () => {
+    expect(toVerificationStatus("unverified")).toBe("unverified");
+  });
+
+  it("returns 'unknown' for empty string", () => {
+    expect(toVerificationStatus("")).toBe("unknown");
+  });
+});
diff --git a/frontend/src/features/ontology/types/index.ts b/frontend/src/features/ontology/types/index.ts
index e354f1d..5a8f663 100644
--- a/frontend/src/features/ontology/types/index.ts
+++ b/frontend/src/features/ontology/types/index.ts
@@ -7,6 +7,11 @@ export interface Framework {
   source_url: string | null;
   created_at: string;
   updated_at: string;
+  // Verification provenance (returned by the API, added in split 07)
+  verification_status: string | null;
+  verification_date: string | null;
+  verification_source: string | null;
+  verification_notes: string | null;
 }
 
 export interface Concept {
@@ -193,3 +198,48 @@ export interface LandscapeProfile {
   activities: string[];
   applicableFrameworks: string[];
 }
+
+// Verification Provenance Types (split 07)
+export type VerificationStatus =
+  | "verified"
+  | "partially-verified"
+  | "structure-verified"
+  | "corrected"
+  | "unverified"
+  | "needs-correction";
+
+const KNOWN_STATUSES: ReadonlySet<string> = new Set<VerificationStatus>([
+  "verified",
+  "partially-verified",
+  "structure-verified",
+  "corrected",
+  "unverified",
+  "needs-correction",
+]);
+
+/**
+ * Normalizes a raw backend verification_status string to a typed
+ * VerificationStatus or "unknown" for null/unrecognized values.
+ * Used by VerificationBadge and ProofPanel for safe style mapping.
+ */
+export function toVerificationStatus(
+  value: string | null,
+): VerificationStatus | "unknown" {
+  if (value !== null && KNOWN_STATUSES.has(value)) {
+    return value as VerificationStatus;
+  }
+  return "unknown";
+}
+
+/**
+ * Response shape of GET /api/ontology/frameworks/{id}/proof
+ * Used by useFrameworkProof hook and ProofPanel component.
+ */
+export interface FrameworkProof {
+  framework_id: string;
+  verification_status: string | null;
+  verification_date: string | null;
+  verification_source: string | null;
+  verification_notes: string | null;
+  proof_content: string | null; // raw markdown; null if no proof file exists
+}
diff --git a/frontend/src/index.css b/frontend/src/index.css
index a89bcc5..95cdcc7 100644
--- a/frontend/src/index.css
+++ b/frontend/src/index.css
@@ -1,5 +1,6 @@
 @import url('https://fonts.googleapis.com/css2?family=IBM+Plex+Mono:wght@400;500;600;700&family=IBM+Plex+Sans:wght@300;400;500;600&display=swap');
 @import "tailwindcss";
+@plugin "@tailwindcss/typography";
 
 @theme {
   /* Technical Cartography Theme - Deep Slate with Amber Accents */
