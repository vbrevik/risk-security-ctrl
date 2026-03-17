diff --git a/docs/plans/frontend-pages/implementation/deep_implement_config.json b/docs/plans/frontend-pages/implementation/deep_implement_config.json
index cff0229..4380944 100644
--- a/docs/plans/frontend-pages/implementation/deep_implement_config.json
+++ b/docs/plans/frontend-pages/implementation/deep_implement_config.json
@@ -23,5 +23,5 @@
     "may_modify_files": false,
     "detected_formatters": []
   },
-  "created_at": "2026-03-17T12:36:10.662387+00:00"
+  "created_at": "2026-03-17T12:38:27.712338+00:00"
 }
\ No newline at end of file
diff --git a/frontend/package.json b/frontend/package.json
index 98d2dfc..972d1bb 100644
--- a/frontend/package.json
+++ b/frontend/package.json
@@ -36,6 +36,9 @@
     "@eslint/js": "^9.39.1",
     "@tanstack/router-devtools": "^1.158.1",
     "@tanstack/router-plugin": "^1.158.1",
+    "@testing-library/jest-dom": "^6.9.1",
+    "@testing-library/react": "^16.3.2",
+    "@testing-library/user-event": "^14.6.1",
     "@types/d3": "^7.4.3",
     "@types/node": "^24.10.1",
     "@types/react": "^19.2.5",
@@ -46,6 +49,7 @@
     "eslint-plugin-react-hooks": "^7.0.1",
     "eslint-plugin-react-refresh": "^0.4.24",
     "globals": "^16.5.0",
+    "jsdom": "^29.0.0",
     "prettier": "^3.8.1",
     "typescript": "~5.9.3",
     "typescript-eslint": "^8.46.4",
diff --git a/frontend/pnpm-lock.yaml b/frontend/pnpm-lock.yaml
index 49c67d4..c7f11cb 100644
--- a/frontend/pnpm-lock.yaml
+++ b/frontend/pnpm-lock.yaml
@@ -72,6 +72,15 @@ importers:
       '@tanstack/router-plugin':
         specifier: ^1.158.1
         version: 1.158.1(@tanstack/react-router@1.158.1(react-dom@19.2.4(react@19.2.4))(react@19.2.4))(vite@7.3.1(@types/node@24.10.10)(jiti@2.6.1)(lightningcss@1.30.2)(tsx@4.21.0))
+      '@testing-library/jest-dom':
+        specifier: ^6.9.1
+        version: 6.9.1
+      '@testing-library/react':
+        specifier: ^16.3.2
+        version: 16.3.2(@testing-library/dom@10.4.1)(@types/react-dom@19.2.3(@types/react@19.2.11))(@types/react@19.2.11)(react-dom@19.2.4(react@19.2.4))(react@19.2.4)
+      '@testing-library/user-event':
+        specifier: ^14.6.1
+        version: 14.6.1(@testing-library/dom@10.4.1)
       '@types/d3':
         specifier: ^7.4.3
         version: 7.4.3
@@ -102,6 +111,9 @@ importers:
       globals:
         specifier: ^16.5.0
         version: 16.5.0
+      jsdom:
+        specifier: ^29.0.0
+        version: 29.0.0
       prettier:
         specifier: ^3.8.1
         version: 3.8.1
@@ -116,10 +128,24 @@ importers:
         version: 7.3.1(@types/node@24.10.10)(jiti@2.6.1)(lightningcss@1.30.2)(tsx@4.21.0)
       vitest:
         specifier: ^4.0.18
-        version: 4.0.18(@types/node@24.10.10)(jiti@2.6.1)(lightningcss@1.30.2)(tsx@4.21.0)
+        version: 4.0.18(@types/node@24.10.10)(jiti@2.6.1)(jsdom@29.0.0)(lightningcss@1.30.2)(tsx@4.21.0)
 
 packages:
 
+  '@adobe/css-tools@4.4.4':
+    resolution: {integrity: sha512-Elp+iwUx5rN5+Y8xLt5/GRoG20WGoDCQ/1Fb+1LiGtvwbDavuSk0jhD/eZdckHAuzcDzccnkv+rEjyWfRx18gg==}
+
+  '@asamuzakjp/css-color@5.0.1':
+    resolution: {integrity: sha512-2SZFvqMyvboVV1d15lMf7XiI3m7SDqXUuKaTymJYLN6dSGadqp+fVojqJlVoMlbZnlTmu3S0TLwLTJpvBMO1Aw==}
+    engines: {node: ^20.19.0 || ^22.12.0 || >=24.0.0}
+
+  '@asamuzakjp/dom-selector@7.0.3':
+    resolution: {integrity: sha512-Q6mU0Z6bfj6YvnX2k9n0JxiIwrCFN59x/nWmYQnAqP000ruX/yV+5bp/GRcF5T8ncvfwJQ7fgfP74DlpKExILA==}
+    engines: {node: ^20.19.0 || ^22.12.0 || >=24.0.0}
+
+  '@asamuzakjp/nwsapi@2.3.9':
+    resolution: {integrity: sha512-n8GuYSrI9bF7FFZ/SjhwevlHc8xaVlb/7HmHelnc/PZXBD2ZR49NnN9sMMuDdEGPeeRQ5d0hqlSlEpgCX3Wl0Q==}
+
   '@babel/code-frame@7.29.0':
     resolution: {integrity: sha512-9NhCeYjq9+3uxgdtp20LSiJXJvN0FeCtNGpJxuMFZ1Kv3cWUNb6DOhJwUvcVCzKGR66cw4njwM6hrJLqgOwbcw==}
     engines: {node: '>=6.9.0'}
@@ -219,6 +245,46 @@ packages:
     resolution: {integrity: sha512-LwdZHpScM4Qz8Xw2iKSzS+cfglZzJGvofQICy7W7v4caru4EaAmyUuO6BGrbyQ2mYV11W0U8j5mBhd14dd3B0A==}
     engines: {node: '>=6.9.0'}
 
+  '@bramus/specificity@2.4.2':
+    resolution: {integrity: sha512-ctxtJ/eA+t+6q2++vj5j7FYX3nRu311q1wfYH3xjlLOsczhlhxAg2FWNUXhpGvAw3BWo1xBcvOV6/YLc2r5FJw==}
+    hasBin: true
+
+  '@csstools/color-helpers@6.0.2':
+    resolution: {integrity: sha512-LMGQLS9EuADloEFkcTBR3BwV/CGHV7zyDxVRtVDTwdI2Ca4it0CCVTT9wCkxSgokjE5Ho41hEPgb8OEUwoXr6Q==}
+    engines: {node: '>=20.19.0'}
+
+  '@csstools/css-calc@3.1.1':
+    resolution: {integrity: sha512-HJ26Z/vmsZQqs/o3a6bgKslXGFAungXGbinULZO3eMsOyNJHeBBZfup5FiZInOghgoM4Hwnmw+OgbJCNg1wwUQ==}
+    engines: {node: '>=20.19.0'}
+    peerDependencies:
+      '@csstools/css-parser-algorithms': ^4.0.0
+      '@csstools/css-tokenizer': ^4.0.0
+
+  '@csstools/css-color-parser@4.0.2':
+    resolution: {integrity: sha512-0GEfbBLmTFf0dJlpsNU7zwxRIH0/BGEMuXLTCvFYxuL1tNhqzTbtnFICyJLTNK4a+RechKP75e7w42ClXSnJQw==}
+    engines: {node: '>=20.19.0'}
+    peerDependencies:
+      '@csstools/css-parser-algorithms': ^4.0.0
+      '@csstools/css-tokenizer': ^4.0.0
+
+  '@csstools/css-parser-algorithms@4.0.0':
+    resolution: {integrity: sha512-+B87qS7fIG3L5h3qwJ/IFbjoVoOe/bpOdh9hAjXbvx0o8ImEmUsGXN0inFOnk2ChCFgqkkGFQ+TpM5rbhkKe4w==}
+    engines: {node: '>=20.19.0'}
+    peerDependencies:
+      '@csstools/css-tokenizer': ^4.0.0
+
+  '@csstools/css-syntax-patches-for-csstree@1.1.1':
+    resolution: {integrity: sha512-BvqN0AMWNAnLk9G8jnUT77D+mUbY/H2b3uDTvg2isJkHaOufUE2R3AOwxWo7VBQKT1lOdwdvorddo2B/lk64+w==}
+    peerDependencies:
+      css-tree: ^3.2.1
+    peerDependenciesMeta:
+      css-tree:
+        optional: true
+
+  '@csstools/css-tokenizer@4.0.0':
+    resolution: {integrity: sha512-QxULHAm7cNu72w97JUNCBFODFaXpbDg+dP8b/oWFAZ2MTRppA3U00Y2L1HqaS4J6yBqxwa/Y3nMBaxVKbB/NsA==}
+    engines: {node: '>=20.19.0'}
+
   '@esbuild/aix-ppc64@0.27.2':
     resolution: {integrity: sha512-GZMB+a0mOMZs4MpDbj8RJp4cw+w1WV5NYD6xzgvzUJ5Ek2jerwfO2eADyI6ExDSUED+1X8aMbegahsJi+8mgpw==}
     engines: {node: '>=18'}
@@ -413,6 +479,15 @@ packages:
     resolution: {integrity: sha512-43/qtrDUokr7LJqoF2c3+RInu/t4zfrpYdoSDfYyhg52rwLV6TnOvdG4fXm7IkSB3wErkcmJS9iEhjVtOSEjjA==}
     engines: {node: ^18.18.0 || ^20.9.0 || >=21.1.0}
 
+  '@exodus/bytes@1.15.0':
+    resolution: {integrity: sha512-UY0nlA+feH81UGSHv92sLEPLCeZFjXOuHhrIo0HQydScuQc8s0A7kL/UdgwgDq8g8ilksmuoF35YVTNphV2aBQ==}
+    engines: {node: ^20.19.0 || ^22.12.0 || >=24.0.0}
+    peerDependencies:
+      '@noble/hashes': ^1.8.0 || ^2.0.0
+    peerDependenciesMeta:
+      '@noble/hashes':
+        optional: true
+
   '@floating-ui/core@1.7.4':
     resolution: {integrity: sha512-C3HlIdsBxszvm5McXlB8PeOEWfBhcGBTZGkGlWc2U0KFY5IwG5OQEuQ8rq52DZmcHDlPLd+YFBK+cZcytwIFWg==}
 
@@ -1196,79 +1271,66 @@ packages:
     resolution: {integrity: sha512-F8sWbhZ7tyuEfsmOxwc2giKDQzN3+kuBLPwwZGyVkLlKGdV1nvnNwYD0fKQ8+XS6hp9nY7B+ZeK01EBUE7aHaw==}
     cpu: [arm]
     os: [linux]
-    libc: [glibc]
 
   '@rollup/rollup-linux-arm-musleabihf@4.57.1':
     resolution: {integrity: sha512-rGfNUfn0GIeXtBP1wL5MnzSj98+PZe/AXaGBCRmT0ts80lU5CATYGxXukeTX39XBKsxzFpEeK+Mrp9faXOlmrw==}
     cpu: [arm]
     os: [linux]
-    libc: [musl]
 
   '@rollup/rollup-linux-arm64-gnu@4.57.1':
     resolution: {integrity: sha512-MMtej3YHWeg/0klK2Qodf3yrNzz6CGjo2UntLvk2RSPlhzgLvYEB3frRvbEF2wRKh1Z2fDIg9KRPe1fawv7C+g==}
     cpu: [arm64]
     os: [linux]
