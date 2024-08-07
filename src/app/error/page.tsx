import type { FC } from "react";

const IllustrationLinks404Page: FC = function () {
    return (
        <section className="bg-white dark:bg-gray-900 w-screen h-screen">
            <div className="mx-auto max-w-screen-xl px-4 py-8 lg:px-6 lg:py-16">
                <div className="mx-auto max-w-screen-sm text-center">
                    <img
                        alt=""
                        src="/404-computer.svg"
                        className="mx-auto mb-4"
                    />
                    <h1 className="mb-4 text-2xl font-extrabold text-primary-600 dark:text-primary-500">
                        404 Not Found
                    </h1>
                    <p className="mb-10 text-3xl font-bold tracking-tight text-gray-900 dark:text-white md:text-4xl">
                        Whoops! That page doesn’t exist.
                    </p>
                </div>
            </div>
        </section>
    );
};

export default IllustrationLinks404Page;
