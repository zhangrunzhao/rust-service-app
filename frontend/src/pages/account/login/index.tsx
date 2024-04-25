import { Button, Form, Input, Message } from '@arco-design/web-react';
import {
  useCountDown,
  useMemoizedFn,
  useRequest,
  useUpdateEffect,
} from 'ahooks';
import { useAtom } from 'jotai';
import { httpPost } from '@/utils';
import { useNavigate } from 'react-router-dom';
import { useState } from 'react';
import { userAtom } from '@/store';

const FormItem = Form.Item;

interface LoginProps {}

interface AccountFormFields {
  username: String;
  pwd: String;
}

// 该组件主要用于登录相关事项
export const Login: React.FC<LoginProps> = () => {
  const [form] = Form.useForm<AccountFormFields>();
  const navigate = useNavigate();

  const [, setUserInfo] = useAtom(userAtom);

  const [leftTime, setLeftTime] = useState(0);
  const [countdown] = useCountDown({
    leftTime,
  });

  useUpdateEffect(() => {
    const currentCount = Math.round(countdown / 1000);
    Message.info({
      id: 'countdown_message',
      content: `登陆成功，${currentCount} 秒后跳转到首页`,
      className: currentCount ? '' : 'hidden',
    });

    if (!currentCount) {
      gotoHomePage();
    }
  }, [countdown]);

  const gotoRegisterPage = useMemoizedFn(() => {
    navigate('/account/register');
  });

  const gotoHomePage = useMemoizedFn(() => {
    navigate('/');
  });

  const { run: login } = useRequest(
    async () => {
      await form.validate();
      const username = form.getFieldValue('username');
      const pwd = form.getFieldValue('pwd');

      const result = await httpPost('/api/login', {
        username,
        pwd,
      });

      const { code, data } = result;

      if (!code) {
        // 开始倒计时
        setLeftTime(3000);
      }

      setUserInfo(data);
    },
    {
      manual: true,
    }
  );

  return (
    <div className="flex justify-center h-[100vh]">
      <div className="border-2 rounded h-[55vh] mt-[15vh] p-8 border-gray-600">
        <div className="text-center pb-4 text-lg">登录页面</div>

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
            onClick={login}
          >
            登录
          </Button>
          <Button
            className="w-full mt-4"
            type="secondary"
            size="large"
            onClick={gotoRegisterPage}
          >
            注册
          </Button>
        </div>
      </div>
    </div>
  );
};
