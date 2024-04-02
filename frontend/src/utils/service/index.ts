import axios from 'axios';

// https://jsnoteclub.com/axios/best-practices/react-axios/
const axiosInstance = axios.create({
  baseURL: 'http://localhost:8080/',
});

const httpGet = axiosInstance.get;

const httpPost = axiosInstance.post;

export { httpGet, httpPost };
