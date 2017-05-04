
document.addEventListener('DOMContentLoaded', function() {
    var download = document.getElementById('download');
    var access_key = document.getElementById('access_key');
    var encryption_pass = document.getElementById('encryption_pass');
    var access_pass = document.getElementById('access_pass');

    function downloadBytes(access_key, access_pass) {
        var request = new XMLHttpRequest();
        request.open('GET', '/download/' + access_key, true);
        request.onreadystatechange = function() {
            if (request.readyState !== XMLHttpRequest.DONE || request.status !== 200) { return; }
            var resp = request.responseText;
            var respData = JSON.parse(resp);
            console.log("Got: ",  respData);
        }
        var data = {
            access_pass: access_pass,
        };
        request.send(JSON.stringify(data));
    }

    download.addEventListener('click', function() {
        if (!encryption_pass.value) {
            alert('encryption_pass required');
            return;
        }
        if (!access_pass.value) {
            alert('access_pass required');
            return;
        }
        if (!access_key.value) {
            alert('access_key required');
            return;
        }
        downloadBytes(access_key.value, access_pass.value);
    });
});