-    libc: [glibc]
 
   '@rollup/rollup-linux-arm64-musl@4.57.1':
     resolution: {integrity: sha512-1a/qhaaOXhqXGpMFMET9VqwZakkljWHLmZOX48R0I/YLbhdxr1m4gtG1Hq7++VhVUmf+L3sTAf9op4JlhQ5u1Q==}
     cpu: [arm64]
     os: [linux]
-    libc: [musl]
 
   '@rollup/rollup-linux-loong64-gnu@4.57.1':
     resolution: {integrity: sha512-QWO6RQTZ/cqYtJMtxhkRkidoNGXc7ERPbZN7dVW5SdURuLeVU7lwKMpo18XdcmpWYd0qsP1bwKPf7DNSUinhvA==}
     cpu: [loong64]
     os: [linux]
-    libc: [glibc]
 
   '@rollup/rollup-linux-loong64-musl@4.57.1':
     resolution: {integrity: sha512-xpObYIf+8gprgWaPP32xiN5RVTi/s5FCR+XMXSKmhfoJjrpRAjCuuqQXyxUa/eJTdAE6eJ+KDKaoEqjZQxh3Gw==}
     cpu: [loong64]
     os: [linux]
-    libc: [musl]
 
   '@rollup/rollup-linux-ppc64-gnu@4.57.1':
     resolution: {integrity: sha512-4BrCgrpZo4hvzMDKRqEaW1zeecScDCR+2nZ86ATLhAoJ5FQ+lbHVD3ttKe74/c7tNT9c6F2viwB3ufwp01Oh2w==}
     cpu: [ppc64]
     os: [linux]
-    libc: [glibc]
 
   '@rollup/rollup-linux-ppc64-musl@4.57.1':
     resolution: {integrity: sha512-NOlUuzesGauESAyEYFSe3QTUguL+lvrN1HtwEEsU2rOwdUDeTMJdO5dUYl/2hKf9jWydJrO9OL/XSSf65R5+Xw==}
     cpu: [ppc64]
     os: [linux]
-    libc: [musl]
 
   '@rollup/rollup-linux-riscv64-gnu@4.57.1':
     resolution: {integrity: sha512-ptA88htVp0AwUUqhVghwDIKlvJMD/fmL/wrQj99PRHFRAG6Z5nbWoWG4o81Nt9FT+IuqUQi+L31ZKAFeJ5Is+A==}
     cpu: [riscv64]
     os: [linux]
-    libc: [glibc]
 
   '@rollup/rollup-linux-riscv64-musl@4.57.1':
     resolution: {integrity: sha512-S51t7aMMTNdmAMPpBg7OOsTdn4tySRQvklmL3RpDRyknk87+Sp3xaumlatU+ppQ+5raY7sSTcC2beGgvhENfuw==}
     cpu: [riscv64]
     os: [linux]
-    libc: [musl]
 
   '@rollup/rollup-linux-s390x-gnu@4.57.1':
     resolution: {integrity: sha512-Bl00OFnVFkL82FHbEqy3k5CUCKH6OEJL54KCyx2oqsmZnFTR8IoNqBF+mjQVcRCT5sB6yOvK8A37LNm/kPJiZg==}
     cpu: [s390x]
     os: [linux]
-    libc: [glibc]
 
   '@rollup/rollup-linux-x64-gnu@4.57.1':
     resolution: {integrity: sha512-ABca4ceT4N+Tv/GtotnWAeXZUZuM/9AQyCyKYyKnpk4yoA7QIAuBt6Hkgpw8kActYlew2mvckXkvx0FfoInnLg==}
     cpu: [x64]
     os: [linux]
-    libc: [glibc]
 
   '@rollup/rollup-linux-x64-musl@4.57.1':
     resolution: {integrity: sha512-HFps0JeGtuOR2convgRRkHCekD7j+gdAuXM+/i6kGzQtFhlCtQkpwtNzkNj6QhCDp7DRJ7+qC/1Vg2jt5iSOFw==}
     cpu: [x64]
     os: [linux]
-    libc: [musl]
 
   '@rollup/rollup-openbsd-x64@4.57.1':
     resolution: {integrity: sha512-H+hXEv9gdVQuDTgnqD+SQffoWoc0Of59AStSzTEj/feWTBAnSfSD3+Dql1ZruJQxmykT/JVY0dE8Ka7z0DH1hw==}
@@ -1341,28 +1403,24 @@ packages:
     engines: {node: '>= 10'}
     cpu: [arm64]
     os: [linux]
-    libc: [glibc]
 
   '@tailwindcss/oxide-linux-arm64-musl@4.1.18':
     resolution: {integrity: sha512-1px92582HkPQlaaCkdRcio71p8bc8i/ap5807tPRDK/uw953cauQBT8c5tVGkOwrHMfc2Yh6UuxaH4vtTjGvHg==}
     engines: {node: '>= 10'}
     cpu: [arm64]
     os: [linux]
-    libc: [musl]
 
   '@tailwindcss/oxide-linux-x64-gnu@4.1.18':
     resolution: {integrity: sha512-v3gyT0ivkfBLoZGF9LyHmts0Isc8jHZyVcbzio6Wpzifg/+5ZJpDiRiUhDLkcr7f/r38SWNe7ucxmGW3j3Kb/g==}
     engines: {node: '>= 10'}
     cpu: [x64]
     os: [linux]
-    libc: [glibc]
 
   '@tailwindcss/oxide-linux-x64-musl@4.1.18':
     resolution: {integrity: sha512-bhJ2y2OQNlcRwwgOAGMY0xTFStt4/wyU6pvI6LSuZpRgKQwxTec0/3Scu91O8ir7qCR3AuepQKLU/kX99FouqQ==}
     engines: {node: '>= 10'}
     cpu: [x64]
     os: [linux]
-    libc: [musl]
 
   '@tailwindcss/oxide-wasm32-wasi@4.1.18':
     resolution: {integrity: sha512-LffYTvPjODiP6PT16oNeUQJzNVyJl1cjIebq/rWWBF+3eDst5JGEFSc5cWxyRCJ0Mxl+KyIkqRxk1XPEs9x8TA==}
@@ -1496,6 +1554,38 @@ packages:
     resolution: {integrity: sha512-cHHDnewHozgjpI+MIVp9tcib6lYEQK5MyUr0ChHpHFGBl8Xei55rohFK0I0ve/GKoHeioaK42Smd8OixPp6CTg==}
     engines: {node: '>=12'}
 
