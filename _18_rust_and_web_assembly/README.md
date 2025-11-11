## Using Rust WebAssembly in an Android App
- You can integrate Rust WebAssembly (WASM) into an Android mobile application by embedding it in a WebView or calling it from a native Rust-compiled library.
---
- Approaches for Android with Rust WebAssembly:
    - Using WebView to Load WASM in a Web App (Easier)
    - Using JNI to Call Rust Code from Android (Native Approach)
    - Using WASM Runtime in Android (wasmtime or wasmer)
---
### Using WebView (Recommended for Simplicity)
- This method embeds a WebAssembly module inside an Android WebView.
    - Steps to Implement:
        - Step 1: Setup Rust WebAssembly
            - Create a new Rust library project:
                ```shell
                cargo new --lib wasm_android_demo
                cd wasm_android_demo
                Modify Cargo.toml:
                ```

                - Modify Cargo.toml:
                ```toml
                [lib]
                crate-type = ["cdylib"]

                [dependencies]
                wasm-bindgen = "0.2"
                ```

                - Modify src/lib.rs:

                ```rust
                use wasm_bindgen::prelude::*;

                // Expose function to JavaScript
                #[wasm_bindgen]
                pub fn add(a: i32, b: i32) -> i32 {
                    a + b
                }
                ```

                - Build for WebAssembly:

                ```sh
                wasm-pack build --target web
                ```

                -Build WebAssembly into target folder:
                ```sh
                wasm-pack build --release --target web --out-dir <folder/path/goes/here>
                ```

                - This generates the pkg/ directory with wasm_android_demo_bg.wasm and JavaScript bindings.

        - Step 2: Create an HTML + JS File
            - Inside your Android project's assets/ folder, create index.html:

                ```html
                <!DOCTYPE html>
                <html lang="en">
                <head>
                    <meta charset="UTF-8">
                    <meta name="viewport" content="width=device-width, initial-scale=1.0">
                    <title>Rust WASM Android</title>
                </head>
                <body>
                    <h1>Rust WebAssembly on Android</h1>
                    <button onclick="runWasm()">Add Numbers</button>
                    <p id="result"></p>

                    <script type="module">
                        import init, { add } from './pkg/wasm_android_demo.js';

                        async function runWasm() {
                            await init();
                            document.getElementById('result').innerText = "3 + 5 = " + add(3, 5);
                        }
                    </script>
                </body>
                </html>
                ```

        - Step 3: Add WebView in Android App
            - Modify AndroidManifest.xml to allow WebView:

                ```xml
                <uses-permission android:name="android.permission.INTERNET"/>
                ```
            
            - Modify MainActivity.java (for Java-based Android):
                ```java
                package com.example.wasmapp;

                import android.os.Bundle;
                import android.webkit.WebView;
                import androidx.appcompat.app.AppCompatActivity;

                public class MainActivity extends AppCompatActivity {
                    @Override
                    protected void onCreate(Bundle savedInstanceState) {
                        super.onCreate(savedInstanceState);

                        WebView webView = new WebView(this);
                        webView.getSettings().setJavaScriptEnabled(true);
                        webView.loadUrl("file:///android_asset/index.html");

                        setContentView(webView);
                    }
                }
                ```
            - If using Kotlin:
                ```kotlin
                package com.example.wasmapp

                import android.os.Bundle
                import android.webkit.WebView
                import androidx.appcompat.app.AppCompatActivity

                class MainActivity : AppCompatActivity() {
                    override fun onCreate(savedInstanceState: Bundle?) {
                        super.onCreate(savedInstanceState)

                        val webView = WebView(this)
                        webView.settings.javaScriptEnabled = true
                        webView.loadUrl("file:///android_asset/index.html")

                        setContentView(webView)
                    }
                }
                ```
        
        - Step 4: Run the Android App
            - Now, run the Android app, and it will execute Rust WebAssembly inside the WebView.

    - Pros of This Approach:
        - Easy integration
        - Uses Web technologies inside Android
        - No need for JNI or Native bindings

---

## Using Rust in an Android App with JNI (Native Approach)
- If you want native performance, you can compile Rust into a shared library (.so) and call it from Android using JNI (Java Native Interface).
---
1. Install Required Tools
    - Ensure you have:

        - Rust with the nightly toolchain
        - Android NDK (Native Development Kit)
        - Cargo NDK (to build Rust for Android)
        - Android Studio with an NDK-compatible project
    - Install NDK & Rust Toolchain
        ```sh
        rustup target add aarch64-linux-android
        cargo install cargo-ndk
        ```
2. Create a Rust Library
    - Step 1: Create a Rust Library
        - Create a Rust project:
            ```sh
            cargo new --lib android_rust_jni
            cd android_rust_jni
            ```
        - Modify Cargo.toml:

            ```toml
            [lib]
            crate-type = ["cdylib"]

            [dependencies]
            jni = "0.21"  # JNI for Rust <-> Java/Kotlin
            ```

3. Write Rust Code
    - modify src/lib.rs:
        ```rust
        use jni::objects::{JClass, JString};
        use jni::sys::jstring;
        use jni::JNIEnv;

        // Function accessible from Java/Kotlin
        #[no_mangle]
        pub extern "system" fn Java_com_example_rustjni_MainActivity_helloFromRust(env: JNIEnv, _class: JClass) -> jstring {
            let output = env.new_string("Hello from Rust!").expect("Failed to create string");
            output.into_inner()
        }
        ```

4. Compile for Android
    - Step 1: Set Up the Android Target
        - Ensure Android NDK paths are set:
            ```sh
            export ANDROID_NDK_HOME=$HOME/Android/Sdk/ndk/25.1.8937393
            ```

    - Step 2: Build Rust Shared Library
        - Run:
            ```sh
            cargo ndk -t arm64-v8a build --release
            ```

        - This generates:
            ```bash
            target/aarch64-linux-android/release/libandroid_rust_jni.so
            ```

5. Integrate Rust into Android Studio
    - Step 1: Copy .so File to Android
        - Copy libandroid_rust_jni.so into:
            ```bash
            app/src/main/jniLibs/arm64-v8a/
            ```
    - Step 2: Create JNI Loader in Android
        - Modify MainActivity.java:

            ```java
            package com.example.rustjni;

            import androidx.appcompat.app.AppCompatActivity;
            import android.os.Bundle;
            import android.widget.TextView;

            public class MainActivity extends AppCompatActivity {
                static {
                    System.loadLibrary("android_rust_jni"); // Load Rust library
                }

                public native String helloFromRust();

                @Override
                protected void onCreate(Bundle savedInstanceState) {
                    super.onCreate(savedInstanceState);
                    TextView textView = new TextView(this);
                    textView.setText(helloFromRust());
                    setContentView(textView);
                }
            }
            ```
        - For Kotlin:
            ```kotlin
            package com.example.rustjni

            import androidx.appcompat.app.AppCompatActivity
            import android.os.Bundle
            import android.widget.TextView

            class MainActivity : AppCompatActivity() {
                companion object {
                    init {
                        System.loadLibrary("android_rust_jni") // Load Rust library
                    }
                }

                external fun helloFromRust(): String

                override fun onCreate(savedInstanceState: Bundle?) {
                    super.onCreate(savedInstanceState)
                    val textView = TextView(this)
                    textView.text = helloFromRust()
                    setContentView(textView)
                }
            }
            ```

6. Run the Android App
    - Open Android Studio.
    - Build and run the app on an Android device/emulator.
    - The screen should display: 
        ```
        "Hello from Rust!" 🎉
        ```