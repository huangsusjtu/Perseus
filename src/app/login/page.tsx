import { Button, Checkbox, Label, TextInput } from "flowbite-react";
import type { FC } from "react";

const BackgroundImageLoginForm: FC = function () {
  return (
    <section className="bg-gray-700 bg-opacity-80 bg-[url('/loginbackground.jpg')] bg-cover bg-center bg-no-repeat bg-blend-multiply">
      <div className="pt:mt-0 mx-auto flex flex-col items-center justify-center px-6 py-8 md:h-screen">
        {/* <a
          href="#"
          className="mb-6 flex items-center text-4xl font-semibold text-white"
        >
          <img
            className="h-16 w-16"
            src="/favicon.png"
            alt="logo"
          />
          System
        </a> */}
        <div className="w-full rounded-lg bg-white shadow dark:bg-gray-800 sm:max-w-md md:mt-0 xl:p-0">
          <div className="space-y-4 p-6 sm:p-8 md:space-y-6 lg:space-y-8">
            <h1 className="text-center text-xl font-bold leading-tight tracking-tight text-gray-900 dark:text-white md:text-2xl">
              Sign in to your account
            </h1>
            <form className="space-y-4 md:space-y-6" action="#">
              <div>
                <Label htmlFor="phone" className="mb-2 block dark:text-white">Your phone</Label>
                <TextInput
                  id="phone"
                  placeholder="+8615121356789"
                  required
                  type="tel"
                />
              </div>
              <div>
                <Label htmlFor="password" className="mb-2 block dark:text-white">Password</Label>
                <TextInput
                  id="confirm-password"
                  placeholder="••••••••"
                  required
                  type="password"
                />
              </div>
              <div className="flex items-center justify-between">
                <div className="flex items-start">
                  <div className="flex h-5 items-center">
                    <Checkbox id="remember-background" />
                  </div>
                  <div className="ml-3 text-sm">
                    <Label htmlFor="remember-background" className="text-gray-500 dark:text-gray-300">Remember me</Label>
                  </div>
                </div>
                {/* <a
                  href="#"
                  className="text-sm font-medium text-primary-600 hover:underline dark:text-primary-500"
                >
                  Forgot password?
                </a> */}
              </div>
              <Button type="submit" className="w-full">
                Log in to your account
              </Button>
              {/* <p className="text-center text-sm text-gray-500 dark:text-gray-300">
                <Button color="none" href="#" className="w-full text-primary-600 dark:text-primary-500 p-0 [&>span]:p-0 hover:underline">
                  Don't have an account?
                </Button>
              </p> */}
            </form>
          </div>
        </div>
      </div>
    </section>
  );
};

export default BackgroundImageLoginForm;
