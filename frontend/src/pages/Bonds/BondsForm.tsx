import React from 'react';
import styled from 'styled-components';
import Toggle from 'rsuite/Toggle';
import Drawer from 'rsuite/Drawer';
import Button from 'rsuite/Button';
import Form from 'rsuite/Form';
import { useBondsState } from '../../store/hooks/bonds';
import CategorySelect from '../../components/inputs/CategorySelect';
import AccountSelect from '../../components/inputs/AccountSelect';
import WalletSelect from '../../components/inputs/WalletSelect';

const Label = styled(Form.ControlLabel)`
  margin-bottom: 10px;
  .rs-form-control {
    float: right;
    width: auto;
  }
`

export default function BondsForm() {
  const {
    state: {
      form: {
        visible,
        errors,
        value
      },
      loading,
      selectedCount
    },
    actions: {
      hideForm,
      updateForm,
      massUpdate
    }
  } = useBondsState();

  return (
    <Drawer open={visible} onClose={() => hideForm()} backdrop="static" enforceFocus placement="left">
      <Drawer.Header>
        <Drawer.Title>Batch update</Drawer.Title>
        <Drawer.Actions>
          <Button onClick={() => hideForm()}>Cancel</Button>
          <Button onClick={massUpdate} appearance="primary" loading={loading}>
            Update {selectedCount} bonds
          </Button>
        </Drawer.Actions>
      </Drawer.Header>
      <Drawer.Body>
        <Form
          onSubmit={massUpdate}
          disabled={loading}
          fluid
          formValue={value}
          onChange={updateForm}
        >
          <Form.Group controlId="categoryId">
            <Label>
              <Form.Control
                size="md"
                checkedChildren="Update category"
                unCheckedChildren="Leave category"
                accepter={Toggle}
                name="updateCategory" />
              Category:
            </Label>
            <CategorySelect
              errors={errors}
              name="categoryId"
              disabled={!value.updateCategory}
            />
          </Form.Group>

          <Form.Group controlId="accountId">
            <Label>
              <Form.Control
                size="md"
                checkedChildren="Update account"
                unCheckedChildren="Leave account"
                accepter={Toggle}
                name="updateAccount" />
              Account:
            </Label>
            <AccountSelect
              errors={errors}
              name="accountId"
              disabled={!value.updateAccount}
            />
          </Form.Group>

          <Form.Group controlId="walletId">
            <Label>
              <Form.Control
                size="md"
                checkedChildren="Update wallet"
                unCheckedChildren="Leave wallet"
                accepter={Toggle}
                name="updateWallet" />
              Wallet:
            </Label>
            <WalletSelect
              errors={errors}
              name="walletId"
              disabled={!value.updateWallet}
            />
          </Form.Group>
        </Form>
      </Drawer.Body>
    </Drawer>
  )
}
