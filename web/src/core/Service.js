import axios from "axios";

let beforeRequest, afterRequest;

class Service {
    upload(url, data) {
        return new Promise((resolve, reject) => {
            beforeRequest && beforeRequest();
            axios.post(url, data, {
                headers: {
                    "Content-Type": "multipart/form-data"
                }
            }).then((rsp) => {
                afterRequest && afterRequest();
                if(rsp.data.rspCode !== "0"){
                    reject(rsp.data);
                }else{
                    resolve(rsp.data);
                }
            }, reject);
        });
    }
    download(url, data) {
        return new Promise((resolve, reject) => {
            axios.get(url, {
                params: data,
                responseType: "blob"
            }).then((rsp) => {
                if (rsp.data.rspCode && rsp.data.rspCode !== "0") {
                    reject(rsp.data);
                } else {
                    const blob = new Blob([rsp.data]);
                    let fileName = data.filename;
                    if ('download' in document.createElement('a')) { // 非IE下载
                        const elink = document.createElement('a');
                        elink.download = fileName;
                        elink.style.display = 'none';
                        elink.href = URL.createObjectURL(blob);
                        document.body.appendChild(elink);
                        elink.click();
                        URL.revokeObjectURL(elink.href);// 释放URL 对象
                        document.body.removeChild(elink)
                    } else { // IE10+下载
                        navigator.msSaveBlob(blob, fileName)
                    }
                }
            }, reject);
        });
    }
    request(url, data){
        return new Promise((resolve, reject) => {
            beforeRequest && beforeRequest();
            axios.post(url, data || {}).then((rsp) => {
                afterRequest && afterRequest();
                if (rsp.data.rspCode !== "0") {
                    reject(rsp.data);
                } else {
                    resolve(rsp.data);
                }
            }, reject);
        });
    }
    setBeforeRequest(func){
        beforeRequest = func;
    }
    setAfterRequest(func){
        afterRequest = func;
    }
}
const service = new Service();
export default service;