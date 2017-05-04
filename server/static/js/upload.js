/** upload.js
 *
 */

document.addEventListener('DOMContentLoaded', function() {
    var upload = document.getElementById('upload');
    var file = document.getElementById('file');
    var encryption_pass = document.getElementById('encryption_pass');
    var access_pass = document.getElementById('access_pass');

    let c = window.crypto;
    let iv = c.getRandomValues(new Uint8Array(16));
    let alg = {name: 'AES-CBC', iv};


    function uploadBytes(data_bytes, confirm_bytes, filename_bytes, access_pass) {
        var request = new XMLHttpRequest();
        request.open("POST", '/upload', true);
        request.onreadystatechange = function() {
            if (request.readyState !== XMLHttpRequest.DONE || request.status !== 200) { return; }
            var resp = request.responseText;
            var respData = JSON.parse(resp);
            alert("Got: " + resp);
        }
        var data = {
            bytes: data_bytes,
            confirm: confirm_bytes,
            iv: Array.from(iv),
            access_pass: access_pass,
            filename: filename_bytes,
        };
        request.send(JSON.stringify(data));
    }

    function encryptAndUpload(file, enc_pass, access_pass) {
        var reader = new FileReader();
        reader.onload = function(e) {
            var data = e.target.result;
            c.subtle.digest('SHA-256', enc_pass).then((hash) => {
                c.subtle.importKey('raw', hash, alg, false, ['encrypt']).then((key) => {
                    c.subtle.encrypt(alg, key, data).then((encrypted_data) => {
                        c.subtle.encrypt(alg, key, hash).then((encrypted_confirm) => {
                            let filename = new TextEncoder().encode(file.name);
                            c.subtle.encrypt(alg, key, filename).then((encrypted_filename) => {
                                let data_bytes = new Uint8Array(encrypted_data);
                                data_bytes = Array.from(data_bytes);

                                let confirm_bytes = new Uint8Array(encrypted_confirm);
                                confirm_bytes = Array.from(confirm_bytes);

                                let filename_bytes = new Uint8Array(encrypted_filename);
                                filename_bytes = Array.from(filename_bytes);

                                uploadBytes(data_bytes, confirm_bytes, filename_bytes, access_pass);
                            });
                        });
                    });
                        //.then((bytes) => c.subtle.decrypt(alg, key, bytes))
                        //.then((dec) => {
                        //    let s = new TextDecoder().decode(dec);
                        //    console.log(s);
                        //})
                });
            });
        }
        reader.readAsArrayBuffer(file);
    }

    upload.addEventListener('click', function(e) {
        if (!encryption_pass.value) {
            alert('encryption password required');
            return;
        }
        var encryption_pass_bytes = new TextEncoder().encode(encryption_pass.value);
        if (!access_pass.value) {
            alert('access passowrd required!');
            return;
        }
        var access_pass_value = access_pass.value;
        if (file.files.length !== 1) {
            alert('file required');
            return;
        }
        var encrypted = encryptAndUpload(file.files[0], encryption_pass_bytes, access_pass_value);
    });
});
