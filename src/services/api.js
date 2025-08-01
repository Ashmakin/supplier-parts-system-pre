import axios from 'axios';

// 创建一个Axios客户端实例，配置后端API的基础URL
const apiClient = axios.create({
    baseURL: 'http://127.0.0.1:8080/api', // 确保这与你的后端地址匹配
});

// 设置请求拦截器
// 在每个请求发送前，检查localStorage中是否有token，如果有，就将其添加到Authorization头中
apiClient.interceptors.request.use(
    (config) => {
        const token = localStorage.getItem('token');
        if (token) {
            config.headers.Authorization = `Bearer ${token}`;
        }
        return config;
    },
    (error) => {
        return Promise.reject(error);
    }
);

// 定义所有API请求函数
export const register = (data) => apiClient.post('/auth/register', data);
export const login = (credentials) => apiClient.post('/auth/login', credentials);

// 未来可以添加更多API调用，例如：
// export const getRfqs = () => apiClient.get('/rfqs');