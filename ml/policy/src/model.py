import tensorflow as tf 

with tf.name_scope('policy') as scope:
    # tf Graph Input
    features = 4
    objects = 8
    layers = 3

    x = tf.placeholder(tf.float32, [None, 8, 8, features], name='input')
    y = tf.placeholder(tf.float32, [None, 8, 8, 1], name='expected')

    # Set model weights
    #first pass filters
    Fa = tf.Variable(tf.random_normal([1,8,features,objects]))
    Fb = tf.Variable(tf.random_normal([8,1,features,objects]))
    b = tf.Variable(tf.random_normal([objects,8,8]))
    
    # Construct model
    La = tf.transpose(tf.nn.conv2d(x, Fa, [1,1,1,1], 'VALID'),[0,3,1,2])
    Lb = tf.transpose(tf.nn.conv2d(x, Fb, [1,1,1,1], 'VALID'),[0,3,1,2])
    L  = tf.transpose(tf.tanh(tf.matmul(La, Lb) + b), [0,2,3,1])
    for i in range(layers):

        Fa = tf.Variable(tf.random_normal([1,8,objects,objects]))
        Fb = tf.Variable(tf.random_normal([8,1,objects,objects]))
        b = tf.Variable(tf.random_normal([objects,8,8]))

        La = tf.transpose(tf.nn.conv2d(L, Fa, [1,1,1,1], 'VALID'),[0,3,1,2])
        Lb = tf.transpose(tf.nn.conv2d(L, Fb, [1,1,1,1], 'VALID'),[0,3,1,2])
        L  = tf.transpose(tf.tanh(tf.matmul(La, Lb) + b), [0,2,3,1])

    #Consolidation filters
    F = tf.Variable(tf.random_normal([1,1,objects,1]))
    b = tf.Variable(tf.random_normal([8,8,1]))

    # the output of the model
    pred = tf.nn.sigmoid(tf.nn.conv2d(L, F, [1,1,1,1], 'VALID') + b, name='output')

    cost = tf.pow(tf.reduce_mean(tf.nn.softmax_cross_entropy_with_logits(
        labels = tf.reshape(y, [-1, 64]), 
        logits = tf.log(tf.reshape(pred, [-1, 64])), 
    )), 2, name='cost')

    learning_rate = tf.placeholder(tf.float32, [], name='learning_rate')
    optimizer = tf.train.GradientDescentOptimizer(learning_rate).minimize(cost, name='train')

tf.variables_initializer(tf.global_variables(), name = 'init')
definition = tf.Session().graph_def
directory = '../../data/policy'



saver = tf.train.Saver(tf.global_variables(), name='saver')
saver_def = saver.as_saver_def()

# The name of the tensor you must feed with a filename when saving/restoring.
print(saver_def.filename_tensor_name)

# The name of the target operation you must run when restoring.
print(saver_def.restore_op_name)

# The name of the target operation you must run when saving.
print(saver_def.save_tensor_name)


tf.train.write_graph(definition, directory, 'policy-{}x{}.pb'.format(objects, layers), as_text=False)

exit()

# Initializing the variables
init = tf.global_variables_initializer()

import random
training_epochs = 20
display_step = 1
lr = 0.01

with tf.Session() as sess:
    sess.run(init)
    batch_size = 100
    # Training cycle
    for epoch in range(training_epochs):
        avg_cost = 0.
        total_batch = 1000
        # Loop over all batches
        batch_xs = []
        batch_ys = []
        for _ in range(batch_size):
            itmi = [[[0 for ___ in range(4)] for __ in range(8)] for _ in range(8)]
            itmj = [[[0] for __ in range(8)] for _ in range(8)]
            batch_xs.append(itmi)
            batch_ys.append(itmj)
        for i in range(total_batch):
            ix = i * batch_size

            for k in range(batch_size):
                for Y in range(8):
                    for X in range(8):
                        itmj[Y][X][0] = random.choice([0.0,1.0])

                        for j in range(4):
                            itmi[Y][X][j] = random.choice([0.0, 1.0])
                batch_xs.append(itmi)
                batch_ys.append(itmj)

            # Run optimization op (backprop) and cost op (to get loss value)
            _, c = sess.run([optimizer, cost], feed_dict={x: batch_xs,
                                                            y: batch_ys,
                                                            learning_rate: lr})
            # Compute average loss
            del batch_xs[:]
            del batch_ys[:]
            avg_cost += c
            #print("cost=",c," avg=",avg_cost/(i+1))
            if (i % 100 == 0):
                print(100.0 * i/float(total_batch), '%')
        # Display logs per epoch step
        if (epoch+1) % display_step == 0:
            print("Epoch:", '%04d' % (epoch+1), "cost=", "{:.9f}".format(avg_cost/total_batch))
        saver.save(sess, 'policy_net', global_step=epoch)
        lr = lr * 0.97

    print("Optimization Finished!")