+  '@testing-library/dom@10.4.1':
+    resolution: {integrity: sha512-o4PXJQidqJl82ckFaXUeoAW+XysPLauYI43Abki5hABd853iMhitooc6znOnczgbTYmEP6U6/y1ZyKAIsvMKGg==}
+    engines: {node: '>=18'}
+
+  '@testing-library/jest-dom@6.9.1':
+    resolution: {integrity: sha512-zIcONa+hVtVSSep9UT3jZ5rizo2BsxgyDYU7WFD5eICBE7no3881HGeb/QkGfsJs6JTkY1aQhT7rIPC7e+0nnA==}
+    engines: {node: '>=14', npm: '>=6', yarn: '>=1'}
+
+  '@testing-library/react@16.3.2':
+    resolution: {integrity: sha512-XU5/SytQM+ykqMnAnvB2umaJNIOsLF3PVv//1Ew4CTcpz0/BRyy/af40qqrt7SjKpDdT1saBMc42CUok5gaw+g==}
+    engines: {node: '>=18'}
+    peerDependencies:
+      '@testing-library/dom': ^10.0.0
+      '@types/react': ^18.0.0 || ^19.0.0
+      '@types/react-dom': ^18.0.0 || ^19.0.0
+      react: ^18.0.0 || ^19.0.0
+      react-dom: ^18.0.0 || ^19.0.0
+    peerDependenciesMeta:
+      '@types/react':
+        optional: true
+      '@types/react-dom':
+        optional: true
+
+  '@testing-library/user-event@14.6.1':
+    resolution: {integrity: sha512-vq7fv0rnt+QTXgPxr5Hjc210p6YKq2kmdziLgnsZGgLJ9e6VAShx1pACLuRjd/AS/sr7phAR58OIIpf0LlmQNw==}
+    engines: {node: '>=12', npm: '>=6'}
+    peerDependencies:
+      '@testing-library/dom': '>=7.21.4'
+
+  '@types/aria-query@5.0.4':
+    resolution: {integrity: sha512-rfT93uj5s0PRL7EzccGMs3brplhcrghnDoV26NqKhCAS1hVo+WdNsPvE/yb6ilfr5hi2MEk6d5EWJTKdxg8jVw==}
+
   '@types/babel__core@7.20.5':
     resolution: {integrity: sha512-qoQprZvz5wQFJwMDqeseRXWv3rqMvhgpbXFfVyWhbx9X47POIA6i/+dXefEmZKoAgOaTdaIgNSMqMIU61yRyzA==}
 
@@ -1734,10 +1824,18 @@ packages:
   ajv@6.12.6:
     resolution: {integrity: sha512-j3fVLgvTo527anyYyJOGTYJbG+vnnQYvE0m5mmkc1TK+nxAppkCLMIL0aZ4dblVCNoGShhm+kzE4ZUykBoMg4g==}
 
+  ansi-regex@5.0.1:
+    resolution: {integrity: sha512-quJQXlTSUGL2LH9SUXo8VwsY4soanhgo6LNSm84E1LBcE8s3O0wpdiRzyR9z/ZZJMlMWv37qOOb9pdJlMUEKFQ==}
+    engines: {node: '>=8'}
+
   ansi-styles@4.3.0:
     resolution: {integrity: sha512-zbB9rCJAT1rbjiVDb2hqKFHNYLxgtk8NURxZ3IZwD3F6NtxbXZQCnnSi1Lkx+IDohdPlFp222wVALIheZJQSEg==}
     engines: {node: '>=8'}
 
+  ansi-styles@5.2.0:
+    resolution: {integrity: sha512-Cxwpt2SfTzTtXcfOlzGEee8O+c+MmUgGrNiBcXnuWxuFJHe6a5Hz7qwhwe5OgaSYI0IJvkLqWX1ASG+cJOkEiA==}
+    engines: {node: '>=10'}
+
   ansis@4.2.0:
     resolution: {integrity: sha512-HqZ5rWlFjGiV0tDm3UxxgNRqsOTniqoKZu0pIAfh7TZQMGuZK+hH0drySty0si0QXj1ieop4+SkSfPZBPPkHig==}
     engines: {node: '>=14'}
@@ -1753,6 +1851,13 @@ packages:
     resolution: {integrity: sha512-ik3ZgC9dY/lYVVM++OISsaYDeg1tb0VtP5uL3ouh1koGOaUMDPpbFIei4JkFimWUFPn90sbMNMXQAIVOlnYKJA==}
     engines: {node: '>=10'}
 
+  aria-query@5.3.0:
+    resolution: {integrity: sha512-b0P0sZPKtyu8HkeRAfCq0IfURZK+SuwMjY1UXGBU27wpAiTwQAIlq56IbIO+ytk/JjS1fMR14ee5WBBfKi5J6A==}
+
+  aria-query@5.3.2:
+    resolution: {integrity: sha512-COROpnaoap1E2F000S62r6A60uHZnmlvomhfyT2DlTcrY1OrBKn2UhH7qn5wTC9zMvD0AY7csdPSNwKP+7WiQw==}
+    engines: {node: '>= 0.4'}
+
   assertion-error@2.0.1:
     resolution: {integrity: sha512-Izi8RQcffqCeNVgFigKli1ssklIbpHnCYc6AknXGYoB6grJqyeby7jv12JUQgmTAnIDnbck1uxksT4dzN3PWBA==}
     engines: {node: '>=12'}
@@ -1781,6 +1886,9 @@ packages:
     resolution: {integrity: sha512-ipDqC8FrAl/76p2SSWKSI+H9tFwm7vYqXQrItCuiVPt26Km0jS+NzSsBWAaBusvSbQcfJG+JitdMm+wZAgTYqg==}
     hasBin: true
 
+  bidi-js@1.0.3:
+    resolution: {integrity: sha512-RKshQI1R3YQ+n9YJz2QQ147P66ELpa1FQEg20Dk8oW9t2KgLbpDLLp9aGZ7y8WHSshDknG0bknqGw5/tyCs5tw==}
+
   binary-extensions@2.3.0:
     resolution: {integrity: sha512-Ceh+7ox5qe7LJuLHoY0feh3pHuUDHAcRUeyL2VYghZwfpkNIy/+8Ocg0a3UuSoYzavmylwuLWQOf3hl0jjMMIw==}
     engines: {node: '>=8'}
@@ -1861,6 +1969,13 @@ packages:
   css-line-break@2.1.0:
     resolution: {integrity: sha512-FHcKFCZcAha3LwfVBhCQbW2nCNbkZXn7KVUJcsT5/P8YmfsVja0FMPJr0B903j/E69HUphKiV9iQArX8SDYA4w==}
 
+  css-tree@3.2.1:
+    resolution: {integrity: sha512-X7sjQzceUhu1u7Y/ylrRZFU2FS6LRiFVp6rKLPg23y3x3c3DOKAwuXGDp+PAGjh6CSnCjYeAul8pcT8bAl+lSA==}
+    engines: {node: ^10 || ^12.20.0 || ^14.13.0 || >=15.0.0}
+
+  css.escape@1.5.1:
+    resolution: {integrity: sha512-YUifsXXuknHlUsmlgyY0PKzgPOr7/FjCePfHNt0jxm83wHZi44VDMQ7/fGNkjY3/jV1MC+1CmZbaHzugyeRtpg==}
+
   csstype@3.2.3:
     resolution: {integrity: sha512-z1HGKcYy2xA8AGQfwrn0PAy+PB7X/GSj3UVJW9qKyn43xWa+gl5nXmU4qqLMRzWVLFC8KusUX8T/0kCiOYpAIQ==}
 
@@ -1991,6 +2106,10 @@ packages:
     resolution: {integrity: sha512-e1U46jVP+w7Iut8Jt8ri1YsPOvFpg46k+K8TpCb0P+zjCkjkPnV7WzfDJzMHy1LnA+wj5pLT1wjO901gLXeEhA==}
     engines: {node: '>=12'}
 
+  data-urls@7.0.0:
+    resolution: {integrity: sha512-23XHcCF+coGYevirZceTVD7NdJOqVn+49IHyxgszm+JIiHLoB2TkmPtsYkNWT1pvRSGkc35L6NHs0yHkN2SumA==}
+    engines: {node: ^20.19.0 || ^22.12.0 || >=24.0.0}
+
   debug@4.4.3:
     resolution: {integrity: sha512-RGwwWnwQvkVfavKVt22FGLw+xYSdzARwm0ru6DhTVA3umU5hZc28V3kO4stgYryrTlLpuvgI9GiijltAjNbcqA==}
     engines: {node: '>=6.0'}
@@ -2000,6 +2119,9 @@ packages:
       supports-color:
         optional: true
 
+  decimal.js@10.6.0:
+    resolution: {integrity: sha512-YpgQiITW3JXGntzdUmyUR1V812Hn8T1YVXhCu+wO3OpS4eU9l4YdD3qjyiKdV6mvV29zapkMeD390UVEf2lkUg==}
+
   deep-is@0.1.4:
     resolution: {integrity: sha512-oIPzksmTg4/MriiaYGO+okXDT7ztn/w3Eptv/+gSIdMdKsJo0u4CfYNFJPy+4SKMuCqGw2wxnA+URMg3t8a/bQ==}
 
@@ -2010,6 +2132,10 @@ packages:
     resolution: {integrity: sha512-ZySD7Nf91aLB0RxL4KGrKHBXl7Eds1DAmEdcoVawXnLD7SDhpNgtuII2aAkg7a7QS41jxPSZ17p4VdGnMHk3MQ==}
     engines: {node: '>=0.4.0'}
 
+  dequal@2.0.3:
+    resolution: {integrity: sha512-0je+qPKHEMohvfRTCEo3CrPG6cAzAYgmzKyxRiYSSDkS6eGJdyVJm7WaYA5ECaAD9wLB2T4EEeymA5aFVcYXCA==}
+    engines: {node: '>=6'}
+
   detect-libc@2.1.2:
     resolution: {integrity: sha512-Btj2BOOO83o3WyH59e8MgXsxEQVcarkUOpEYrubB0urwnN10yQ364rsiByU11nZlqWYZm05i/of7io4mzihBtQ==}
     engines: {node: '>=8'}
@@ -2021,6 +2147,12 @@ packages:
     resolution: {integrity: sha512-qejHi7bcSD4hQAZE0tNAawRK1ZtafHDmMTMkrrIGgSLl7hTnQHmKCeB45xAcbfTqK2zowkM3j3bHt/4b/ARbYQ==}
     engines: {node: '>=0.3.1'}
 
+  dom-accessibility-api@0.5.16:
+    resolution: {integrity: sha512-X7BJ2yElsnOJ30pZF4uIIDfBEVgF4XEBxL9Bxhy6dnrm5hkzqmsWHGTiHqRiITNhMyFLyAiWndIJP7Z1NTteDg==}
+
+  dom-accessibility-api@0.6.3:
+    resolution: {integrity: sha512-7ZgogeTnjuHbo+ct10G9Ffp0mif17idi0IyWNVA/wcwcm7NPOD/WEHVP3n7n3MhXqxoIYm8d6MuZohYWIZ4T3w==}
+
   dunder-proto@1.0.1:
     resolution: {integrity: sha512-KIN/nDJBQRcXw0MLVhZE9iQHmG68qAVIBg9CqmUYjmQIhgij9U5MFvrqkUL5FbtyyzZuOeOt0zdeRe4UY7ct+A==}
     engines: {node: '>= 0.4'}
@@ -2032,6 +2164,10 @@ packages:
     resolution: {integrity: sha512-phv3E1Xl4tQOShqSte26C7Fl84EwUdZsyOuSSk9qtAGyyQs2s3jJzComh+Abf4g187lUUAvH+H26omrqia2aGg==}
     engines: {node: '>=10.13.0'}
 
+  entities@6.0.1:
+    resolution: {integrity: sha512-aN97NXWF6AWBTahfVOIrB/NShkzi5H7F9r1s9mD3cDj4Ko5f2qhhVoYMibXF7GlLveb/D2ioWay8lxI97Ven3g==}
+    engines: {node: '>=0.12'}
+
   es-define-property@1.0.1:
     resolution: {integrity: sha512-e3nRfgfUZ4rNGL232gUgX06QNyyez04KdjFrF+LTRoOXmrOgFKDg4BCdsjW8EnT69eqdYGmRpJwiPVYNrCaW3g==}
     engines: {node: '>= 0.4'}
@@ -2262,6 +2398,10 @@ packages:
   hermes-parser@0.25.1:
     resolution: {integrity: sha512-6pEjquH3rqaI6cYAXYPcz9MS4rY6R4ngRgrgfDshRptUZIc3lw0MCIJIGDj9++mfySOuPTHB4nrSW99BCvOPIA==}
 
+  html-encoding-sniffer@6.0.0:
+    resolution: {integrity: sha512-CV9TW3Y3f8/wT0BRFc1/KAVQ3TUHiXmaAb6VW9vtiMFf7SLoMd1PdAc4W3KFOFETBJUb90KatHqlsZMWV+R9Gg==}
+    engines: {node: ^20.19.0 || ^22.12.0 || >=24.0.0}
+
   html-parse-stringify@3.0.1:
     resolution: {integrity: sha512-KknJ50kTInJ7qIScF3jeaFRpMpE8/lfiTdzf/twXyPBLAGrLRTmkz3AdTnKeh40X8k9L2fdYwEp/42WGXIRGcg==}
 
@@ -2300,6 +2440,10 @@ packages:
     resolution: {integrity: sha512-JmXMZ6wuvDmLiHEml9ykzqO6lwFbof0GG4IkcGaENdCRDDmMVnny7s5HsIgHCbaq0w2MyPhDqkhTUgS2LU2PHA==}
     engines: {node: '>=0.8.19'}
 
+  indent-string@4.0.0:
+    resolution: {integrity: sha512-EdDDZu4A2OyIK7Lr/2zG+w5jmbuk1DVBnEwREQvBzspBJkCEbRa8GxU1lghYcaGJCnRWibjDXlq779X1/y5xwg==}
+    engines: {node: '>=8'}
+
   internmap@2.0.3:
     resolution: {integrity: sha512-5Hh7Y1wQbvY5ooGgPbDaL5iYLAPzMTUrjMulskHLH6wnv/A+1q5rgEaiuqEjB+oxGXIVZs1FF+R/KPN3ZSQYYg==}
     engines: {node: '>=12'}
@@ -2320,6 +2464,9 @@ packages:
     resolution: {integrity: sha512-41Cifkg6e8TylSpdtTpeLVMqvSBEVzTttHvERD741+pnZ8ANv0004MRL43QKPDlK9cGvNp6NZWZUBlbGXYxxng==}
     engines: {node: '>=0.12.0'}
 
+  is-potential-custom-element-name@1.0.1:
+    resolution: {integrity: sha512-bCYeRA2rVibKZd+s2625gGnGF/t7DSqDs4dP7CrLA1m7jKWz6pps0LpYLJN8Q64HtmPKJ1hrN3nzPNKFEKOUiQ==}
+
   isbot@5.1.34:
     resolution: {integrity: sha512-aCMIBSKd/XPRYdiCQTLC8QHH4YT8B3JUADu+7COgYIZPvkeoMcUHMRjZLM9/7V8fCj+l7FSREc1lOPNjzogo/A==}
     engines: {node: '>=18'}
@@ -2338,6 +2485,15 @@ packages:
     resolution: {integrity: sha512-qQKT4zQxXl8lLwBtHMWwaTcGfFOZviOJet3Oy/xmGk2gZH677CJM9EvtfdSkgWcATZhj/55JZ0rmy3myCT5lsA==}
     hasBin: true
 
+  jsdom@29.0.0:
+    resolution: {integrity: sha512-9FshNB6OepopZ08unmmGpsF7/qCjxGPbo3NbgfJAnPeHXnsODE9WWffXZtRFRFe0ntzaAOcSKNJFz8wiyvF1jQ==}
+    engines: {node: ^20.19.0 || ^22.13.0 || >=24.0.0}
+    peerDependencies:
+      canvas: ^3.0.0
+    peerDependenciesMeta:
+      canvas:
+        optional: true
+
   jsesc@3.1.0:
     resolution: {integrity: sha512-/sM3dO2FOzXjKQhJuo0Q173wf2KOo8t4I8vHy6lF9poUp7bKT0/NHE8fPX23PwfhnykfqnC2xRxOnVw5XuGIaA==}
     engines: {node: '>=6'}
@@ -2399,28 +2555,24 @@ packages:
     engines: {node: '>= 12.0.0'}
     cpu: [arm64]
     os: [linux]
-    libc: [glibc]
 
   lightningcss-linux-arm64-musl@1.30.2:
     resolution: {integrity: sha512-5Vh9dGeblpTxWHpOx8iauV02popZDsCYMPIgiuw97OJ5uaDsL86cnqSFs5LZkG3ghHoX5isLgWzMs+eD1YzrnA==}
     engines: {node: '>= 12.0.0'}
     cpu: [arm64]
     os: [linux]
-    libc: [musl]
 
   lightningcss-linux-x64-gnu@1.30.2:
     resolution: {integrity: sha512-Cfd46gdmj1vQ+lR6VRTTadNHu6ALuw2pKR9lYq4FnhvgBc4zWY1EtZcAc6EffShbb1MFrIPfLDXD6Xprbnni4w==}
     engines: {node: '>= 12.0.0'}
     cpu: [x64]
     os: [linux]
-    libc: [glibc]
 
   lightningcss-linux-x64-musl@1.30.2:
     resolution: {integrity: sha512-XJaLUUFXb6/QG2lGIW6aIk6jKdtjtcffUT0NKvIqhSBY3hh9Ch+1LCeH80dR9q9LBjG3ewbDjnumefsLsP6aiA==}
     engines: {node: '>= 12.0.0'}
     cpu: [x64]
     os: [linux]
-    libc: [musl]
 
   lightningcss-win32-arm64-msvc@1.30.2:
     resolution: {integrity: sha512-FZn+vaj7zLv//D/192WFFVA0RgHawIcHqLX9xuWiQt7P0PtdFEVaxgF9rjM/IRYHQXNnk61/H/gb2Ei+kUQ4xQ==}
@@ -2445,6 +2597,10 @@ packages:
   lodash.merge@4.6.2:
     resolution: {integrity: sha512-0KpjqXRVvrYyCsX1swR/XTK0va6VQkQM6MNo7PqW77ByjAhoARA8EfrP1N4+KlKj8YS0ZUCtRT/YUuhyYDujIQ==}
 
+  lru-cache@11.2.7:
+    resolution: {integrity: sha512-aY/R+aEsRelme17KGQa/1ZSIpLpNYYrhcrepKTZgE+W3WM16YMCaPwOHLHsmopZHELU0Ojin1lPVxKR0MihncA==}
+    engines: {node: 20 || >=22}
+
   lru-cache@5.1.1:
     resolution: {integrity: sha512-KpNARQA3Iwv+jTA0utUVVbrh+Jlrr1Fv0e56GGzAFOXN7dk/FviaDW8LHmK52DlcH4WP2n6gI8vN1aesBFgo9w==}
 
@@ -2453,6 +2609,10 @@ packages:
     peerDependencies:
       react: ^16.5.1 || ^17.0.0 || ^18.0.0 || ^19.0.0
 
+  lz-string@1.5.0:
+    resolution: {integrity: sha512-h5bgJWpxJNswbU7qCrV0tIKQCaS3blPDrqKWx+QxzuzL1zGUzij9XCWLrSLsJPu5t+eWA/ycetzYAO5IOMcWAQ==}
+    hasBin: true
+
   magic-string@0.30.21:
     resolution: {integrity: sha512-vd2F4YUyEXKGcLHoq+TEyCjxueSeHnFxyyjNp80yg0XV4vUhnDer/lvvlqM/arB5bXQN5K2/3oinyCRyx8T2CQ==}
 
@@ -2460,6 +2620,9 @@ packages:
     resolution: {integrity: sha512-/IXtbwEk5HTPyEwyKX6hGkYXxM9nbj64B+ilVJnC/R6B0pH5G4V3b0pVbL7DBj4tkhBAppbQUlf6F6Xl9LHu1g==}
     engines: {node: '>= 0.4'}
 
+  mdn-data@2.27.1:
+    resolution: {integrity: sha512-9Yubnt3e8A0OKwxYSXyhLymGW4sCufcLG6VdiDdUGVkPhpqLxlvP5vl1983gQjJl3tqbrM731mjaZaP68AgosQ==}
+
   mime-db@1.52.0:
     resolution: {integrity: sha512-sPU4uV7dYlvtWJxwwxHD0PuihVNiE7TyAbQ5SWxDCB9mUYvOgroQOwYQQOKPJ8CIbE+1ETVlOoK1UC2nU3gYvg==}
     engines: {node: '>= 0.6'}
@@ -2468,6 +2631,10 @@ packages:
     resolution: {integrity: sha512-ZDY+bPm5zTTF+YpCrAU9nK0UgICYPT0QtT1NZWFv4s++TNkcgVaT0g6+4R2uI4MjQjzysHB1zxuWL50hzaeXiw==}
     engines: {node: '>= 0.6'}
 
+  min-indent@1.0.1:
+    resolution: {integrity: sha512-I9jwMn07Sy/IwOj3zVkVik2JTvgpaykDZEigL6Rx6N9LbMywwUSMtxET+7lVoDLLd3O3IXwJwvuuns8UB/HeAg==}
+    engines: {node: '>=4'}
+
   minimatch@3.1.2:
     resolution: {integrity: sha512-J7p63hRiAjw1NDEww1W7i37+ByIrOWO5XQQAzZ3VOcL0PNybwpfmV/N05zFAzwQ9USyEcX6t3UO+K5aqBQOIHw==}
 
@@ -2512,6 +2679,9 @@ packages:
     resolution: {integrity: sha512-GQ2EWRpQV8/o+Aw8YqtfZZPfNRWZYkbidE9k5rpl/hC3vtHHBfGm2Ifi6qWV+coDGkrUKZAxE3Lot5kcsRlh+g==}
     engines: {node: '>=6'}
 
+  parse5@8.0.0:
+    resolution: {integrity: sha512-9m4m5GSgXjL4AjumKzq1Fgfp3Z8rsvjRNbnkVwfu2ImRqE5D0LnY2QfDen18FSY9C573YU5XxSapdHZTZ2WolA==}
+
   path-exists@4.0.0:
     resolution: {integrity: sha512-ak9Qy5Q7jYb2Wwcey5Fpvg2KoAc/ZIhLSLOSBmRmygPsGwkVVt0fZa0qrtMz+m6tJTAHfZQ8FnmB4MG4LWy7/w==}
     engines: {node: '>=8'}
@@ -2547,6 +2717,10 @@ packages:
     engines: {node: '>=14'}
     hasBin: true
 
+  pretty-format@27.5.1:
+    resolution: {integrity: sha512-Qb1gy5OrP5+zDf2Bvnzdl3jsTf1qXVMazbvCoKhtKqVs4/YK4ozX4gKQJJVyNe+cajNPn0KoC0MC3FUmaHWEmQ==}
+    engines: {node: ^10.13.0 || ^12.13.0 || ^14.15.0 || >=15.0.0}
+
   proxy-from-env@1.1.0:
     resolution: {integrity: sha512-D+zkORCbA9f1tdWRK0RaCR3GPv50cMxcrz4X8k5LTSUD1Dkw47mKJEZQNunItRTkWwgtaUSo1RVFRIG9ZXiFYg==}
 
@@ -2588,6 +2762,9 @@ packages:
       typescript:
         optional: true
 
+  react-is@17.0.2:
+    resolution: {integrity: sha512-w2GsyukL62IJnlaff/nRegPQR94C/XXamvMWmSHRJ4y7Ts/4ocGRmTHvOs8PSE6pB3dWOrD/nueuU5sduBsQ4w==}
+
   react-refresh@0.18.0:
     resolution: {integrity: sha512-QgT5//D3jfjJb6Gsjxv0Slpj23ip+HtOpnNgnb2S5zU3CB26G/IDPGoy4RJB42wzFE46DRsstbW6tKHoKbhAxw==}
     engines: {node: '>=0.10.0'}
@@ -2634,6 +2811,14 @@ packages:
     resolution: {integrity: sha512-YTUo+Flmw4ZXiWfQKGcwwc11KnoRAYgzAE2E7mXKCjSviTKShtxBsN6YUUBB2gtaBzKzeKunxhUwNHQuRryhWA==}
     engines: {node: '>= 4'}
 
+  redent@3.0.0:
+    resolution: {integrity: sha512-6tDA8g98We0zd0GvVeMT9arEOnTw9qM03L9cJXaCjrip1OO764RDBLBfrB4cwzNGDj5OA5ioymC9GkizgWJDUg==}
+    engines: {node: '>=8'}
+
+  require-from-string@2.0.2:
+    resolution: {integrity: sha512-Xf0nWe6RseziFMu+Ap9biiUbmplq6S9/p+7w7YXP/JBHhrUDDUhwa+vANyubuqfZWTveU//DYVGsDG7RKL/vEw==}
+    engines: {node: '>=0.10.0'}
+
   resolve-from@4.0.0:
     resolution: {integrity: sha512-pb/MYmXstAkysRFx8piNI1tGFNQIFA3vkE3Gq4EuA1dF6gHp/+vgZqsCGJapvy8N3Q+4o7FwvquPJcnZ7RYy4g==}
     engines: {node: '>=4'}
@@ -2655,6 +2840,10 @@ packages:
   safer-buffer@2.1.2:
     resolution: {integrity: sha512-YZo3K82SD7Riyi0E1EQPojLz7kpepnSQI9IyPbHHg1XXXevb5dJI7tpyN2ADxGcQbHG7vcyRHk0cbwqcQriUtg==}
 
+  saxes@6.0.0:
+    resolution: {integrity: sha512-xAg7SOnEhrm5zI3puOOKyy1OMcMlIJZYNJY7xLBwSze0UjhPLnWfj2GF2EpT0jmzaJKIWKHLsaSSajf35bcYnA==}
+    engines: {node: '>=v12.22.7'}
+
   scheduler@0.27.0:
     resolution: {integrity: sha512-eNv+WrVbKu1f3vbYJT/xtiF5syA5HPIMtf9IgY/nKg0sWqzAUEvqY/xm7OcZc/qafLx/iO9FgOmeSAp4v5ti/Q==}
 
@@ -2706,6 +2895,10 @@ packages:
   std-env@3.10.0:
     resolution: {integrity: sha512-5GS12FdOZNliM5mAOxFRg7Ir0pWz8MdpYm6AY6VPkGpbA7ZzmbzNcBJQ0GPvvyWgcY7QAhCgf9Uy89I03faLkg==}
 
+  strip-indent@3.0.0:
+    resolution: {integrity: sha512-laJTa3Jb+VQpaC6DseHhF7dXVqHTfJPCRDaEbid/drOhgitgYku/letMUqOXFoWV0zIIUbjpdH2t+tYj4bQMRQ==}
+    engines: {node: '>=8'}
+
   strip-json-comments@3.1.1:
     resolution: {integrity: sha512-6fPc+R4ihwqP6N/aIv2f1gMH8lOVtWQHoqC4yK6oSDVVocumAsfCqjkXnqiYMhmMwS/mEHLp7Vehlt3ql6lEig==}
     engines: {node: '>=8'}
@@ -2714,6 +2907,9 @@ packages:
     resolution: {integrity: sha512-qpCAvRl9stuOHveKsn7HncJRvv501qIacKzQlO/+Lwxc9+0q2wLyv4Dfvt80/DPn2pqOBsJdDiogXGR9+OvwRw==}
     engines: {node: '>=8'}
 
+  symbol-tree@3.2.4:
+    resolution: {integrity: sha512-9QNk5KwDF+Bvz+PyObkmSYjI5ksVUYtjW7AU22r2NKcfLJcXp96hkDWU3+XndOsUb+AQ9QhfzfCT2O+CNWT5Tw==}
+
   tailwind-merge@3.4.0:
     resolution: {integrity: sha512-uSaO4gnW+b3Y2aWoWfFpX62vn2sR3skfhbjsEnaBI81WD1wBLlHZe5sWf0AqjksNdYTbGBEd0UasQMT3SNV15g==}
 
@@ -2748,10 +2944,25 @@ packages:
     resolution: {integrity: sha512-PSkbLUoxOFRzJYjjxHJt9xro7D+iilgMX/C9lawzVuYiIdcihh9DXmVibBe8lmcFrRi/VzlPjBxbN7rH24q8/Q==}
     engines: {node: '>=14.0.0'}
 
+  tldts-core@7.0.26:
+    resolution: {integrity: sha512-5WJ2SqFsv4G2Dwi7ZFVRnz6b2H1od39QME1lc2y5Ew3eWiZMAeqOAfWpRP9jHvhUl881406QtZTODvjttJs+ew==}
+
+  tldts@7.0.26:
+    resolution: {integrity: sha512-WiGwQjr0qYdNNG8KpMKlSvpxz652lqa3Rd+/hSaDcY4Uo6SKWZq2LAF+hsAhUewTtYhXlorBKgNF3Kk8hnjGoQ==}
+    hasBin: true
+
   to-regex-range@5.0.1:
     resolution: {integrity: sha512-65P7iz6X5yEr1cwcgvQxbbIw7Uk3gOy5dIdtZ4rDveLqhrdJP+Li/Hx6tyK0NEb+2GCyneCMJiGqrADCSNk8sQ==}
     engines: {node: '>=8.0'}
 
+  tough-cookie@6.0.1:
+    resolution: {integrity: sha512-LktZQb3IeoUWB9lqR5EWTHgW/VTITCXg4D21M+lvybRVdylLrRMnqaIONLVb5mav8vM19m44HIcGq4qASeu2Qw==}
+    engines: {node: '>=16'}
+
+  tr46@6.0.0:
+    resolution: {integrity: sha512-bLVMLPtstlZ4iMQHpFHTR7GAGj2jxi8Dg0s2h2MafAE4uSWF98FC/3MomU51iQAMf8/qDUbKWf5GxuvvVcXEhw==}
+    engines: {node: '>=20'}
+
   ts-api-utils@2.4.0:
     resolution: {integrity: sha512-3TaVTaAv2gTiMB35i3FiGJaRfwb3Pyn/j3m/bfAvGe8FB7CF6u+LMYqYlDh7reQf7UNvoTvdfAqHGmPGOSsPmA==}
     engines: {node: '>=18.12'}
@@ -2785,6 +2996,10 @@ packages:
   undici-types@7.16.0:
     resolution: {integrity: sha512-Zz+aZWSj8LE6zoxD+xrjh4VfkIG8Ya6LvYkZqtUQGJPZjYl53ypCaUwWqo7eI0x66KBGeRo+mlBEkMSeSZ38Nw==}
 
+  undici@7.24.4:
+    resolution: {integrity: sha512-BM/JzwwaRXxrLdElV2Uo6cTLEjhSb3WXboncJamZ15NgUURmvlXvxa6xkwIOILIjPNo9i8ku136ZvWV0Uly8+w==}
+    engines: {node: '>=20.18.1'}
+
   unplugin@2.3.11:
     resolution: {integrity: sha512-5uKD0nqiYVzlmCRs01Fhs2BdkEgBS3SAVP6ndrBsuK42iC2+JHyxM05Rm9G8+5mkmRtzMZGY8Ct5+mliZxU/Ww==}
     engines: {node: '>=18.12.0'}
@@ -2904,9 +3119,25 @@ packages:
     resolution: {integrity: sha512-Dhxzh5HZuiHQhbvTW9AMetFfBHDMYpo23Uo9btPXgdYP+3T5S+p+jgNy7spra+veYhBP2dCSgxR/i2Y02h5/6w==}
     engines: {node: '>=0.10.0'}
 
+  w3c-xmlserializer@5.0.0:
+    resolution: {integrity: sha512-o8qghlI8NZHU1lLPrpi2+Uq7abh4GGPpYANlalzWxyWteJOCsr/P+oPBA49TOLu5FTZO4d3F9MnWJfiMo4BkmA==}
+    engines: {node: '>=18'}
+
+  webidl-conversions@8.0.1:
+    resolution: {integrity: sha512-BMhLD/Sw+GbJC21C/UgyaZX41nPt8bUTg+jWyDeg7e7YN4xOM05YPSIXceACnXVtqyEw/LMClUQMtMZ+PGGpqQ==}
+    engines: {node: '>=20'}
+
   webpack-virtual-modules@0.6.2:
     resolution: {integrity: sha512-66/V2i5hQanC51vBQKPH4aI8NMAcBW59FVBs+rC7eGHupMyfn34q7rZIE+ETlJ+XTevqfUhVVBgSUNSW2flEUQ==}
 
+  whatwg-mimetype@5.0.0:
+    resolution: {integrity: sha512-sXcNcHOC51uPGF0P/D4NVtrkjSU2fNsm9iog4ZvZJsL3rjoDAzXZhkm2MWt1y+PUdggKAYVoMAIYcs78wJ51Cw==}
+    engines: {node: '>=20'}
+
+  whatwg-url@16.0.1:
+    resolution: {integrity: sha512-1to4zXBxmXHV3IiSSEInrreIlu02vUOvrhxJJH5vcxYTBDAx51cqZiKdyTxlecdKNSjj8EcxGBxNf6Vg+945gw==}
+    engines: {node: ^20.19.0 || ^22.12.0 || >=24.0.0}
+
   which@2.0.2:
     resolution: {integrity: sha512-BLI3Tl1TW3Pvl70l3yq3Y64i+awpwXqsGBYWkkqMtnbXgrMD+yj7rhW0kuEDxzJaYXGjEW5ogapKNMEKNMjibA==}
     engines: {node: '>= 8'}
@@ -2921,6 +3152,13 @@ packages:
     resolution: {integrity: sha512-BN22B5eaMMI9UMtjrGd5g5eCYPpCPDUy0FJXbYsaT5zYxjFOckS53SQDE3pWkVoWpHXVb3BrYcEN4Twa55B5cA==}
     engines: {node: '>=0.10.0'}
 
+  xml-name-validator@5.0.0:
+    resolution: {integrity: sha512-EvGK8EJ3DhaHfbRlETOWAS5pO9MZITeauHKJyb8wyajUfQUenkIg2MvLDTZ4T/TgIcm3HU0TFBgWWboAZ30UHg==}
+    engines: {node: '>=18'}
+
+  xmlchars@2.2.0:
+    resolution: {integrity: sha512-JZnDKK8B0RCDw84FNdDAIpZK+JuJw+s7Lz8nksI7SIuU3UXJJslUthsi+uWBUYOwPFwW7W7PRLRfUKpxjtjFCw==}
+
   yallist@3.1.1:
     resolution: {integrity: sha512-a4UGQaWPH59mOXUYnAG2ewncQS4i4F43Tv3JoAM+s2VDAmS9NsK8GpDMLrCHPksFT7h3K6TOoUNn2pb7RoXx4g==}
 
@@ -2942,6 +3180,26 @@ packages:
 
 snapshots:
 
+  '@adobe/css-tools@4.4.4': {}
+
+  '@asamuzakjp/css-color@5.0.1':
+    dependencies:
+      '@csstools/css-calc': 3.1.1(@csstools/css-parser-algorithms@4.0.0(@csstools/css-tokenizer@4.0.0))(@csstools/css-tokenizer@4.0.0)
+      '@csstools/css-color-parser': 4.0.2(@csstools/css-parser-algorithms@4.0.0(@csstools/css-tokenizer@4.0.0))(@csstools/css-tokenizer@4.0.0)
+      '@csstools/css-parser-algorithms': 4.0.0(@csstools/css-tokenizer@4.0.0)
+      '@csstools/css-tokenizer': 4.0.0
+      lru-cache: 11.2.7
+
+  '@asamuzakjp/dom-selector@7.0.3':
+    dependencies:
+      '@asamuzakjp/nwsapi': 2.3.9
+      bidi-js: 1.0.3
+      css-tree: 3.2.1
+      is-potential-custom-element-name: 1.0.1
+      lru-cache: 11.2.7
+
+  '@asamuzakjp/nwsapi@2.3.9': {}
+
   '@babel/code-frame@7.29.0':
     dependencies:
       '@babel/helper-validator-identifier': 7.28.5
@@ -3066,6 +3324,34 @@ snapshots:
       '@babel/helper-string-parser': 7.27.1
       '@babel/helper-validator-identifier': 7.28.5
 
+  '@bramus/specificity@2.4.2':
+    dependencies:
+      css-tree: 3.2.1
+
+  '@csstools/color-helpers@6.0.2': {}
+
+  '@csstools/css-calc@3.1.1(@csstools/css-parser-algorithms@4.0.0(@csstools/css-tokenizer@4.0.0))(@csstools/css-tokenizer@4.0.0)':
+    dependencies:
+      '@csstools/css-parser-algorithms': 4.0.0(@csstools/css-tokenizer@4.0.0)
+      '@csstools/css-tokenizer': 4.0.0
+
+  '@csstools/css-color-parser@4.0.2(@csstools/css-parser-algorithms@4.0.0(@csstools/css-tokenizer@4.0.0))(@csstools/css-tokenizer@4.0.0)':
+    dependencies:
+      '@csstools/color-helpers': 6.0.2
+      '@csstools/css-calc': 3.1.1(@csstools/css-parser-algorithms@4.0.0(@csstools/css-tokenizer@4.0.0))(@csstools/css-tokenizer@4.0.0)
+      '@csstools/css-parser-algorithms': 4.0.0(@csstools/css-tokenizer@4.0.0)
+      '@csstools/css-tokenizer': 4.0.0
+
+  '@csstools/css-parser-algorithms@4.0.0(@csstools/css-tokenizer@4.0.0)':
+    dependencies:
+      '@csstools/css-tokenizer': 4.0.0
+
+  '@csstools/css-syntax-patches-for-csstree@1.1.1(css-tree@3.2.1)':
+    optionalDependencies:
+      css-tree: 3.2.1
+
+  '@csstools/css-tokenizer@4.0.0': {}
+
   '@esbuild/aix-ppc64@0.27.2':
     optional: true
 
@@ -3190,6 +3476,8 @@ snapshots:
       '@eslint/core': 0.17.0
       levn: 0.4.1
 
+  '@exodus/bytes@1.15.0': {}
+
   '@floating-ui/core@1.7.4':
     dependencies:
       '@floating-ui/utils': 0.2.10
@@ -4260,6 +4548,42 @@ snapshots:
 
   '@tanstack/virtual-file-routes@1.154.7': {}
 
+  '@testing-library/dom@10.4.1':
+    dependencies:
+      '@babel/code-frame': 7.29.0
+      '@babel/runtime': 7.28.6
+      '@types/aria-query': 5.0.4
+      aria-query: 5.3.0
+      dom-accessibility-api: 0.5.16
+      lz-string: 1.5.0
+      picocolors: 1.1.1
+      pretty-format: 27.5.1
+
+  '@testing-library/jest-dom@6.9.1':
+    dependencies:
+      '@adobe/css-tools': 4.4.4
+      aria-query: 5.3.2
+      css.escape: 1.5.1
+      dom-accessibility-api: 0.6.3
+      picocolors: 1.1.1
+      redent: 3.0.0
+
+  '@testing-library/react@16.3.2(@testing-library/dom@10.4.1)(@types/react-dom@19.2.3(@types/react@19.2.11))(@types/react@19.2.11)(react-dom@19.2.4(react@19.2.4))(react@19.2.4)':
+    dependencies:
+      '@babel/runtime': 7.28.6
+      '@testing-library/dom': 10.4.1
+      react: 19.2.4
+      react-dom: 19.2.4(react@19.2.4)
+    optionalDependencies:
+      '@types/react': 19.2.11
+      '@types/react-dom': 19.2.3(@types/react@19.2.11)
+
+  '@testing-library/user-event@14.6.1(@testing-library/dom@10.4.1)':
+    dependencies:
+      '@testing-library/dom': 10.4.1
+
+  '@types/aria-query@5.0.4': {}
+
   '@types/babel__core@7.20.5':
     dependencies:
       '@babel/parser': 7.29.0
@@ -4578,10 +4902,14 @@ snapshots:
       json-schema-traverse: 0.4.1
       uri-js: 4.4.1
 
+  ansi-regex@5.0.1: {}
+
   ansi-styles@4.3.0:
     dependencies:
       color-convert: 2.0.1
 
+  ansi-styles@5.2.0: {}
+
   ansis@4.2.0: {}
 
   anymatch@3.1.3:
@@ -4595,6 +4923,12 @@ snapshots:
     dependencies:
       tslib: 2.8.1
 
+  aria-query@5.3.0:
+    dependencies:
+      dequal: 2.0.3
+
+  aria-query@5.3.2: {}
+
   assertion-error@2.0.1: {}
 
   ast-types@0.16.1:
@@ -4626,6 +4960,10 @@ snapshots:
 
   baseline-browser-mapping@2.9.19: {}
 
+  bidi-js@1.0.3:
+    dependencies:
+      require-from-string: 2.0.2
+
   binary-extensions@2.3.0: {}
 
   brace-expansion@1.1.12:
@@ -4711,6 +5049,13 @@ snapshots:
     dependencies:
       utrie: 1.0.2
 
+  css-tree@3.2.1:
+    dependencies:
+      mdn-data: 2.27.1
+      source-map-js: 1.2.1
+
+  css.escape@1.5.1: {}
+
   csstype@3.2.3: {}
 
   d3-array@3.2.4:
@@ -4865,10 +5210,19 @@ snapshots:
       d3-transition: 3.0.1(d3-selection@3.0.0)
       d3-zoom: 3.0.0
 
+  data-urls@7.0.0:
+    dependencies:
+      whatwg-mimetype: 5.0.0
+      whatwg-url: 16.0.1
+    transitivePeerDependencies:
+      - '@noble/hashes'
+
   debug@4.4.3:
     dependencies:
       ms: 2.1.3
 
+  decimal.js@10.6.0: {}
+
   deep-is@0.1.4: {}
 
   delaunator@5.0.1:
@@ -4877,12 +5231,18 @@ snapshots:
 
   delayed-stream@1.0.0: {}
 
+  dequal@2.0.3: {}
+
   detect-libc@2.1.2: {}
 
   detect-node-es@1.1.0: {}
 
   diff@8.0.3: {}
 
+  dom-accessibility-api@0.5.16: {}
+
+  dom-accessibility-api@0.6.3: {}
+
   dunder-proto@1.0.1:
     dependencies:
       call-bind-apply-helpers: 1.0.2
@@ -4896,6 +5256,8 @@ snapshots:
       graceful-fs: 4.2.11
       tapable: 2.3.0
 
+  entities@6.0.1: {}
+
   es-define-property@1.0.1: {}
 
   es-errors@1.3.0: {}
@@ -5150,6 +5512,12 @@ snapshots:
     dependencies:
       hermes-estree: 0.25.1
 
+  html-encoding-sniffer@6.0.0:
+    dependencies:
+      '@exodus/bytes': 1.15.0
+    transitivePeerDependencies:
+      - '@noble/hashes'
+
   html-parse-stringify@3.0.1:
     dependencies:
       void-elements: 3.1.0
@@ -5184,6 +5552,8 @@ snapshots:
 
   imurmurhash@0.1.4: {}
 
+  indent-string@4.0.0: {}
+
   internmap@2.0.3: {}
 
   is-binary-path@2.1.0:
@@ -5198,6 +5568,8 @@ snapshots:
 
   is-number@7.0.0: {}
 
+  is-potential-custom-element-name@1.0.1: {}
+
   isbot@5.1.34: {}
 
   isexe@2.0.0: {}
@@ -5210,6 +5582,32 @@ snapshots:
     dependencies:
       argparse: 2.0.1
 
+  jsdom@29.0.0:
+    dependencies:
+      '@asamuzakjp/css-color': 5.0.1
+      '@asamuzakjp/dom-selector': 7.0.3
+      '@bramus/specificity': 2.4.2
+      '@csstools/css-syntax-patches-for-csstree': 1.1.1(css-tree@3.2.1)
+      '@exodus/bytes': 1.15.0
+      css-tree: 3.2.1
+      data-urls: 7.0.0
+      decimal.js: 10.6.0
+      html-encoding-sniffer: 6.0.0
+      is-potential-custom-element-name: 1.0.1
+      lru-cache: 11.2.7
+      parse5: 8.0.0
+      saxes: 6.0.0
+      symbol-tree: 3.2.4
+      tough-cookie: 6.0.1
+      undici: 7.24.4
+      w3c-xmlserializer: 5.0.0
+      webidl-conversions: 8.0.1
+      whatwg-mimetype: 5.0.0
+      whatwg-url: 16.0.1
+      xml-name-validator: 5.0.0
+    transitivePeerDependencies:
+      - '@noble/hashes'
+
   jsesc@3.1.0: {}
 
   json-buffer@3.0.1: {}
@@ -5284,6 +5682,8 @@ snapshots:
 
   lodash.merge@4.6.2: {}
 
+  lru-cache@11.2.7: {}
+
   lru-cache@5.1.1:
     dependencies:
       yallist: 3.1.1
@@ -5292,18 +5692,24 @@ snapshots:
     dependencies:
       react: 19.2.4
 
+  lz-string@1.5.0: {}
+
   magic-string@0.30.21:
     dependencies:
       '@jridgewell/sourcemap-codec': 1.5.5
 
   math-intrinsics@1.1.0: {}
 
+  mdn-data@2.27.1: {}
+
   mime-db@1.52.0: {}
 
   mime-types@2.1.35:
     dependencies:
       mime-db: 1.52.0
 
+  min-indent@1.0.1: {}
+
   minimatch@3.1.2:
     dependencies:
       brace-expansion: 1.1.12
@@ -5345,6 +5751,10 @@ snapshots:
     dependencies:
       callsites: 3.1.0
 
+  parse5@8.0.0:
+    dependencies:
+      entities: 6.0.1
+
   path-exists@4.0.0: {}
 
   path-key@3.1.1: {}
@@ -5367,6 +5777,12 @@ snapshots:
 
   prettier@3.8.1: {}
 
+  pretty-format@27.5.1:
+    dependencies:
+      ansi-regex: 5.0.1
+      ansi-styles: 5.2.0
+      react-is: 17.0.2
+
   proxy-from-env@1.1.0: {}
 
   punycode@2.3.1: {}
@@ -5450,6 +5866,8 @@ snapshots:
       react-dom: 19.2.4(react@19.2.4)
       typescript: 5.9.3
 
+  react-is@17.0.2: {}
+
   react-refresh@0.18.0: {}
 
   react-remove-scroll-bar@2.3.8(@types/react@19.2.11)(react@19.2.4):
@@ -5493,6 +5911,13 @@ snapshots:
       tiny-invariant: 1.3.3
       tslib: 2.8.1
 
+  redent@3.0.0:
+    dependencies:
+      indent-string: 4.0.0
+      strip-indent: 3.0.0
+
+  require-from-string@2.0.2: {}
+
   resolve-from@4.0.0: {}
 
   resolve-pkg-maps@1.0.0: {}
@@ -5534,6 +5959,10 @@ snapshots:
 
   safer-buffer@2.1.2: {}
 
+  saxes@6.0.0:
+    dependencies:
+      xmlchars: 2.2.0
+
   scheduler@0.27.0: {}
 
   semver@6.3.1: {}
@@ -5564,12 +5993,18 @@ snapshots:
 
   std-env@3.10.0: {}
 
+  strip-indent@3.0.0:
+    dependencies:
+      min-indent: 1.0.1
+
   strip-json-comments@3.1.1: {}
 
   supports-color@7.2.0:
     dependencies:
       has-flag: 4.0.0
 
+  symbol-tree@3.2.4: {}
+
   tailwind-merge@3.4.0: {}
 
   tailwindcss@4.1.18: {}
@@ -5595,10 +6030,24 @@ snapshots:
 
   tinyrainbow@3.0.3: {}
 
+  tldts-core@7.0.26: {}
+
+  tldts@7.0.26:
+    dependencies:
+      tldts-core: 7.0.26
+
   to-regex-range@5.0.1:
     dependencies:
       is-number: 7.0.0
 
+  tough-cookie@6.0.1:
+    dependencies:
+      tldts: 7.0.26
+
+  tr46@6.0.0:
+    dependencies:
+      punycode: 2.3.1
+
   ts-api-utils@2.4.0(typescript@5.9.3):
     dependencies:
       typescript: 5.9.3
@@ -5631,6 +6080,8 @@ snapshots:
 
   undici-types@7.16.0: {}
 
+  undici@7.24.4: {}
+
   unplugin@2.3.11:
     dependencies:
       '@jridgewell/remapping': 2.3.5
@@ -5686,7 +6137,7 @@ snapshots:
       lightningcss: 1.30.2
       tsx: 4.21.0
 
-  vitest@4.0.18(@types/node@24.10.10)(jiti@2.6.1)(lightningcss@1.30.2)(tsx@4.21.0):
+  vitest@4.0.18(@types/node@24.10.10)(jiti@2.6.1)(jsdom@29.0.0)(lightningcss@1.30.2)(tsx@4.21.0):
     dependencies:
       '@vitest/expect': 4.0.18
       '@vitest/mocker': 4.0.18(vite@7.3.1(@types/node@24.10.10)(jiti@2.6.1)(lightningcss@1.30.2)(tsx@4.21.0))
@@ -5710,6 +6161,7 @@ snapshots:
       why-is-node-running: 2.3.0
     optionalDependencies:
       '@types/node': 24.10.10
+      jsdom: 29.0.0
     transitivePeerDependencies:
       - jiti
       - less
@@ -5725,8 +6177,24 @@ snapshots:
 
   void-elements@3.1.0: {}
 
+  w3c-xmlserializer@5.0.0:
+    dependencies:
+      xml-name-validator: 5.0.0
+
+  webidl-conversions@8.0.1: {}
+
   webpack-virtual-modules@0.6.2: {}
 
+  whatwg-mimetype@5.0.0: {}
+
+  whatwg-url@16.0.1:
+    dependencies:
+      '@exodus/bytes': 1.15.0
+      tr46: 6.0.0
+      webidl-conversions: 8.0.1
+    transitivePeerDependencies:
+      - '@noble/hashes'
+
   which@2.0.2:
     dependencies:
       isexe: 2.0.0
@@ -5738,6 +6206,10 @@ snapshots:
 
   word-wrap@1.2.5: {}
 
+  xml-name-validator@5.0.0: {}
+
+  xmlchars@2.2.0: {}
+
   yallist@3.1.1: {}
 
   yocto-queue@0.1.0: {}
diff --git a/frontend/src/features/ontology/api/__tests__/hooks.test.ts b/frontend/src/features/ontology/api/__tests__/hooks.test.ts
new file mode 100644
index 0000000..3db8372
--- /dev/null
+++ b/frontend/src/features/ontology/api/__tests__/hooks.test.ts
@@ -0,0 +1,190 @@
+import { describe, it, expect, vi, beforeEach } from "vitest";
+import { renderHook, waitFor } from "@testing-library/react";
+import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
+import React from "react";
+import { useAllConcepts, useFrameworkStats } from "../index";
+
+// Mock the api module
+vi.mock("@/lib/api", () => ({
+  api: {
+    get: vi.fn(),
+  },
+}));
+
+import { api } from "@/lib/api";
+
+const mockedApi = vi.mocked(api);
+
+function createWrapper() {
+  const queryClient = new QueryClient({
+    defaultOptions: {
+      queries: { retry: false },
+    },
+  });
+  return function Wrapper({ children }: { children: React.ReactNode }) {
+    return React.createElement(
+      QueryClientProvider,
+      { client: queryClient },
+      children
+    );
+  };
+}
+
+const FW_A = { id: "fw-a", name: "FW A", version: null, description: null, source_url: null, created_at: "", updated_at: "" };
+const FW_B = { id: "fw-b", name: "FW B", version: null, description: null, source_url: null, created_at: "", updated_at: "" };
+
+function makeConcept(id: string, frameworkId: string, type = "control") {
+  return {
+    id,
+    framework_id: frameworkId,
+    parent_id: null,
+    concept_type: type,
+    code: null,
+    name_en: id,
+    name_nb: null,
+    definition_en: null,
+    definition_nb: null,
+    source_reference: null,
+    sort_order: null,
+    created_at: "",
+    updated_at: "",
+  };
+}
+
+describe("useAllConcepts", () => {
+  beforeEach(() => {
+    vi.resetAllMocks();
+  });
+
+  it("returns combined concepts from multiple frameworks", async () => {
+    mockedApi.get.mockImplementation(async (url: string) => {
+      if (url === "/ontology/frameworks") {
+        return { data: [FW_A, FW_B] };
+      }
+      if (url.includes("framework_id=fw-a")) {
+        return { data: { data: [makeConcept("c1", "fw-a")], total: 1, page: 1, limit: 500, total_pages: 1 } };
+      }
+      if (url.includes("framework_id=fw-b")) {
+        return { data: { data: [makeConcept("c2", "fw-b")], total: 1, page: 1, limit: 500, total_pages: 1 } };
+      }
+      return { data: [] };
+    });
+
+    const { result } = renderHook(() => useAllConcepts(), { wrapper: createWrapper() });
+
+    await waitFor(() => {
+      expect(result.current.isLoading).toBe(false);
+    });
+
+    expect(result.current.data).toHaveLength(2);
+    expect(result.current.data.map((c) => c.id).sort()).toEqual(["c1", "c2"]);
+  });
+
+  it("returns errors array when some queries fail", async () => {
+    mockedApi.get.mockImplementation(async (url: string) => {
+      if (url === "/ontology/frameworks") {
+        return { data: [FW_A, FW_B] };
+      }
+      if (url.includes("framework_id=fw-a")) {
+        return { data: { data: [makeConcept("c1", "fw-a")], total: 1, page: 1, limit: 500, total_pages: 1 } };
+      }
+      if (url.includes("framework_id=fw-b")) {
+        throw new Error("Network error");
+      }
+      return { data: [] };
+    });
+
+    const { result } = renderHook(() => useAllConcepts(), { wrapper: createWrapper() });
+
+    await waitFor(() => {
+      expect(result.current.errors.length).toBeGreaterThan(0);
+    });
+
+    // Successful framework's concepts should still be available
+    expect(result.current.data.some((c) => c.id === "c1")).toBe(true);
+  });
+
+  it("builds correct concept-to-framework Map", async () => {
+    mockedApi.get.mockImplementation(async (url: string) => {
+      if (url === "/ontology/frameworks") {
+        return { data: [FW_A, FW_B] };
+      }
+      if (url.includes("framework_id=fw-a")) {
+        return { data: { data: [makeConcept("c1", "fw-a")], total: 1, page: 1, limit: 500, total_pages: 1 } };
+      }
+      if (url.includes("framework_id=fw-b")) {
+        return { data: { data: [makeConcept("c2", "fw-b")], total: 1, page: 1, limit: 500, total_pages: 1 } };
+      }
+      return { data: [] };
+    });
+
+    const { result } = renderHook(() => useAllConcepts(), { wrapper: createWrapper() });
+
+    await waitFor(() => {
+      expect(result.current.isLoading).toBe(false);
+    });
+
+    expect(result.current.conceptToFramework.get("c1")).toBe("fw-a");
+    expect(result.current.conceptToFramework.get("c2")).toBe("fw-b");
+  });
+});
+
+describe("useFrameworkStats", () => {
+  beforeEach(() => {
+    vi.resetAllMocks();
+  });
+
+  it("returns correct stats per framework", async () => {
+    mockedApi.get.mockImplementation(async (url: string) => {
+      if (url === "/ontology/frameworks") {
+        return { data: [FW_A, FW_B] };
+      }
+      if (url === "/ontology/relationships") {
+        return {
+          data: [
+            { id: "r1", source_concept_id: "c1", target_concept_id: "c3", relationship_type: "maps_to", description: null, created_at: null },
+          ],
+        };
+      }
+      if (url.includes("framework_id=fw-a")) {
+        return {
+          data: {
+            data: [
+              makeConcept("c1", "fw-a", "control"),
+              makeConcept("c2", "fw-a", "principle"),
+            ],
+            total: 2, page: 1, limit: 500, total_pages: 1,
+          },
+        };
+      }
+      if (url.includes("framework_id=fw-b")) {
+        return {
+          data: {
+            data: [makeConcept("c3", "fw-b", "control")],
+            total: 1, page: 1, limit: 500, total_pages: 1,
+          },
+        };
+      }
+      return { data: [] };
+    });
+
+    const { result } = renderHook(() => useFrameworkStats(), { wrapper: createWrapper() });
+
+    await waitFor(() => {
+      expect(result.current.isLoading).toBe(false);
+    });
+
+    const statsA = result.current.data.get("fw-a");
+    expect(statsA).toBeDefined();
+    expect(statsA!.conceptCount).toBe(2);
+    expect(statsA!.conceptTypes).toEqual({ control: 1, principle: 1 });
+    expect(statsA!.connectedFrameworks).toBe(1); // connected to fw-b via r1
+    expect(statsA!.relationshipCount).toBe(1);
+
+    const statsB = result.current.data.get("fw-b");
+    expect(statsB).toBeDefined();
+    expect(statsB!.conceptCount).toBe(1);
+    expect(statsB!.connectedFrameworks).toBe(1); // connected to fw-a via r1
+    expect(statsB!.relationshipCount).toBe(1);
+  });
+});
diff --git a/frontend/src/features/ontology/api/index.ts b/frontend/src/features/ontology/api/index.ts
index 258409b..7394468 100644
--- a/frontend/src/features/ontology/api/index.ts
+++ b/frontend/src/features/ontology/api/index.ts
@@ -1,4 +1,5 @@
-import { useQuery } from "@tanstack/react-query";
+import { useQuery, useQueries } from "@tanstack/react-query";
+import { useMemo } from "react";
 import { api } from "@/lib/api";
 import type {
   Framework,
@@ -7,6 +8,7 @@ import type {
   ConceptWithRelationships,
   PaginatedResponse,
   Topic,
+  FrameworkStats,
 } from "../types";
 
 // Query keys
@@ -136,3 +138,101 @@ export function useSearchConcepts(query: string, frameworkId?: string) {
     enabled: query.length >= 2,
   });
 }
+
+// Fetch all concepts across all frameworks
+export function useAllConcepts(): {
+  data: Concept[];
+  conceptToFramework: Map<string, string>;
+  isLoading: boolean;
+  errors: Error[];
+} {
+  const { data: frameworks, isLoading: fwsLoading } = useFrameworks();
+
+  const queries = useQueries({
+    queries: (frameworks ?? []).map((fw) => ({
+      queryKey: ontologyKeys.concepts(fw.id),
+      queryFn: async () => {
+        const params = new URLSearchParams();
+        params.set("framework_id", fw.id);
+        params.set("limit", "500");
+        const { data } = await api.get<PaginatedResponse<Concept>>(
+          `/ontology/concepts?${params}`
+        );
+        return data.data;
+      },
+      staleTime: 1000 * 60 * 5,
+    })),
+  });
+
+  const data = useMemo(
+    () => queries.flatMap((q) => q.data ?? []),
+    [queries]
+  );
+
+  const conceptToFramework = useMemo(() => {
+    const map = new Map<string, string>();
+    for (const concept of data) {
+      map.set(concept.id, concept.framework_id);
+    }
+    return map;
+  }, [data]);
+
+  const errors = queries
+    .filter((q) => q.error != null)
+    .map((q) => q.error as Error);
+
+  const isLoading = fwsLoading || queries.some((q) => q.isPending);
+
+  return { data, conceptToFramework, isLoading, errors };
+}
+
+// Compute per-framework statistics from concepts and relationships
+export function useFrameworkStats(): {
+  data: Map<string, FrameworkStats>;
+  isLoading: boolean;
+} {
+  const { data: frameworks, isLoading: fwLoading } = useFrameworks();
+  const { data: allConcepts, conceptToFramework, isLoading: conceptsLoading } = useAllConcepts();
+  const { data: relationships, isLoading: relsLoading } = useRelationships();
+
+  const isLoading = fwLoading || conceptsLoading || relsLoading;
+
+  const data = useMemo(() => {
+    const stats = new Map<string, FrameworkStats>();
+    if (!frameworks) return stats;
+
+    for (const fw of frameworks) {
+      const fwConcepts = allConcepts.filter((c) => c.framework_id === fw.id);
+      const conceptTypes: Record<string, number> = {};
+      for (const c of fwConcepts) {
+        conceptTypes[c.concept_type] = (conceptTypes[c.concept_type] || 0) + 1;
+      }
+
+      const fwConceptIds = new Set(fwConcepts.map((c) => c.id));
+      const fwRelationships = (relationships ?? []).filter(
+        (r) =>
+          fwConceptIds.has(r.source_concept_id) ||
+          fwConceptIds.has(r.target_concept_id)
+      );
+
+      const connectedFws = new Set<string>();
+      for (const rel of fwRelationships) {
+        const sourceFw = conceptToFramework.get(rel.source_concept_id);
+        const targetFw = conceptToFramework.get(rel.target_concept_id);
+        if (sourceFw && sourceFw !== fw.id) connectedFws.add(sourceFw);
+        if (targetFw && targetFw !== fw.id) connectedFws.add(targetFw);
+      }
+
+      stats.set(fw.id, {
+        conceptCount: fwConcepts.length,
+        conceptTypes,
+        connectedFrameworks: connectedFws.size,
+        relationshipCount: fwRelationships.length,
+      });
+    }
+
+    return stats;
+  }, [frameworks, allConcepts, relationships, conceptToFramework]);
+
+  return { data, isLoading };
+}
diff --git a/frontend/src/features/ontology/types/index.ts b/frontend/src/features/ontology/types/index.ts
index 5a5fba0..2723833 100644
--- a/frontend/src/features/ontology/types/index.ts
+++ b/frontend/src/features/ontology/types/index.ts
@@ -140,3 +140,24 @@ export interface SavedLayout {
   expandedNodes: string[];
   timestamp: number;
 }
+
+// Framework Explorer Page Types
+export interface FrameworkStats {
+  conceptCount: number;
+  conceptTypes: Record<string, number>;
+  connectedFrameworks: number;
+  relationshipCount: number;
+}
+
+export interface CrosswalkCell {
+  sourceFrameworkId: string;
+  targetFrameworkId: string;
+  count: number;
+  relationships: Relationship[];
+}
+
+export interface LandscapeProfile {
+  sector: string;
+  activities: string[];
+  applicableFrameworks: string[];
+}
diff --git a/frontend/src/features/ontology/utils/__tests__/frameworkDomains.test.ts b/frontend/src/features/ontology/utils/__tests__/frameworkDomains.test.ts
new file mode 100644
index 0000000..871b739
--- /dev/null
+++ b/frontend/src/features/ontology/utils/__tests__/frameworkDomains.test.ts
@@ -0,0 +1,74 @@
+import { describe, it, expect } from "vitest";
+import { groupFrameworksByDomain } from "../frameworkDomains";
+import type { Framework } from "../../types";
+
+function makeFramework(id: string): Framework {
+  return {
+    id,
+    name: id,
+    version: null,
+    description: null,
+    source_url: null,
+    created_at: "",
+    updated_at: "",
+  };
+}
+
+const ALL_IDS = [
+  "iso31000", "iso31010", "iso27000", "iso9000", "nist-csf", "nist-800-53", "nist-rmf",
+  "eu-ai-act", "nist-ai-rmf", "iso42001", "iso42005", "iso23894", "google-saif", "mitre-atlas",
+  "gdpr", "nis2", "dora", "cer-directive",
+  "zero-trust", "cisa-ztmm", "data-centric", "fmn",
+];
+
+describe("groupFrameworksByDomain", () => {
+  it("returns 4 groups with correct labels", () => {
+    const groups = groupFrameworksByDomain(ALL_IDS.map(makeFramework));
+    expect(groups).toHaveLength(4);
+    expect(groups.map((g) => g.label)).toEqual([
+      "Risk & Security Standards",
+      "AI Governance",
+      "EU Regulations",
+      "Architecture & Models",
+    ]);
+  });
+
+  it("each group contains expected framework IDs", () => {
+    const groups = groupFrameworksByDomain(ALL_IDS.map(makeFramework));
+    const byLabel = Object.fromEntries(groups.map((g) => [g.label, g.frameworkIds]));
+
+    expect(byLabel["Risk & Security Standards"]).toEqual(
+      expect.arrayContaining(["iso31000", "iso31010", "iso27000", "iso9000", "nist-csf", "nist-800-53", "nist-rmf"])
+    );
+    expect(byLabel["AI Governance"]).toEqual(
+      expect.arrayContaining(["eu-ai-act", "nist-ai-rmf", "iso42001", "iso42005", "iso23894", "google-saif", "mitre-atlas"])
+    );
+    expect(byLabel["EU Regulations"]).toEqual(
+      expect.arrayContaining(["gdpr", "nis2", "dora", "cer-directive"])
+    );
+    expect(byLabel["Architecture & Models"]).toEqual(
+      expect.arrayContaining(["zero-trust", "cisa-ztmm", "data-centric", "fmn"])
+    );
+  });
+
+  it("all 22 frameworks assigned to exactly one group", () => {
+    const groups = groupFrameworksByDomain(ALL_IDS.map(makeFramework));
+    const allIds = groups.flatMap((g) => g.frameworkIds);
+    expect(allIds).toHaveLength(22);
+    expect(new Set(allIds).size).toBe(22);
+  });
+
+  it("handles empty framework array", () => {
+    const groups = groupFrameworksByDomain([]);
+    expect(groups).toHaveLength(4);
+    groups.forEach((g) => {
+      expect(g.frameworkIds).toEqual([]);
+    });
+  });
+
+  it("unknown framework IDs excluded", () => {
+    const groups = groupFrameworksByDomain([makeFramework("unknown-fw")]);
+    const allIds = groups.flatMap((g) => g.frameworkIds);
+    expect(allIds).not.toContain("unknown-fw");
+  });
+});
diff --git a/frontend/src/features/ontology/utils/__tests__/landscapeMapping.test.ts b/frontend/src/features/ontology/utils/__tests__/landscapeMapping.test.ts
new file mode 100644
index 0000000..66faf37
--- /dev/null
+++ b/frontend/src/features/ontology/utils/__tests__/landscapeMapping.test.ts
@@ -0,0 +1,37 @@
+import { describe, it, expect } from "vitest";
+import { getApplicableFrameworks } from "../landscapeMapping";
+
+const UNIVERSALS = ["iso31000", "iso31010", "iso9000"];
+
+describe("getApplicableFrameworks", () => {
+  it("Financial sector returns base frameworks", () => {
+    const result = getApplicableFrameworks("Financial", []);
+    expect(result).toEqual(expect.arrayContaining([...UNIVERSALS, "dora", "nis2", "iso27000", "gdpr"]));
+  });
+
+  it("Deploying AI systems activity adds correct frameworks", () => {
+    const result = getApplicableFrameworks("", ["Deploying AI systems"]);
+    expect(result).toEqual(
+      expect.arrayContaining(["eu-ai-act", "nist-ai-rmf", "iso42001", "iso23894"])
+    );
+  });
+
+  it("combined sector + activities produces no duplicates", () => {
+    const result = getApplicableFrameworks("Financial", ["Financial services"]);
+    const uniqueCount = new Set(result).size;
+    expect(result).toHaveLength(uniqueCount);
+    expect(result).toContain("dora");
+  });
+
+  it("universal frameworks always included", () => {
+    const result = getApplicableFrameworks("Healthcare", ["Processing personal data"]);
+    UNIVERSALS.forEach((fw) => {
+      expect(result).toContain(fw);
+    });
+  });
+
+  it("empty sector + no activities returns only universals", () => {
+    const result = getApplicableFrameworks("", []);
+    expect(result.sort()).toEqual([...UNIVERSALS].sort());
+  });
+});
diff --git a/frontend/src/features/ontology/utils/__tests__/urlParams.test.ts b/frontend/src/features/ontology/utils/__tests__/urlParams.test.ts
new file mode 100644
index 0000000..7edd381
--- /dev/null
+++ b/frontend/src/features/ontology/utils/__tests__/urlParams.test.ts
@@ -0,0 +1,20 @@
+import { describe, it, expect } from "vitest";
+import { parseCommaSeparated } from "../urlParams";
+
+describe("parseCommaSeparated", () => {
+  it("splits comma-separated values", () => {
+    expect(parseCommaSeparated("a,b,c")).toEqual(["a", "b", "c"]);
+  });
+
+  it("empty string returns empty array", () => {
+    expect(parseCommaSeparated("")).toEqual([]);
+  });
+
+  it("filters out empty strings from consecutive commas", () => {
+    expect(parseCommaSeparated("a,,b")).toEqual(["a", "b"]);
+  });
+
+  it("undefined returns empty array", () => {
+    expect(parseCommaSeparated(undefined)).toEqual([]);
+  });
+});
diff --git a/frontend/src/features/ontology/utils/frameworkDomains.ts b/frontend/src/features/ontology/utils/frameworkDomains.ts
new file mode 100644
index 0000000..c567eba
--- /dev/null
+++ b/frontend/src/features/ontology/utils/frameworkDomains.ts
@@ -0,0 +1,30 @@
+import type { Framework } from "../types";
+
+const DOMAIN_MAP: { label: string; ids: string[] }[] = [
+  {
+    label: "Risk & Security Standards",
+    ids: ["iso31000", "iso31010", "iso27000", "iso9000", "nist-csf", "nist-800-53", "nist-rmf"],
+  },
+  {
+    label: "AI Governance",
+    ids: ["eu-ai-act", "nist-ai-rmf", "iso42001", "iso42005", "iso23894", "google-saif", "mitre-atlas"],
+  },
+  {
+    label: "EU Regulations",
+    ids: ["gdpr", "nis2", "dora", "cer-directive"],
+  },
+  {
+    label: "Architecture & Models",
+    ids: ["zero-trust", "cisa-ztmm", "data-centric", "fmn"],
+  },
+];
+
+export function groupFrameworksByDomain(
+  frameworks: Framework[]
+): { label: string; frameworkIds: string[] }[] {
+  const available = new Set(frameworks.map((f) => f.id));
+  return DOMAIN_MAP.map((domain) => ({
+    label: domain.label,
+    frameworkIds: domain.ids.filter((id) => available.has(id)),
+  }));
+}
diff --git a/frontend/src/features/ontology/utils/index.ts b/frontend/src/features/ontology/utils/index.ts
index 0d2e8fc..64716e9 100644
--- a/frontend/src/features/ontology/utils/index.ts
+++ b/frontend/src/features/ontology/utils/index.ts
@@ -1,2 +1,5 @@
 export * from "./treeBuilder";
 export * from "./graphTransform";
+export * from "./frameworkDomains";
+export * from "./landscapeMapping";
+export * from "./urlParams";
diff --git a/frontend/src/features/ontology/utils/landscapeMapping.ts b/frontend/src/features/ontology/utils/landscapeMapping.ts
new file mode 100644
index 0000000..337e4c8
--- /dev/null
+++ b/frontend/src/features/ontology/utils/landscapeMapping.ts
@@ -0,0 +1,39 @@
+const UNIVERSAL_FRAMEWORKS = ["iso31000", "iso31010", "iso9000"];
+
+const SECTOR_FRAMEWORKS: Record<string, string[]> = {
+  Financial: ["dora", "nis2", "iso27000", "gdpr"],
+  Healthcare: ["nis2", "gdpr", "iso27000"],
+  "Critical Infrastructure": ["nis2", "cer-directive", "iso27000", "nist-csf"],
+  "Government/Public Admin": ["nis2", "gdpr", "iso27000"],
+  "Technology/AI Provider": ["gdpr", "iso27000"],
+  "General Enterprise": ["iso27000", "gdpr"],
+};
+
+const ACTIVITY_FRAMEWORKS: Record<string, string[]> = {
+  "Processing personal data": ["gdpr"],
+  "Deploying AI systems": ["eu-ai-act", "nist-ai-rmf", "iso42001", "iso23894"],
+  "Operating critical infrastructure": ["cer-directive", "nist-csf"],
+  "Financial services": ["dora"],
+  "Defense/NATO context": ["fmn", "zero-trust", "cisa-ztmm"],
+};
+
+export function getApplicableFrameworks(
+  sector: string,
+  activities: string[]
+): string[] {
+  const result = new Set(UNIVERSAL_FRAMEWORKS);
+
+  const sectorFws = SECTOR_FRAMEWORKS[sector];
+  if (sectorFws) {
+    sectorFws.forEach((fw) => result.add(fw));
+  }
+
+  for (const activity of activities) {
+    const activityFws = ACTIVITY_FRAMEWORKS[activity];
+    if (activityFws) {
+      activityFws.forEach((fw) => result.add(fw));
+    }
+  }
+
+  return [...result];
+}
diff --git a/frontend/src/features/ontology/utils/urlParams.ts b/frontend/src/features/ontology/utils/urlParams.ts
new file mode 100644
index 0000000..212bdb1
--- /dev/null
+++ b/frontend/src/features/ontology/utils/urlParams.ts
@@ -0,0 +1,4 @@
+export function parseCommaSeparated(value: string | undefined): string[] {
+  if (!value) return [];
+  return value.split(",").filter(Boolean);
+}
diff --git a/frontend/src/test-setup.ts b/frontend/src/test-setup.ts
new file mode 100644
index 0000000..f149f27
--- /dev/null
+++ b/frontend/src/test-setup.ts
@@ -0,0 +1 @@
+import "@testing-library/jest-dom/vitest";
diff --git a/frontend/vitest.config.ts b/frontend/vitest.config.ts
new file mode 100644
index 0000000..52d0272
--- /dev/null
+++ b/frontend/vitest.config.ts
@@ -0,0 +1,15 @@
+import { defineConfig } from "vitest/config";
+import path from "path";
+
+export default defineConfig({
+  test: {
+    environment: "jsdom",
+    globals: true,
+    setupFiles: ["./src/test-setup.ts"],
+  },
+  resolve: {
+    alias: {
+      "@": path.resolve(__dirname, "./src"),
+    },
+  },
+});
