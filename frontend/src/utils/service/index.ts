import axios, { AxiosRequestConfig, AxiosResponse } from 'axios';
import { DefaultResponseData } from './type';

// https://jsnoteclub.com/axios/best-practices/react-axios/
const axiosInstance = axios.create({
  // TODO：这个字段最后需要换成域名
  baseURL: 'http://localhost:5173/',
});

const httpGet = async <T = Object>(url: string, config?: AxiosRequestConfig) => {
  const response = await axiosInstance.get<any, AxiosResponse<DefaultResponseData<T>, any>>(url, config);

  return response?.data || { code: 9998, data: { message: "未知错误" } };
};


const httpPost = async <Q = any, R = Object>(url: string, body?: Q, config?: AxiosRequestConfig) => {
  const response = await axiosInstance.post<any, AxiosResponse<DefaultResponseData<R>, any>>(url, body, config);

  return response?.data || { code: 9998, data: { message: "未知错误" } };
}


axiosInstance.interceptors.response.use((response) => {

  if (response.status !== 200) {
    console.log("============ 查看错误响应信息 ============")
    console.log(response)
    console.log("=======================================")
  }

  return response
})

export { httpGet, httpPost };
