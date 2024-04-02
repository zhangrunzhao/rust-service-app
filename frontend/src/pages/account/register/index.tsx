import { Button, Form, Input } from '@arco-design/web-react';
import { useMemoizedFn, useRequest } from 'ahooks';
import { httpPost } from '@/utils';
import { useNavigate } from 'react-router-dom';

const FormItem = Form.Item;

interface RegisterProps {}

interface AccountFormFields {
  username: String;
  pwd: String;
}

// 该组件主要用于注册相关事项
export const Register: React.FC<RegisterProps> = () => {
  const [form] = Form.useForm<AccountFormFields>();
  const navigate = useNavigate();

  const gotoLoginPage = useMemoizedFn(() => {
    navigate('/account/login');
  });

  const { run: handleRegister } = useRequest(
    async () => {
      await form.validate();
      const username = form.getFieldValue('username');
      const pwd = form.getFieldValue('pwd');

      // await httpGet('/happy?id=12345');
      // TODO: 开发一下注册的接口
      await httpPost('/api/login', {
        username,
        pwd,
      });
    },
    {
      manual: true,
    }
  );

  return (
    <div className="flex justify-center h-[100vh]">
      <div className="border-2 rounded h-[55vh] mt-[15vh] p-8 border-gray-600">
        <div className="text-center pb-4 text-lg">注册页面</div>

        <Form layout="vertical" className="w-[450px]" form={form} colon>
          <FormItem<AccountFormFields>
            field="username"
            rules={[{ required: true }]}
            label="用户名"
          >
            <Input placeholder="请输入用户名" />
          </FormItem>

          <FormItem<AccountFormFields>
            field="pwd"
            label="密码"
            rules={[{ required: true }]}
          >
            <Input placeholder="请输入密码" />
          </FormItem>
        </Form>

        <div>
          <Button
            className="w-full mt-8"
            type="primary"
            size="large"
            onClick={handleRegister}
          >
            注册
          </Button>
          <Button
            className="w-full mt-4"
            type="secondary"
            size="large"
            onClick={gotoLoginPage}
          >
            已有账号？前往登录
          </Button>
        </div>
      </div>
    </div>
  );
};
