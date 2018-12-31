# -*- coding: utf-8 -*-

import sys
import time
import requests#测试使用第三方库。测试时记得pip install requests

coolq_sdk = None

def main():
    sys.stdout = coolq_sdk.Logger()#把stdout重定向到酷q日志。可以不使用。
    coolq_sdk.register("on_private_message", on_private_message)#注册事件
    coolq_sdk.register("exit", exit)

    return {
        "version": "0.0.1",
        "author": "soeur",
        "name": "test"
    }#返回版本，作者，应用名字等信息。非强制。希望提供。

def on_private_message(sub_type, send_time, user_id, msg, font):
    if msg == "test":
        baidu_url = 'https://www.baidu.com'
        response = requests.get(baidu_url)
        coolq_sdk.send_private_msg(user_id, response.content.decode())
    else:
        coolq_sdk.send_private_msg(user_id, "现在时间: " + str(time.asctime(time.localtime(time.time()))))


def exit():
    print("test exit")