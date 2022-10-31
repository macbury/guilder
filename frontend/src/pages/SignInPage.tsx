import React, { useState, useCallback } from 'react';
import Modal from 'rsuite/Modal';
import FlexboxGrid from 'rsuite/FlexboxGrid';
import Panel from 'rsuite/Panel';
import Form from 'rsuite/Form';
import ButtonToolbar from 'rsuite/ButtonToolbar';
import Button from 'rsuite/Button';
import useWindowTitle from '../components/hooks/useWindowTitle';
import { useAuthenticationManager } from '../store/hooks/session';

export default function SignInPage() {
  useWindowTitle('Sign in');

  const { signIn, status } = useAuthenticationManager();
  const [formValue, setFormValue] = useState<any>({ login: "", password: "" });

  const onFormSubmit = useCallback(async (e) => {
    e.preventDefault()

    signIn(formValue.login, formValue.password);
  }, [signIn, formValue]);

  return (
    <Modal open={true}>
      <Modal.Body>
        <Form
          onSubmit={onFormSubmit}
          layout="horizontal"
          readOnly={status == 'loading'}
          fluid
          formValue={formValue}
          onChange={formValue => setFormValue(formValue)}>
          <Form.Group>
            <Form.ControlLabel>Login</Form.ControlLabel>
            <Form.Control
              onPressEnter={onFormSubmit}
              defaultValue=""
              name="login" />
          </Form.Group>
          <Form.Group>
            <Form.ControlLabel>Password</Form.ControlLabel>
            <Form.Control
              onPressEnter={onFormSubmit}
              defaultValue=""
              name="password"
              type="password"
              autoComplete="off" />
          </Form.Group>
        </Form>
      </Modal.Body>
      <Modal.Footer>
        <Button onClick={onFormSubmit} loading={status == 'loading'} appearance="primary">
          Sign in
        </Button>
      </Modal.Footer>
    </Modal>
  );
}
