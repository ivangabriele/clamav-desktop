import { toast, ToastBar, Toaster as RhtToaster } from 'react-hot-toast'
import { MdOutlineClose } from 'react-icons/md'
import styled from 'styled-components'

const Box = styled.div`
  .Toast {
    color: white;
    border-radius: 4px;
    font-weight: 400;
    padding: 0;

    [role='status'] {
      margin: 0;
      padding: 0.5rem 0;
    }
  }
`

const IconBox = styled.div`
  padding: 0 0 0 1rem;
`

const MessageBox = styled.div`
  padding: 0.25rem 1rem;
`

const Button = styled.button`
  align-items: center;
  align-self: flex-start;
  background-color: transparent;
  border: 0;
  color: lightgray;
  cursor: pointer;
  display: flex;
  height: 50%;
  padding: 1.25rem 0.5rem;

  :hover {
    color: white;
  }

  > svg {
    fill: white;
    height: 1.5rem;
    width: 1.5rem;
  }
`

/** @see https://react-hot-toast.com/docs/toaster */
const TOAST_OPTIONS = {
  className: 'Toast',
  duration: 5000,
  error: {
    duration: Infinity,
    style: {
      background: 'red',
    },
  },
  style: {
    background: 'blue',
  },
  success: {
    style: {
      background: 'green',
    },
  },
}

export function Toaster() {
  return (
    <Box>
      <RhtToaster position="top-right" toastOptions={TOAST_OPTIONS}>
        {thisToast => (
          <ToastBar toast={thisToast}>
            {({ icon, message }) => (
              <>
                {thisToast.type !== 'blank' && <IconBox>{icon}</IconBox>}
                <MessageBox>{message}</MessageBox>
                {thisToast.type === 'error' && (
                  <Button onClick={() => toast.dismiss(thisToast.id)} type="button">
                    <MdOutlineClose />
                  </Button>
                )}
              </>
            )}
          </ToastBar>
        )}
      </RhtToaster>
    </Box>
  )
}
